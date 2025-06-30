use std::marker::PhantomData;

use crate::{
    criteria::duration::input::DurationMatrix, neighborhood::AllInsertMovesIter, problem::Tsp,
    tour::Tour,
};
use orx_iterable::Collection;
use orx_local_search::CandidateMove;

pub struct DurationMoves<'a, 'b, 'c>
where
    'a: 'b + 'c,
    'b: 'c,
{
    p: PhantomData<&'a ()>,
    tour: &'b Tour,
    duration_matrix: &'c DurationMatrix,
    iter: AllInsertMovesIter,
}

impl<'a, 'b, 'c> DurationMoves<'a, 'b, 'c>
where
    'a: 'b + 'c,
    'b: 'c,
{
    pub fn new(tour: &'a Tour, duration_matrix: &'b DurationMatrix) -> Self {
        let iter = AllInsertMovesIter::new(tour.iter().len());
        Self {
            p: PhantomData,
            tour,
            duration_matrix,
            iter,
        }
    }
}

impl<'a, 'b, 'c> Iterator for DurationMoves<'a, 'b, 'c> {
    type Item = CandidateMove<Tsp>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|mv| {
            let cost = self.duration_matrix.tour_cost_after_move(self.tour, &mv);
            CandidateMove::new(mv, cost)
        })
    }
}
