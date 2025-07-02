use crate::{
    criteria::capacity::{input::CapacityInput, move_generator::CapacityMoveGenerator},
    problem::Tsp,
};
use orx_local_search::{ComposedCriteria, Criterion, ObjectiveUnitOf, SolutionOf};
use orx_meta::queue::One;

#[derive(Default, Clone, Copy)]
pub struct Capacity;

impl Criterion for Capacity {
    type Problem = Tsp;

    type Input<'i> = CapacityInput;

    type MoveGenerator<'i> = CapacityMoveGenerator;

    type InputQueue<'i> = One<Self::Input<'i>>;

    type ComposeWith<X>
        = ComposedCriteria<Self, X>
    where
        X: Criterion<Problem = Self::Problem>;

    fn move_generator<'i>(self) -> Self::MoveGenerator<'i> {
        CapacityMoveGenerator
    }

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
