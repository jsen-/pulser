use super::{delim, wrap, Colon, Comma, CurlyBraces, Delimited, IteratorIterator, Quotes, Wrapped};
use std::str::Bytes;

pub struct NilProp;
pub struct Property<V: IntoIterator<Item = u8>, T> {
    name: &'static str,
    value: V,
    tail: T,
}

pub trait Prop: Sized {
    fn push<V: IntoIterator<Item = u8>>(self, name: &'static str, value: V) -> Property<V, Self> {
        Property {
            name,
            value,
            tail: self,
        }
    }
}

impl Prop for NilProp {}
impl<V: IntoIterator<Item = u8>, T> Prop for Property<V, T> {}

impl<V, T, U> IntoIterator for Property<V, Property<T, U>>
where
    V: IntoIterator<Item = u8>,
    Property<T, U>: IntoIterator<Item = u8>,
    T: IntoIterator<Item = u8>,
{
    type IntoIter = Delimited<
        Delimited<Wrapped<Quotes, Bytes<'static>>, Colon, V::IntoIter>,
        Comma,
        <Property<T, U> as IntoIterator>::IntoIter,
    >;
    type Item = u8;

    fn into_iter(self) -> Self::IntoIter {
        delim(
            delim(
                wrap(Quotes, self.name.bytes()),
                Colon,
                self.value.into_iter(),
            ),
            Comma,
            self.tail.into_iter(),
        )
    }
}

impl<V> IntoIterator for Property<V, NilProp>
where
    V: IntoIterator<Item = u8>,
{
    type IntoIter = Delimited<Wrapped<Quotes, Bytes<'static>>, Colon, V::IntoIter>;
    type Item = u8;

    fn into_iter(self) -> Self::IntoIter {
        delim(
            wrap(Quotes, self.name.bytes()),
            Colon,
            self.value.into_iter(),
        )
    }
}

pub struct JsonObject<P>(P);
impl<P> IntoIterator for JsonObject<P>
where
    P: IntoIterator<Item = u8>,
{
    type Item = u8;
    type IntoIter = Wrapped<CurlyBraces, P::IntoIter>;

    fn into_iter(self) -> Self::IntoIter {
        wrap(CurlyBraces, self.0.into_iter())
    }
}

pub fn dynamic_object<T, K, V>(it: T) -> JsonObject<impl Iterator<Item = u8>>
where
    K: IntoIterator<Item = u8>,
    V: IntoIterator<Item = u8>,
    T: IntoIterator<Item = (K, V)>,
{
    // let inner_it: T::Item = it.into_iter().next().unwrap();
    // let (key, value) = inner_it.into_iter().next().unwrap();

    let it = it
        .into_iter()
        .map(|(key, value)| delim(wrap(Quotes, key.into_iter()), Colon, value.into_iter()));

    JsonObject(IteratorIterator::new(Comma, it))
}

pub fn object() -> JsonObject<NilProp> {
    JsonObject(NilProp)
}

impl<P: Prop> JsonObject<P> {
    pub fn prop<V: IntoIterator<Item = u8>>(
        self,
        name: &'static str,
        value: V,
    ) -> JsonObject<Property<V, P>> {
        Self(self.0.push(name, value))
    }
}
