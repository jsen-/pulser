use super::Separator;
use std::marker::PhantomData;

pub struct IteratorIterator<T, W>
where
    T: IntoIterator,
    T::Item: IntoIterator<Item = u8>,
    W: Separator,
{
    current: Option<<T::Item as IntoIterator>::IntoIter>,
    it: T::IntoIter,
    delim: bool,
    p: PhantomData<W>,
}

impl<T, W> IteratorIterator<T, W>
where
    T: IntoIterator,
    T::Item: IntoIterator<Item = u8>,
    W: Separator,
{
    pub fn new(_delimiter: W, it: T) -> IteratorIterator<T::IntoIter, W> {
        let mut it = it.into_iter();
        IteratorIterator {
            current: it.next().map(|v| v.into_iter()),
            it: it,
            delim: false,
            p: PhantomData,
        }
    }
}

impl<T, W> Iterator for IteratorIterator<T, W>
where
    T: Iterator,
    T::Item: IntoIterator<Item = u8>,
    W: Separator,
{
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        if self.delim {
            self.delim = false;
            return Some(W::SEPARATOR);
        }
        self.current.take().and_then(|mut current| {
            current
                .next()
                .map(|next| {
                    self.current = Some(current);
                    next
                })
                .or_else(|| {
                    self.current = self.it.next().map(|next| {
                        self.delim = true;
                        next.into_iter()
                    });
                    self.next()
                })
        })
    }
}
