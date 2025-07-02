use crate::{
    CandidateMove, Move, ObjectiveValue, sorted_intersecting_iterator::composed_next::ComposedNext,
};

pub struct SortedIntersectingIter<I1, I2, M, O>
where
    M: Move,
    O: ObjectiveValue,
    I1: Iterator<Item = CandidateMove<M, O>>,
    I2: Iterator<Item = CandidateMove<M, O>>,
{
    iter1: I1,
    iter2: I2,
}

impl<I1, I2, M, O> SortedIntersectingIter<I1, I2, M, O>
where
    M: Move,
    O: ObjectiveValue,
    I1: Iterator<Item = CandidateMove<M, O>>,
    I2: Iterator<Item = CandidateMove<M, O>>,
{
    pub fn new(iter1: I1, iter2: I2) -> Self {
        Self { iter1, iter2 }
    }
}

impl<I1, I2, M, O> Iterator for SortedIntersectingIter<I1, I2, M, O>
where
    M: Move,
    O: ObjectiveValue,
    I1: Iterator<Item = CandidateMove<M, O>>,
    I2: Iterator<Item = CandidateMove<M, O>>,
{
    type Item = I1::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let next1 = self.iter1.next();
        let next2 = self.iter2.next();
        let mut result = ComposedNext::new(next1, next2);

        loop {
            match result {
                ComposedNext::BothYieldedSameValue(next) => return Some(next),
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
