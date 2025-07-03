use crate::{Tour, tsp::Tsp};
use orx_iterable::Collection;
use orx_local_search::{Criterion, ObjectiveUnitOf, SolutionOf};
use orx_meta::queue::One;

#[derive(Default, Clone, Copy)]
pub struct Capacity;

impl Criterion for Capacity {
    type Problem = Tsp;

    type Input<'i> = &'i CapacityInput;

    type InputQueue<'i> = One<Self::Input<'i>>;

    fn evaluate(
        self,
        tour: &SolutionOf<Self>,
        capacity_input: &Self::Input<'_>,
    ) -> Option<ObjectiveUnitOf<Self>> {
        match capacity_input.is_tour_feasible(tour) {
            true => Some(0),
            false => None,
        }
    }
}

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

    pub fn example_input() -> Self {
        Self {
            vehicle_capacity: 15,
            city_capacity_delta: vec![7, 6, -6, 4, -8, -3],
        }
    }
}
