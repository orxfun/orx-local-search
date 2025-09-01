use crate::{
    criteria::capacity::{Capacity, CapacityInput},
    insert::{
        neighborhood::{AllInsertMovesIter, Insert, InsertMove},
        tour_after_move::TourAfterInsertIter,
    },
    tour::Tour,
};
use orx_iterable::Collection;
use orx_local_search::{Criterion, EvalMove, Moves, Problem};

#[derive(Default)]
pub struct InsertForCapacity;

impl<'i> Moves<'i> for InsertForCapacity {
    type Neighborhood = Insert;

    type X = Capacity;

    fn moves<'a>(
        &'a mut self,
        input: &'i <Self::X as Criterion>::Input<'i>,
        tour: &'a <<Self::X as Criterion>::Problem as Problem>::Solution,
    ) -> impl Iterator<Item = EvalMove<Self::Neighborhood>> + 'a
    where
        'i: 'a,
    {
        CapacityMoves {
            iter: AllInsertMovesIter::new(tour.iter().len()),
            input,
            tour,
        }
    }
}

// moves iter

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

    fn is_tour_feasible_after_move(&self, mv: &InsertMove) -> bool {
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
    type Item = EvalMove<Insert>;

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
