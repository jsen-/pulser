mod delimited;
mod seq;
mod wrap;

pub use delimited::*;
pub use seq::*;
use std::io;
pub use wrap::*;

// mod list;
// use list::*;

pub trait IntoOwned<T> {
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

struct Comma;
impl Separator for Comma {
    const SEPARATOR: u8 = b',';
}

pub fn main() {
    // let vec: Vec<Box<Iterator<Item = u8>>> = vec![Box::new(y1), Box::new(y2), Box::new(y3)];
    // let mut list_items = vec.into_iter();
    // let list = List {
    //     current: list_items.next(),
    //     it: list_items,
    // };

    // ;

    let y1 = delim(b"key", Comma, b"value");
    let y2 = wrap(CurlyBraces, b"*");
    let list = seq2(y1, y2);

    {
        let mut reader = ReadAdapter(list);
        let out = std::io::stdout();
        let mut lock = out.lock();
        std::io::copy(&mut reader, &mut lock).unwrap();
    }
    println!();
    println!();
    println!();

}
