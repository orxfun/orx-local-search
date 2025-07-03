use crate::{
    Tour,
    insert::{AllInsertMovesIter, criteria::capacity::CapacityInput, problem::Tsp},
};
use orx_iterable::Collection;
use orx_local_search::{CandidateMove, CandidateMoveOf};

pub struct CapacityMoves<'a> {
    tour: &'a Tour,
    input: &'a CapacityInput,
    iter: AllInsertMovesIter,
}

impl<'a> CapacityMoves<'a> {
    pub fn new(tour: &'a Tour, input: &'a CapacityInput) -> Self {
        let iter = AllInsertMovesIter::new(tour.iter().len());
        Self { tour, input, iter }
    }
}

impl<'a> Iterator for CapacityMoves<'a> {
    type Item = CandidateMoveOf<Tsp>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.iter.next() {
                None => return None,
                Some(mv) if self.input.is_tour_feasible_after_move(self.tour, &mv) => {
                    return Some(CandidateMove::new(mv, 0));
                }
                _ => { /* infeasible move, continue to the next */ }
            }
        }
    }
}
