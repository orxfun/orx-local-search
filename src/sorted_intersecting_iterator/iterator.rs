use crate::sorted_intersecting_iterator::composed_next::ComposedNext;

pub struct SortedIntersectingIter<I1, I2>(I1, I2)
where
    I1: Iterator,
    I2: Iterator<Item = I1::Item>,
    I1::Item: Ord;

impl<I1, I2> Iterator for SortedIntersectingIter<I1, I2>
where
    I1: Iterator,
    I2: Iterator<Item = I1::Item>,
    I1::Item: Ord,
{
    type Item = I1::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let next1 = self.0.next();
        let next2 = self.1.next();
        let mut result = ComposedNext::new(next1, next2);

        loop {
            match result {
                ComposedNext::BothYieldedSameValue(next) => return next,
                ComposedNext::FirstIteratorYieldedGreater { next1 } => {
                    result = ComposedNext::new(next1, self.1.next())
                }
                ComposedNext::FirstIteratorYieldedSmaller { next2 } => {
                    result = ComposedNext::new(self.0.next(), next2)
                }
                ComposedNext::OneIteratorConsumed => return None,
            }
        }
    }
}
