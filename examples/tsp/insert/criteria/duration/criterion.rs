use crate::insert::{
    criteria::duration::{DurationMatrix, move_generator::DurationMoveGenerator},
    neighborhood::InsertNeighborhood,
};
use orx_local_search::{Criterion, ObjectiveUnitOf, SolutionOf};
use orx_meta::queue::One;

#[derive(Default, Clone, Copy)]
pub struct Duration;

impl Criterion for Duration {
    type Neighborhood = InsertNeighborhood;

    type Input<'i> = &'i DurationMatrix;

    type MoveGenerator<'i> = DurationMoveGenerator;

    type InputQueue<'i> = One<Self::Input<'i>>;

    fn move_generator<'i>(self) -> Self::MoveGenerator<'i> {
        DurationMoveGenerator
    }

    fn evaluate(
        self,
        tour: &SolutionOf<Self>,
        duration_matrix: &Self::Input<'_>,
    ) -> Option<ObjectiveUnitOf<Self>> {
        Some(duration_matrix.tour_cost(tour))
    }
}
