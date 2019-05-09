#![allow(dead_code, unused_variables)]
mod array;
mod delimited;
mod digits;
mod object;
mod read_adapter;
mod string;
mod wrap;

use delimited::*;
use digits::Digits;
use object::object;
use read_adapter::ReadAdapter;
use string::string;
use wrap::*;

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

struct DbRow {
    id: isize,
    name: String,
    age: u8,
}

impl IntoIterator for DbRow {
    type Item = u8;
    type IntoIter = Box<Iterator<Item = u8>>;

    fn into_iter(self) -> Self::IntoIter {
        let id = self.id.digits();
        let name = string(self.name);
        let age = self.age.digits();
        Box::new(
            object()
                .prop("age", age)
                .prop("name", name)
                .prop("id", id)
                .into_iter(),
        )
    }
}

//

//

//

fn main() {
    let mut r = ReadAdapter::new((std::f32::NAN).digits());
    let stdout = std::io::stdout();
    let mut l = stdout.lock();
    std::io::copy(&mut r, &mut l).unwrap();
    println!();
    println!();

    let mut r = ReadAdapter::new((-14987_isize).digits());
    let stdout = std::io::stdout();
    let mut l = stdout.lock();
    std::io::copy(&mut r, &mut l).unwrap();
    println!();
    println!();

    //

    let row1 = DbRow {
        id: 1,
        name: "Jozo".to_string(),
        age: 70,
    };
    let mut r = ReadAdapter::new(row1.into_iter());
    let stdout = std::io::stdout();
    let mut l = stdout.lock();
    std::io::copy(&mut r, &mut l).unwrap();
    println!();
    println!();

    //

    let row2 = DbRow {
        id: 2,
        name: "Milan".to_string(),
        age: 52,
    };
    let row3 = DbRow {
        id: 3,
        name: "Cecilka".to_string(),
        age: 92,
    };
    let r = array::from_iter(vec![row2, row3].into_iter());

    let mut r = ReadAdapter::new(r);
    let stdout = std::io::stdout();
    let mut l = stdout.lock();
    std::io::copy(&mut r, &mut l).unwrap();
    println!();
    println!();
}
