use super::Wrapper;
use std::marker::PhantomData;

enum WrapState {
    Begin,
    Middle,
    End,
}

pub struct Wrapped<W, I>
where
    W: Wrapper,
    I: Iterator<Item = u8>,
{
    it: I,
    state: WrapState,
    p: PhantomData<W>,
}

impl<W, I> Iterator for Wrapped<W, I>
where
    W: Wrapper,
    I: Iterator<Item = u8>,
{
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        use WrapState::*;
        match &self.state {
            Begin => {
                self.state = Middle;
                Some(W::START)
            }
            Middle => self.it.next().or_else(|| {
                self.state = End;
                Some(W::END)
            }),
            End => None,
        }
    }
}

pub fn wrap<W, I>(_w: W, it: I) -> Wrapped<W, I::IntoIter>
where
    I: IntoIterator<Item = u8>,
    I::IntoIter: Iterator<Item = I::Item>,
    W: Wrapper,
{
    Wrapped {
        it: it.into_iter(),
        state: WrapState::Begin,
        p: PhantomData,
    }
}
