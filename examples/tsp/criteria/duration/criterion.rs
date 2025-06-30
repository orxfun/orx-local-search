use crate::{
    criteria::duration::{input::DurationMatrix, move_generator::DurationMoveGenerator},
    problem::Tsp,
};
use orx_local_search::{Criterion, ObjectiveUnitOf, SolutionOf};

pub struct Duration;

impl Criterion for Duration {
    type Problem = Tsp;

    type Input<'a> = &'a DurationMatrix;

    type MoveGenerator = DurationMoveGenerator;

    fn move_generator(&self) -> Self::MoveGenerator {
        DurationMoveGenerator
    }

    fn evaluate<'a>(
        &self,
        tour: &'a SolutionOf<Self>,
        duration_matrix: &'a DurationMatrix,
    ) -> Option<ObjectiveUnitOf<Self>> {
        Some(duration_matrix.tour_cost(tour))
    }
}
