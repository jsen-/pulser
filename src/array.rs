use super::{delim, wrap, Comma, Delimited, SquareBrackets, Wrapped, IteratorIterator};

pub struct NilItem;
pub struct Item<V: IntoIterator<Item = u8>, T> {
    value: V,
    tail: T,
}

pub trait Itm: Sized {
    fn push<V: IntoIterator<Item = u8>>(self, value: V) -> Item<V, Self> {
        Item { value, tail: self }
    }
}

impl Itm for NilItem {}
impl<V: IntoIterator<Item = u8>, T> Itm for Item<V, T> {}

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

pub fn array() -> JsonArray<NilItem> {
    JsonArray(NilItem)
}

pub fn dynamic_array<T>(it: T) -> JsonArray<IteratorIterator<T::IntoIter, Comma>>
where
    T: IntoIterator,
    T::Item: IntoIterator<Item = u8>,
{
    JsonArray(IteratorIterator::new(Comma, it))
}

impl<P: Itm> JsonArray<P> {
    pub fn item<V: IntoIterator<Item = u8>>(
        self,
        value: V,
    ) -> JsonArray<Item<V, P>> {
        Self(self.0.push(value))
    }
}
