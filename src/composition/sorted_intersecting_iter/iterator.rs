use super::composed_next::ComposedNext;
use crate::core::{EvalMove, Neighborhood, Problem};

pub struct SortedIntersectingIter<P, N, I1, I2>
where
    P: Problem,
    N: Neighborhood<P>,
    I1: Iterator<Item = EvalMove<P, N>>,
    I2: Iterator<Item = EvalMove<P, N>>,
{
    iter1: I1,
    iter2: I2,
}

impl<P, N, I1, I2> SortedIntersectingIter<P, N, I1, I2>
where
    P: Problem,
    N: Neighborhood<P>,
    I1: Iterator<Item = EvalMove<P, N>>,
    I2: Iterator<Item = EvalMove<P, N>>,
{
    pub fn new(iter1: I1, iter2: I2) -> Self {
        Self { iter1, iter2 }
    }
}

impl<P, N, I1, I2> Iterator for SortedIntersectingIter<P, N, I1, I2>
where
    P: Problem,
    N: Neighborhood<P>,
    I1: Iterator<Item = EvalMove<P, N>>,
    I2: Iterator<Item = EvalMove<P, N>>,
{
    type Item = I1::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let next1 = self.iter1.next();
        let next2 = self.iter2.next();
        let mut result = ComposedNext::new(next1, next2);

        loop {
            match result {
                ComposedNext::BothYieldedSameValue { composed } => return Some(composed),
                ComposedNext::FirstIteratorYieldedGreater { next1 } => {
                    result = ComposedNext::new_with_left(next1, self.iter2.next())
                }
                ComposedNext::FirstIteratorYieldedSmaller { next2 } => {
                    result = ComposedNext::new_with_right(self.iter1.next(), next2)
                }
                ComposedNext::OneIteratorConsumed => return None,
            }
        }
    }
}
