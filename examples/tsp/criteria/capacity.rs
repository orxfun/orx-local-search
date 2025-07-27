use crate::{tour::Tour, tour_cost::TourCost, tsp::Tsp};
use orx_iterable::Collection;
use orx_local_search::{Criterion, Objective};

#[derive(Default, Clone, Copy)]
pub struct Capacity;

impl Criterion for Capacity {
    type Input = CapacityInput;

    type Solution = Tour;

    type Objective = TourCost;

    type Problem = Tsp;

    fn evaluate(
        self,
        input: &Self::Input,
        tour: &Self::Solution,
    ) -> Option<<Self::Objective as Objective>::Unit> {
        match input.is_tour_feasible(tour) {
            true => Some(0),
            false => None,
        }
    }
}

pub struct CapacityInput {
    pub vehicle_capacity: u64,
    pub city_capacity_delta: Vec<i64>,
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

    pub fn example_input() -> Self {
        Self {
            vehicle_capacity: 15,
            city_capacity_delta: vec![7, 6, -6, 4, -8, -3],
        }
    }
}
