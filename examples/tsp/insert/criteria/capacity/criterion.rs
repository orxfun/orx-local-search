use crate::insert::{
    criteria::capacity::{CapacityInput, move_generator::CapacityMoveGenerator},
    neighborhood::InsertNeighborhood,
    problem::Tsp,
};
use orx_local_search::{Criterion, ObjectiveUnitOf, SolutionOf};
use orx_meta::queue::One;

#[derive(Default, Clone, Copy)]
pub struct Capacity;

impl Criterion for Capacity {
    type Neighborhood = InsertNeighborhood;

    type Input<'i> = &'i CapacityInput;

    type MoveGenerator<'i> = CapacityMoveGenerator;

    type InputQueue<'i> = One<Self::Input<'i>>;

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
