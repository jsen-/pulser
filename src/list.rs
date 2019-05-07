
pub struct List<L>
where
    L: Iterator<Item = Box<Iterator<Item = u8>>>,
{
    it: L,
    current: Option<Box<Iterator<Item = u8>>>,
}

impl<L> Iterator for List<L>
where
    L: Iterator<Item = Box<Iterator<Item = u8>>>,
{
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current.take();
        current.and_then(|mut current| match current.next() {
            Some(x) => {
                self.current = Some(current);
                Some(x)
            }
            None => {
                self.current = self.it.next();
                self.next()
            }
        })
    }
}
