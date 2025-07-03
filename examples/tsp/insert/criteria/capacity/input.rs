use orx_iterable::Collection;

use crate::{
    Tour,
    insert::{InsertMove, TourAfterInsertIter},
};

pub struct CapacityInput {
    vehicle_capacity: u64,
    city_capacity_delta: Vec<i64>,
}

impl CapacityInput {
    pub fn new(vehicle_capacity: u64, city_capacity_delta: Vec<i64>) -> Self {
        Self {
            vehicle_capacity,
            city_capacity_delta,
        }
    }

    pub fn is_tour_feasible(&self, tour: &Tour) -> bool {
        let feasible_range = 0..self.vehicle_capacity as i64;
        let mut current_capacity = 0i64;
        for city in tour.iter().copied() {
            current_capacity += self.city_capacity_delta[city];
            if !feasible_range.contains(&current_capacity) {
                return false;
            }
        }
        true
    }

    pub fn is_tour_feasible_after_move(&self, tour: &Tour, mv: &InsertMove) -> bool {
        let feasible_range = 0..self.vehicle_capacity as i64;
        let mut current_capacity = 0i64;
        for city in TourAfterInsertIter::new(mv.clone(), tour) {
            current_capacity += self.city_capacity_delta[city];
            if !feasible_range.contains(&current_capacity) {
                return false;
            }
        }
        true
    }

    pub fn example_input() -> Self {
        Self {
            vehicle_capacity: 15,
            city_capacity_delta: vec![7, 6, -6, 4, -8, -3],
        }
    }
}
