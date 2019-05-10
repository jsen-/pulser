mod array;
mod iterator_iterator;
mod delimited;
mod digits;
mod object;
mod read_adapter;
mod string;
mod wrap;

pub use delimited::*;
pub use digits::Digits;
pub use object::*;
pub use read_adapter::ReadAdapter;
pub use string::*;
pub use wrap::*;
pub use array::*;
pub use iterator_iterator::IteratorIterator;

pub trait Wrapper {
    const START: u8;
    const END: u8;
}
pub trait Separator {
    const SEPARATOR: u8;
}

pub struct CurlyBraces;
impl Wrapper for CurlyBraces {
    const START: u8 = b'{';
    const END: u8 = b'}';
}
pub struct SquareBrackets;
impl Wrapper for SquareBrackets {
    const START: u8 = b'[';
    const END: u8 = b']';
}

pub struct Quotes;
impl Wrapper for Quotes {
    const START: u8 = b'"';
    const END: u8 = b'"';
}

pub struct Comma;
impl Separator for Comma {
    const SEPARATOR: u8 = b',';
}

pub struct Colon;
impl Separator for Colon {
    const SEPARATOR: u8 = b':';
}

//

//


