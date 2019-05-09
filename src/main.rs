#![allow(dead_code, unused_variables)]
mod delimited;
mod seq;
mod wrap;

pub use delimited::*;
pub use seq::*;
use std::io;
pub use wrap::*;

// mod list;
// use list::*;

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

pub trait Wrapper {
    const START: u8;
    const END: u8;
}
pub trait Separator {
    const SEPARATOR: u8;
}

struct CurlyBraces;
impl Wrapper for CurlyBraces {
    const START: u8 = b'{';
    const END: u8 = b'}';
}

struct Quotes;
impl Wrapper for Quotes {
    const START: u8 = b'"';
    const END: u8 = b'"';
}

struct Comma;
impl Separator for Comma {
    const SEPARATOR: u8 = b',';
}

struct Colon;
impl Separator for Colon {
    const SEPARATOR: u8 = b':';
}

struct IteratorAdapter<T>
where
    T: IntoIterator<Item = u8>,
{
    it: T::IntoIter,
}

impl<T> Iterator for IteratorAdapter<T>
where
    T: IntoIterator<Item = u8>,
{
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        self.it.next()
    }
}

struct StringIterator(String, usize);
impl StringIterator {
    fn new(str: String) -> Self {
        Self(str, 0)
    }
}
impl Iterator for StringIterator {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        if self.1 < self.0.len() {
            let x = Some(self.0.as_bytes()[self.1]);
            self.1 += 1;
            x
        } else {
            None
        }
    }
}

struct DbRow {
    id: isize,
    name: String,
    age: u8,
}

trait NumberOfDigits {
    fn digits(&self) -> u8;
}

impl NumberOfDigits for usize {
    fn digits(&self) -> u8 {
        match *self {
            0 => 1,
            std::usize::MAX => (std::usize::MAX as f64).log10().ceil() as _,
            _ => ((self + 1) as f64).log10().ceil() as _,
        }
    }
}
impl NumberOfDigits for isize {
    fn digits(&self) -> u8 {
        match self.abs() {
            0 => 1,
            std::isize::MAX => (std::isize::MAX as f64).log10().ceil() as _,
            _ => ((self + 1) as f64).log10().ceil() as _,
        }
    }
}

#[derive(Debug)]
struct IsizeIterator {
    digits: u8,
    number: isize,
}
impl IsizeIterator {
    pub fn new(mut number: isize) -> Self {
        let digits = if number < 0 {
            number = number.abs();
            number.digits() | 128
        } else {
            number.digits()
        };
        Self { number, digits }
    }
}
impl Iterator for IsizeIterator {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        if self.digits == 0 {
            None
        } else {
            if self.digits & 128 != 0 {
                self.digits = self.digits ^ 128;
                Some(b'-')
            } else {
                let exp = 10_usize.pow(self.digits as u32 - 1) as isize;
                self.digits -= 1;

                let ret = self.number / exp;
                self.number = self.number % exp;
                Some(ret as u8 + b'0')
            }
        }
    }
}

impl IntoIterator for DbRow {
    type Item = u8;
    type IntoIter = Box<Iterator<Item = u8>>;

    fn into_iter(self) -> Self::IntoIter {
        let id = StringIterator::new(self.id.to_string());
        let name = StringIterator::new(self.name);
        let age = StringIterator::new(self.age.to_string());

        Box::new(JsonObject(Nil.push("age", age)).into_iter())
    }
}

// pub fn main() {
//     let row1 = DbRow {
//         id: 1,
//         name: "Jozo".to_string(),
//         age: 70,
//     }
//     .into_iter();
//     let row2 = DbRow {
//         id: 2,
//         name: "Milan".to_string(),
//         age: 52,
//     }
//     .into_iter();
//     let row3 = DbRow {
//         id: 3,
//         name: "Cecilka".to_string(),
//         age: 92,
//     }
//     .into_iter();
//     let mut db_results = vec![row1, row2, row3].into_iter();
//     let current: Option<Box<dyn Iterator<Item = u8>>> = db_results.next().map(|first| {
//         let j: Box<dyn Iterator<Item = u8>> = Box::new(first);
//         j
//     });

