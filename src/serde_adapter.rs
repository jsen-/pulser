use std::io::{Cursor, Read, Result};

#[derive(Debug)]
enum AdapterState {
    Begin(Cursor<[u8; 2]>),
    Buffer(Cursor<Vec<u8>>),
    Delimiter(Cursor<[u8; 2]>, Option<Cursor<Vec<u8>>>),
    End(Cursor<[u8; 2]>),
    Done,
}

pub struct SerdeAdapter<I>
where
    I: Iterator,
    I::Item: serde::Serialize,
{
    state: AdapterState,
    it: I,
}

const BEGIN: [u8; 2] = *b"[\n";
const END: [u8; 2] = *b"\n]";
const DELIMITER: [u8; 2] = *b",\n";

impl<I> SerdeAdapter<I>
where
    I: Iterator,
    I::Item: serde::Serialize,
{
    pub fn new<T>(it: T) -> Self
    where
        T: IntoIterator<IntoIter = I, Item = I::Item>,
    {
        Self {
            state: AdapterState::Begin(Cursor::new(BEGIN)),
            it: it.into_iter(),
        }
    }
}

impl<I> Read for SerdeAdapter<I>
where
    I: Iterator,
    I::Item: serde::Serialize,
{
    fn read(&mut self, target: &mut [u8]) -> Result<usize> {
        let max = target.len();
        let mut bytes = 0;
        loop {
            use AdapterState::*;
            match self.state {
                Begin(ref mut begin) => {
                    bytes += begin.read(&mut target[bytes..])?;
                    if bytes == max {
                        return Ok(bytes);
                    }
                    self.state = self
                        .it
                        .next()
                        .as_ref()
                        .map(|next| Buffer(Cursor::new(serde_json::to_vec(next).unwrap())))
                        .unwrap_or(End(Cursor::new(END)));
                }
                Buffer(ref mut buffer) => {
                    bytes += buffer.read(&mut target[bytes..])?;
                    if bytes == max {
                        return Ok(bytes);
                    }
                    self.state = self
                        .it
                        .next()
                        .as_ref()
                        .map(|next| {
                            Delimiter(
                                Cursor::new(DELIMITER),
                                Some(Cursor::new(serde_json::to_vec(next).unwrap())),
                            )
                        })
                        .unwrap_or(End(Cursor::new(END)));
                }
                Delimiter(ref mut delimiter, ref mut next) => {
                    bytes += delimiter.read(&mut target[bytes..])?;
                    if bytes == max {
                        return Ok(bytes);
                    }
                    self.state = Buffer(next.take().unwrap());
                }
                End(ref mut end) => {
                    bytes += end.read(&mut target[bytes..])?;
                    if bytes == max {
                        return Ok(bytes);
                    }
                    self.state = Done;
                }
                Done => return Ok(bytes),
            }
        }
    }
}
