use crate::{
    criteria::capacity::{input::CapacityInput, move_generator::CapacityMoveGenerator},
    problem::Tsp,
};
use orx_local_search::{Criterion, ObjectiveUnitOf, SolutionOf};

pub struct Capacity;

impl Criterion for Capacity {
    type Problem = Tsp;

    type Input = CapacityInput;

    type MoveGenerator = CapacityMoveGenerator;

    fn move_generator() -> Self::MoveGenerator {
        CapacityMoveGenerator
    }

    fn evaluate(
        tour: &SolutionOf<Self>,
        capacity_input: &Self::Input,
    ) -> Option<ObjectiveUnitOf<Self>> {
        match capacity_input.is_tour_feasible(tour) {
            true => Some(0),
            false => None,
        }
    }
}
