use crate::{
    Tour,
    insert::{AllInsertMovesIter, criteria::time_windows::TimeWindowInput, problem::Tsp},
};
use orx_iterable::Collection;
use orx_local_search::{CandidateMove, CandidateMoveOf};

pub struct TimeWindowMoves<'a> {
    tour: &'a Tour,
    input: &'a TimeWindowInput<'a>,
    iter: AllInsertMovesIter,
}

impl<'a> TimeWindowMoves<'a> {
    pub fn new(tour: &'a Tour, input: &'a TimeWindowInput) -> Self {
        let iter = AllInsertMovesIter::new(tour.iter().len());
        Self { tour, input, iter }
    }
}

impl<'a> Iterator for TimeWindowMoves<'a> {
    type Item = CandidateMoveOf<Tsp>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.iter.next() {
                None => return None,
                Some(mv) => {
                    match self.input.tour_cost_after_move(self.tour, &mv) {
                        None => { /* infeasible move, continue to the next */ }
                        Some(cost) => return Some(CandidateMove::new(mv, cost)),
                    }
                }
            }
        }
    }
}