//     let list = List {
//         current,
//         it: db_results,
//     };

//     // let y1 = delim(b"key", Comma, b"value");
//     // let y2 = wrap(CurlyBraces, b"*");
//     // let list = seq2(y1, y2);

//     {
//         let mut reader = ReadAdapter(list);
//         let out = std::io::stdout();
//         let mut lock = out.lock();
//         std::io::copy(&mut reader, &mut lock).unwrap();
//     }
//     println!();
// }

struct Nil;
struct Properties<V: IntoIterator<Item = u8>, T> {
    name: &'static str,
    value: V,
    tail: T,
}

trait Prop: Sized {
    fn push<V: IntoIterator<Item = u8>>(self, name: &'static str, value: V) -> Properties<V, Self> {
        Properties {
            name,
            value,
            tail: self,
        }
    }
}

impl Prop for Nil {}
impl<V: IntoIterator<Item = u8>, T> Prop for Properties<V, T> {}

use std::str::Bytes;

impl<V, T, U> IntoIterator for Properties<V, Properties<T, U>>
where
    V: IntoIterator<Item = u8>,
    Properties<T, U>: IntoIterator<Item = u8>,
    T: IntoIterator<Item = u8>,
{
    type IntoIter = Delimited<
        Delimited<Wrapped<Quotes, Bytes<'static>>, Colon, V::IntoIter>,
        Comma,
        <Properties<T, U> as IntoIterator>::IntoIter,
    >;
    type Item = u8;

    fn into_iter(self) -> Self::IntoIter {
        delim(
            delim(
                wrap(Quotes, self.name.bytes()),
                Colon,
                self.value.into_iter(),
            ),
            Comma,
            self.tail.into_iter(),
        )
    }
}

impl<V> IntoIterator for Properties<V, Nil>
where
    V: IntoIterator<Item = u8>,
{
    type IntoIter = Delimited<Wrapped<Quotes, Bytes<'static>>, Colon, V::IntoIter>;
    type Item = u8;

    fn into_iter(self) -> Self::IntoIter {
        delim(
            wrap(Quotes, self.name.bytes()),
            Colon,
            self.value.into_iter(),
        )
    }
}

struct JsonObject<P>(P);
impl<P> IntoIterator for JsonObject<P>
where
    P: IntoIterator<Item = u8>,
{
    type Item = u8;
    type IntoIter = Wrapped<CurlyBraces, P::IntoIter>;

    fn into_iter(self) -> Self::IntoIter {
        wrap(CurlyBraces, self.0.into_iter())
    }
}

impl<P: Prop> JsonObject<P> {
    fn prop<V: IntoIterator<Item = u8>>(
        self,
        name: &'static str,
        value: V,
    ) -> JsonObject<Properties<V, P>> {
        Self(self.0.push(name, value))
    }
}

struct JsonString<P>(P);
impl<P> IntoIterator for JsonString<P>
where
    P: IntoIterator<Item = u8>,
{
    type Item = u8;
    type IntoIter = Wrapped<Quotes, P::IntoIter>;

    fn into_iter(self) -> Self::IntoIter {
        wrap(Quotes, self.0)
    }
}

fn main() {
    let num = -14987;

    let it = IsizeIterator::new(num);
    let mut r = ReadAdapter(it);

    let stdout = std::io::stdout();
    let mut l = stdout.lock();
    std::io::copy(&mut r, &mut l).unwrap();
    println!();

    let x = JsonObject(Nil)
        .prop("key2", JsonString("x2".bytes()))
        .prop("key1", JsonString("x1".bytes()));

    let y = JsonObject(Nil)
        .prop("a", JsonString("1".bytes()))
        .prop("b", x);

    y.into_iter().for_each(|x| {
        print!("{}", std::str::from_utf8(&[x]).unwrap());
    })
}
