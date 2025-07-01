use crate::{
    criteria::duration::{input::DurationMatrix, move_generator::DurationMoveGenerator},
    neighborhood::AllInsertMovesIter,
    problem::Tsp,
    tour::Tour,
};
use orx_iterable::Collection;
use orx_local_search::CandidateMove;

pub struct DurationMoves<'a> {
    move_generator: &'a DurationMoveGenerator,
    tour: &'a Tour,
    duration_matrix: &'a DurationMatrix,
    iter: AllInsertMovesIter,
}

impl<'a> DurationMoves<'a> {
    pub fn new(
        move_generator: &'a DurationMoveGenerator,
        tour: &'a Tour,
        duration_matrix: &'a DurationMatrix,
    ) -> Self {
        let iter = AllInsertMovesIter::new(tour.iter().len());
        Self {
            move_generator,
            tour,
            duration_matrix,
            iter,
        }
    }
}

impl<'a> Iterator for DurationMoves<'a> {
    type Item = CandidateMove<Tsp>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|mv| {
            let cost = self.duration_matrix.tour_cost_after_move(self.tour, &mv);
            CandidateMove::new(mv, cost)
        })
    }
}
