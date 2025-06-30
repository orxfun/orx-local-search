use crate::{
    criteria::duration::input::DurationMatrix, neighborhood::AllInsertMovesIter, problem::Tsp,
    tour::Tour,
};
use orx_iterable::Collection;
use orx_local_search::CandidateMove;

pub struct DurationMoves<'a> {
    tour: &'a Tour,
    duration_matrix: &'a DurationMatrix,
    iter: AllInsertMovesIter,
}

impl<'a> DurationMoves<'a> {
    pub fn new(tour: &'a Tour, duration_matrix: &'a DurationMatrix) -> Self {
        let iter = AllInsertMovesIter::new(tour.iter().len());
        Self {
            tour,
            duration_matrix,
            iter,
        }
    }
}

impl Iterator for DurationMoves<'_> {
    type Item = CandidateMove<Tsp>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|mv| {
            let cost = self.duration_matrix.tour_cost_after_move(self.tour, &mv);
            CandidateMove::new(mv, cost)
        })
    }
}
