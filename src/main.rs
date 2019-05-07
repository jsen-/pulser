#![allow(dead_code)]
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

struct DbRow {
    id: isize,
    name: String,
    age: u8,
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

trait MakeIt {
    type It: Iterator<Item = u8>;
    fn make_it(self) -> Self::It;
}

struct StringIterator(String, usize);
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

impl MakeIt for String {
    type It = StringIterator;
    fn make_it(self) -> Self::It {
        StringIterator(self, 0)
    }
}
impl MakeIt for isize {
    type It = StringIterator;
    fn make_it(self) -> Self::It {
        StringIterator(self.to_string(), 0)
    }
}
impl MakeIt for u8 {
    type It = StringIterator;
    fn make_it(self) -> Self::It {
        StringIterator(self.to_string(), 0)
    }
}

impl IntoIterator for DbRow {
    type IntoIter = Wrapped<
        CurlyBraces,
        Delimited<
            Delimited<
                Delimited<
                    Wrapped<Quotes, std::iter::Cloned<std::slice::Iter<'static, u8>>>,
                    Colon,
                    StringIterator,
                >,
                Comma,
                Delimited<
                    Wrapped<Quotes, std::iter::Cloned<std::slice::Iter<'static, u8>>>,
                    Colon,
                    StringIterator,
                >,
            >,
            Comma,
            Delimited<
                Wrapped<Quotes, std::iter::Cloned<std::slice::Iter<'static, u8>>>,
                Colon,
                StringIterator,
            >,
        >,
    >;
    type Item = u8;
    fn into_iter(self) -> Self::IntoIter {
        let id_prop = wrap(Quotes, b"id".into_iter().cloned());
        let id_value = self.id.make_it();
        let id = delim(id_prop, Colon, id_value);

        let name_prop = wrap(Quotes, b"name".into_iter().cloned());
        let name_value = self.name.make_it();
        let name = delim(name_prop, Colon, name_value);

        let age_prop = wrap(Quotes, b"age".into_iter().cloned());
        let age_value = self.age.make_it();
        let age = delim(age_prop, Colon, age_value);

        wrap(CurlyBraces, delim(delim(id, Comma, name), Comma, age))
    }
}

struct List<T: Sized>
where
    T: Iterator,
    T::Item: Iterator<Item = u8>,
{
    current: Option<Box<dyn Iterator<Item = u8>>>,
    it: T,
}
impl<T> Iterator for List<T>
where
    T: Iterator,
    T::Item: Iterator<Item = u8>,
    T::Item: 'static,
{
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current.take();
        current.and_then(|mut current| match current.next() {
            Some(x) => {
                self.current = Some(current);
                Some(x)
            }
            None => {
                self.current = self.it.next().map(|next| {
                    let j: Box<dyn Iterator<Item = u8>> = Box::new(next);
                    j
                });
                self.next()
            }
        })
    }
}

pub fn main() {
    let row1 = DbRow {
        id: 1,
        name: "Jozo".to_string(),
        age: 70,
    }
    .into_iter();
    let row2 = DbRow {
        id: 2,
        name: "Milan".to_string(),
        age: 52,
    }
    .into_iter();
    let row3 = DbRow {
        id: 3,
        name: "Cecilka".to_string(),
        age: 92,
    }
    .into_iter();
    let mut db_results = vec![row1, row2, row3].into_iter();
    let current: Option<Box<dyn Iterator<Item = u8>>> = db_results.next().map(|first| {
        let j: Box<dyn Iterator<Item = u8>> = Box::new(first);
        j
    });

    let list = List {
        current,
        it: db_results,
    };

    // let y1 = delim(b"key", Comma, b"value");
    // let y2 = wrap(CurlyBraces, b"*");
    // let list = seq2(y1, y2);

    {
        let mut reader = ReadAdapter(list);
        let out = std::io::stdout();
        let mut lock = out.lock();
        std::io::copy(&mut reader, &mut lock).unwrap();
    }
    println!();
}
