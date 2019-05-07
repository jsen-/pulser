use std::marker::PhantomData;
use super::{IntoOwned, Wrapper};

enum WrapState {
    Begin,
    Middle,
    End,
}

pub struct Wrapped<I, W>
where
    W: Wrapper,
    I: Iterator,
    I::Item: IntoOwned<u8>,
{
    it: I,
    state: WrapState,
    p: PhantomData<W>,
}

impl<I, W> Iterator for Wrapped<I, W>
where
    W: Wrapper,
    I: Iterator,
    I::Item: IntoOwned<u8>,
{
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        use WrapState::*;
        match &self.state {
            Begin => {
                self.state = Middle;
                Some(W::START)
            }
            Middle => self.it.next().map(|x| x.into_owned()).or_else(|| {
                self.state = End;
                Some(W::END)
            }),
            End => None,
        }
    }
}

pub fn wrap<W, I>(_w: W, it: I) -> Wrapped<I::IntoIter, W>
where
    I: IntoIterator,
    I::IntoIter: Iterator<Item = I::Item>,
    I::Item: IntoOwned<u8>,
    W: Wrapper,
{
    Wrapped {
        it: it.into_iter(),
        state: WrapState::Begin,
        p: PhantomData,
    }
}