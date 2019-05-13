use super::{wrap, Quotes, Wrapped};

pub struct StringIterator(String, usize);
impl StringIterator {
    pub fn new(str: String) -> Self {
        Self(str, 0)
    }
}
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

pub struct JsonString<P>(P);
impl<P> IntoIterator for JsonString<P>
where
    P: IntoIterator<Item = u8>,
{
    type Item = u8;
    type IntoIter = Wrapped<Quotes, P::IntoIter>;

    fn into_iter(self) -> Self::IntoIter {
        wrap(Quotes, self.0)
    }
}

pub fn string(s: String) -> JsonString<StringIterator> {
    JsonString(StringIterator::new(s))
}
