use std::marker::PhantomData;

use crate::{
    criteria::duration::{input::DurationMatrix, move_generator::DurationMoveGenerator},
    neighborhood::AllInsertMovesIter,
    problem::Tsp,
    tour::Tour,
};
use orx_iterable::Collection;
use orx_local_search::CandidateMove;

pub struct DurationMoves<'a, 's, 'i> {
    x: &'a DurationMoveGenerator,
    tour: &'s Tour,
    duration_matrix: &'i DurationMatrix,
    iter: AllInsertMovesIter,
}

impl<'a, 's, 'i> DurationMoves<'a, 's, 'i> {
    pub fn new(
        x: &'a DurationMoveGenerator,
        tour: &'s Tour,
        duration_matrix: &'i DurationMatrix,
    ) -> Self {
        let iter = AllInsertMovesIter::new(tour.iter().len());
        Self {
            x,
            tour,
            duration_matrix,
            iter,
        }
    }
}

impl<'a, 's, 'i> Iterator for DurationMoves<'a, 's, 'i> {
    type Item = CandidateMove<Tsp>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|mv| {
            let cost = self.duration_matrix.tour_cost_after_move(self.tour, &mv);
            CandidateMove::new(mv, cost)
        })
    }
}
