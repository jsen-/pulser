use super::{IntoOwned, Separator};
use std::marker::PhantomData;

enum DelimitedState {
    First,
    Delim,
    Done,
}

pub struct Delimited<I1, S, I2>
where
    I1: Iterator,
    I1::Item: IntoOwned<u8>,
    S: Separator,
    I2: Iterator,
    I2::Item: IntoOwned<u8>,
{
    i1: I1,
    i2: I2,
    state: DelimitedState,
    p: PhantomData<S>,
}

impl<I1, S, I2> Iterator for Delimited<I1, S, I2>
where
    I1: Iterator,
    I1::Item: IntoOwned<u8>,
    S: Separator,
    I2: Iterator,
    I2::Item: IntoOwned<u8>,
{
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        use DelimitedState::*;
        match self.state {
            First => self.i1.next().map(|k| k.into_owned()).or_else(|| {
                self.state = Delim;
                Some(S::SEPARATOR)
            }),
            Delim => self.i2.next().map(|v| v.into_owned()).or_else(|| {
                self.state = Done;
                None
            }),
            Done => None,
        }
    }
}

pub fn delim<I1, S, I2>(i1: I1, _separator: S, i2: I2) -> Delimited<I1::IntoIter, S, I2::IntoIter>
where
    I1: IntoIterator,
    I1::IntoIter: Iterator<Item = I1::Item>,
    I1::Item: IntoOwned<u8>,
    S: Separator,
    I2: IntoIterator,
    I2::IntoIter: Iterator<Item = I2::Item>,
    I2::Item: IntoOwned<u8>,
{
    Delimited {
        i1: i1.into_iter(),
        i2: i2.into_iter(),
        state: DelimitedState::First,
        p: PhantomData,
    }
}
