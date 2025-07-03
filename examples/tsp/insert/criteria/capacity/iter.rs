use crate::{
    Tour,
    criteria::CapacityInput,
    insert::{
        AllInsertMovesIter, InsertMove, TourAfterInsertIter, neighborhood::InsertNeighborhood,
    },
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

    fn is_tour_feasible_after_move(input: &CapacityInput, tour: &Tour, mv: &InsertMove) -> bool {
        let feasible_range = 0..input.vehicle_capacity as i64;
        let mut current_capacity = 0i64;
        for city in TourAfterInsertIter::new(mv.clone(), tour) {
            current_capacity += input.city_capacity_delta[city];
            if !feasible_range.contains(&current_capacity) {
                return false;
            }
        }
        true
    }
}

impl<'a> Iterator for CapacityMoves<'a> {
    type Item = CandidateMoveOf<InsertNeighborhood>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.iter.next() {
                None => return None,
                Some(mv) if Self::is_tour_feasible_after_move(self.input, self.tour, &mv) => {
                    return Some(CandidateMove::new(mv, 0));
                }
                _ => { /* infeasible move, continue to the next */ }
            }
        }
    }
}
