use std::io;

pub struct ReadAdapter<T: Iterator<Item = u8>>(T);
impl<T> ReadAdapter<T>
where
    T: Iterator<Item = u8>,
{
    pub fn new<I>(it: I) -> Self
    where
        I: IntoIterator<Item = u8, IntoIter = T>,
    {
        Self(it.into_iter())
    }
}

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
