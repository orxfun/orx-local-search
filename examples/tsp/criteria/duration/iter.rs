use crate::{
    all_moves::AllInsertMovesIter, criteria::duration::input::DurationMatrix, problem::Tsp,
    tour::Tour,
};
use orx_local_search::CandidateMove;

pub struct DurationMoves<'a> {
    tour: &'a Tour,
    duration_matrix: &'a DurationMatrix,
    iter: AllInsertMovesIter,
}

impl Iterator for DurationMoves<'_> {
    type Item = CandidateMove<Tsp>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|mv| {
            let cost = self.duration_matrix.tour_cost(self.tour, &mv);
            CandidateMove::new(mv, cost)
        })
    }
}
