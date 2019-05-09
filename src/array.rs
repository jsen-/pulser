use super::{delim, wrap, Comma, Delimited, SquareBrackets, Wrapped};

pub struct NilItem;
pub struct Item<V: IntoIterator<Item = u8>, T> {
    value: V,
    tail: T,
}

pub trait Prop: Sized {
    fn push<V: IntoIterator<Item = u8>>(self, name: &'static str, value: V) -> Item<V, Self> {
        Item { value, tail: self }
    }
}

impl Prop for NilItem {}
impl<V: IntoIterator<Item = u8>, T> Prop for Item<V, T> {}

impl<V, T, U> IntoIterator for Item<V, Item<T, U>>
where
    V: IntoIterator<Item = u8>,
    Item<T, U>: IntoIterator<Item = u8>,
    T: IntoIterator<Item = u8>,
{
    type IntoIter = Delimited<V::IntoIter, Comma, <Item<T, U> as IntoIterator>::IntoIter>;
    type Item = u8;

    fn into_iter(self) -> Self::IntoIter {
        delim(self.value.into_iter(), Comma, self.tail.into_iter())
    }
}

impl<V> IntoIterator for Item<V, NilItem>
where
    V: IntoIterator<Item = u8>,
{
    type IntoIter = V::IntoIter;
    type Item = u8;

    fn into_iter(self) -> Self::IntoIter {
        self.value.into_iter()
    }
}

pub struct JsonArray<P>(P);
impl<P> IntoIterator for JsonArray<P>
where
    P: IntoIterator<Item = u8>,
{
    type Item = u8;
    type IntoIter = Wrapped<SquareBrackets, P::IntoIter>;

    fn into_iter(self) -> Self::IntoIter {
        wrap(SquareBrackets, self.0.into_iter())
    }
}

pub struct IteratorIterator<T>
where
    T: Iterator,
    T::Item: IntoIterator<Item = u8>,
{
    current: Option<<T::Item as IntoIterator>::IntoIter>,
    it: T,
    delim: bool,
}

impl<T> Iterator for IteratorIterator<T>
where
    T: Iterator,
    T::Item: IntoIterator<Item = u8>,
{
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        if self.delim {
            self.delim = false;
            return Some(b',');
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

pub fn array() -> JsonArray<NilItem> {
    JsonArray(NilItem)
}

pub fn from_iter<T>(mut it: T) -> JsonArray<IteratorIterator<T>>
where
    T: Iterator,
    T::Item: IntoIterator<Item = u8>,
{
    JsonArray(IteratorIterator {
        current: it.next().map(|v| v.into_iter()),
        it: it,
        delim: false,
    })
}

impl<P: Prop> JsonArray<P> {
    pub fn prop<V: IntoIterator<Item = u8>>(
        self,
        name: &'static str,
        value: V,
    ) -> JsonArray<Item<V, P>> {
        Self(self.0.push(name, value))
    }
}
