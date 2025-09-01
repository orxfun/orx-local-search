use crate::{tour::Tour, tsp::Tsp};
use orx_iterable::Collection;
use orx_local_search::{Criterion, EvalSoln, Problem};

#[derive(Default, Clone, Copy)]
pub struct Capacity;

impl Criterion for Capacity {
    type Problem = Tsp;

    type Input<'i> = &'i CapacityInput;

    fn evaluate(
        input: &Self::Input<'_>,
        tour: &<Self::Problem as Problem>::Solution,
    ) -> EvalSoln<Self::Problem> {
        match input.is_tour_feasible(tour) {
            true => EvalSoln::Feasible(0),
            false => EvalSoln::Infeasible,
        }
    }
}

pub struct CapacityInput {
    pub vehicle_capacity: u64,
    pub city_capacity_delta: Vec<i64>,
}

impl CapacityInput {
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
