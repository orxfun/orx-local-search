use crate::{
    criteria::CapacityInput,
    insert_neighborhood::{
        r#move::Insert,
        neighborhood::{AllInsertMovesIter, InsertNeighborhood},
        tour_after_insert::TourAfterInsertIter,
    },
    tour::Tour,
};
use orx_iterable::Collection;
use orx_local_search::EvalMove;

pub struct CapacityMoves<'a> {
    input: &'a CapacityInput,
    tour: &'a Tour,
    iter: AllInsertMovesIter,
}

impl<'a> CapacityMoves<'a> {
    pub fn new(input: &'a CapacityInput, tour: &'a Tour) -> Self {
        let iter = AllInsertMovesIter::new(tour.iter().len());
        Self { tour, input, iter }
    }

    fn is_tour_feasible_after_move(&self, mv: &Insert) -> bool {
        let feasible_range = 0..self.input.vehicle_capacity as i64;
        let mut current_capacity = 0i64;
        for city in TourAfterInsertIter::new(mv.clone(), self.tour) {
            current_capacity += self.input.city_capacity_delta[city];
            if !feasible_range.contains(&current_capacity) {
                return false;
            }
        }
        true
    }
}

impl<'a> Iterator for CapacityMoves<'a> {
    type Item = EvalMove<InsertNeighborhood>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.iter.next() {
                None => return None,
                Some(mv) if self.is_tour_feasible_after_move(&mv) => {
                    return Some(EvalMove::new(mv, 0));
                }
                _ => { /* infeasible move, continue to the next */ }
            }
        }
    }
}
