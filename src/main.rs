#![allow(dead_code)]

use std::io;
use std::marker::PhantomData;

struct ReadAdapter<T: Iterator<Item = u8>>(T);

impl<T> io::Read for ReadAdapter<T>
where
    T: Iterator<Item = u8>,
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        Ok(self
            .0
            .by_ref()
            .take(buf.len())
            .enumerate()
            .map(|(offset, val)| buf[offset] = val)
            .count())
    }
}

enum WrapState {
    Begin,
    Middle,
    End,
}
trait Wrapped {
    const START: u8;
    const END: u8;
}

struct Wrapper<T, W>
where
    W: Wrapped,
    T: Iterator,
    T::Item: Into<u8>,
{
    it: T,
    state: WrapState,
    p: PhantomData<W>,
}

impl<T, W> Iterator for Wrapper<T, W>
where
    W: Wrapped,
    T: Iterator,
    T::Item: Into<u8>,
{
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        use WrapState::*;
        match &self.state {
            Begin => {
                self.state = Middle;
                Some(W::START)
            }
            Middle => self.it.next().map(Into::into).or_else(|| {
                self.state = End;
                Some(W::END)
            }),
            End => None,
        }
    }
}

struct CurlyBraces;
impl Wrapped for CurlyBraces {
    const START: u8 = b'{';
    const END: u8 = b'}';
}

fn wrap<W, T>(_w: W, it: T) -> Wrapper<T, W>
where
    W: Wrapped,
    T: Iterator,
    T::Item: Into<u8>,
{
    Wrapper {
        it,
        state: WrapState::Begin,
        p: PhantomData,
    }
}

//

//

struct KeyValue<K, V> {
    key: K,
    value: V,
    state: WrapState,
}

trait IntoOwned<T> {
    fn into_owned(&self) -> T;
}

impl<T: Copy> IntoOwned<T> for &T {
    fn into_owned(&self) -> T {
        **self
    }
}
impl<T: Copy> IntoOwned<T> for T {
    fn into_owned(&self) -> T {
        *self
    }
}

impl<K, V> Iterator for KeyValue<K, V>
where
    K: Iterator,
    K::Item: IntoOwned<u8>,
    V: Iterator<Item = u8>,
    V::Item: IntoOwned<u8>,
{
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        use WrapState::*;
        match self.state {
            Begin => self.key.next().map(|k| k.into_owned()).or_else(|| {
                self.state = Middle;
                Some(b':')
            }),
            Middle => self.value.next().map(|v| v.into_owned()).or_else(|| {
                self.state = End;
                None
            }),
            End => None,
        }
    }
}

impl<K, V> KeyValue<K, V>
where
    K: Iterator,
    K::Item: IntoOwned<u8>,
    V: Iterator<Item = u8>,
    V::Item: IntoOwned<u8>,
{
    fn new<IK, IV>(key: IK, value: IV) -> Self
    where
        IK: IntoIterator<IntoIter = K, Item = K::Item>,
        IV: IntoIterator<IntoIter = V, Item = V::Item>,
    {
        Self {
            key: key.into_iter(),
            value: value.into_iter(),
            state: WrapState::Begin,
        }
    }
}

// struct List<T> {
//     list: T
// }

// impl Iterator

fn sequence2<I1, I2>(first: I1, second: I2) -> impl Iterator<Item = u8>
where
    I1: IntoIterator,
    I1::IntoIter: Iterator<Item = I1::Item>,
    I1::Item: IntoOwned<u8>,
    I2: IntoIterator,
    I2::Item: IntoOwned<u8>,
    I2::IntoIter: Iterator<Item = I2::Item>,
{
    first
        .into_iter()
        .map(|x| x.into_owned())
        .chain(second.into_iter().map(|x| x.into_owned()))
}
fn sequence3<I1, I2, I3>(first: I1, second: I2, third: I3) -> impl Iterator<Item = u8>
where
    I1: IntoIterator,
    I1::IntoIter: Iterator<Item = I1::Item>,
    I1::Item: IntoOwned<u8>,
    I2: IntoIterator,
    I2::Item: IntoOwned<u8>,
    I2::IntoIter: Iterator<Item = I2::Item>,
    I3: IntoIterator,
    I3::Item: IntoOwned<u8>,
    I3::IntoIter: Iterator<Item = I3::Item>,
{
    sequence2(first, sequence2(second, third))
}

fn sequence4<I1, I2, I3, I4>(
    first: I1,
    second: I2,
    third: I3,
    fourth: I4,
) -> impl Iterator<Item = u8>
where
    I1: IntoIterator,
    I1::IntoIter: Iterator<Item = I1::Item>,
    I1::Item: IntoOwned<u8>,
    I2: IntoIterator,
    I2::Item: IntoOwned<u8>,
    I2::IntoIter: Iterator<Item = I2::Item>,
    I3: IntoIterator,
    I3::Item: IntoOwned<u8>,
    I3::IntoIter: Iterator<Item = I3::Item>,
    I4: IntoIterator,
    I4::Item: IntoOwned<u8>,
    I4::IntoIter: Iterator<Item = I4::Item>,
{
    sequence2(sequence2(first, second), sequence2(third, fourth))
}

struct List<T, I>
where
    T: Iterator<Item = I>,
    I: Iterator<Item = u8>,
{
    it: T,
    current: Option<*mut I>, // self referential struct
    p: PhantomData<I>,
}

impl<T, I> Iterator for List<T, I>
where
    T: Iterator<Item = I>,
    I: Iterator<Item = u8>,
{
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        self.current.and_then(|current| {
            let current = unsafe { &mut *current };
            match current.next() {
                Some(x) => Some(x),
                None => {
                    let mut i: Option<I> = self.it.next();
                    self.current = i.as_mut().map(|next| next as *mut _);
                    self.next()
                }
            }
        })
    }
}

fn main() {
    // wrap(CurlyBraces, y)
    let y1 = sequence2(b"hello", b"+");
    let y2 = sequence2(b"world", b"-");

    let mut list_items = vec![y2, y1].into_iter();

    let mut list = List {
        current: list_items.next().as_mut().map(|first| first as *mut _),
        it: list_items,
        p: PhantomData,
    };

    println!(
        "{:?}",
        std::str::from_utf8(&[list.next().unwrap()]).unwrap()
    );
    println!(
        "{:?}",
        std::str::from_utf8(&[list.next().unwrap()]).unwrap()
    );
    println!(
        "{:?}",
        std::str::from_utf8(&[list.next().unwrap()]).unwrap()
    );
    println!(
        "{:?}",
        std::str::from_utf8(&[list.next().unwrap()]).unwrap()
    );
    println!(
        "{:?}",
        std::str::from_utf8(&[list.next().unwrap()]).unwrap()
    );
    println!(
        "{:?}",
        std::str::from_utf8(&[list.next().unwrap()]).unwrap()
    );
    println!(
        "{:?}",
        std::str::from_utf8(&[list.next().unwrap()]).unwrap()
    );
    println!(
        "{:?}",
        std::str::from_utf8(&[list.next().unwrap()]).unwrap()
    );
    println!(
        "{:?}",
        std::str::from_utf8(&[list.next().unwrap()]).unwrap()
    );
    println!(
        "{:?}",
        std::str::from_utf8(&[list.next().unwrap()]).unwrap()
    );
    println!(
        "{:?}",
        std::str::from_utf8(&[list.next().unwrap()]).unwrap()
    );

    // let mut reader = ReadAdapter(list);

    // let out = std::io::stdout();
    // let mut lock = out.lock();

    // std::io::copy(&mut reader, &mut lock).unwrap();
}
