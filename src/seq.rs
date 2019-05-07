use super::IntoOwned;

enum SeqState {
    First,
    Second,
    Done,
}

pub struct Seq<I1, I2>(SeqState, I1, I2);

impl<I1, I2> Iterator for Seq<I1, I2>
where
    I1: Iterator,
    I2: Iterator,

    I1::Item: IntoOwned<u8>,
    I2::Item: IntoOwned<u8>,
{
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        use SeqState::*;
        match &self.0 {
            First => self.1.next().map(|k| k.into_owned()).or_else(|| {
                self.0 = Second;
                self.next()
            }),
            Second => self.2.next().map(|v| v.into_owned()).or_else(|| {
                self.0 = Done;
                None
            }),
            Done => None,
        }
    }
}

pub fn seq2<I1, I2>(first: I1, second: I2) -> Seq<I1::IntoIter, I2::IntoIter>
where
    I1: IntoIterator,
    I2: IntoIterator,
    I1::IntoIter: Iterator<Item = I1::Item>,
    I2::IntoIter: Iterator<Item = I2::Item>,
    I1::Item: IntoOwned<u8>,
    I2::Item: IntoOwned<u8>,
{
    let first = first.into_iter();
    let second = second.into_iter();
    Seq(SeqState::First, first, second)
}

pub fn sequence3<I1, I2, I3>(
    first: I1,
    second: I2,
    third: I3,
) -> Seq<I1::IntoIter, Seq<I2::IntoIter, I3::IntoIter>>
where
    I1: IntoIterator,
    I1::IntoIter: Iterator<Item = I1::Item>,
    I1::Item: IntoOwned<u8>,
    I2: IntoIterator,
    I2::Item: IntoOwned<u8>,
    I2::IntoIter: Iterator<Item = I2::Item>,
    I3: IntoIterator,
    I3::Item: IntoOwned<u8>,
    I3::IntoIter: Iterator<Item = I3::Item>,
{
    let first = first.into_iter();
    let second = second.into_iter();
    let third = third.into_iter();
    seq2(first, seq2(second, third))
}

pub fn sequence4<I1, I2, I3, I4>(
    first: I1,
    second: I2,
    third: I3,
    fourth: I4,
) -> Seq<Seq<I1::IntoIter, I2::IntoIter>, Seq<I3::IntoIter, I4::IntoIter>>
where
    I1: IntoIterator,
    I1::IntoIter: Iterator<Item = I1::Item>,
    I1::Item: IntoOwned<u8>,
    I2: IntoIterator,
    I2::Item: IntoOwned<u8>,
    I2::IntoIter: Iterator<Item = I2::Item>,
    I3: IntoIterator,
    I3::Item: IntoOwned<u8>,
    I3::IntoIter: Iterator<Item = I3::Item>,
    I4: IntoIterator,
    I4::Item: IntoOwned<u8>,
    I4::IntoIter: Iterator<Item = I4::Item>,
{
    let first = first.into_iter();
    let second = second.into_iter();
    let third = third.into_iter();
    let fourth = fourth.into_iter();

    seq2(seq2(first, second), seq2(third, fourth))
}
