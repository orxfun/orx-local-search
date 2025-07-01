use crate::{
    criteria::duration::{input::DurationMatrix, move_generator::DurationMoveGenerator},
    problem::Tsp,
};
use orx_local_search::{Criterion, ObjectiveUnitOf, SolutionOf};
use orx_meta::queue::One;

pub struct Duration;

impl Criterion for Duration {
    type Problem = Tsp;

    type Input = DurationMatrix;

    type InputQueue = One<Self::Input>;

    type MoveGenerator = DurationMoveGenerator;

    fn move_generator() -> Self::MoveGenerator {
        DurationMoveGenerator
    }

    fn evaluate(
        tour: &SolutionOf<Self>,
        duration_matrix: &Self::Input,
    ) -> Option<ObjectiveUnitOf<Self>> {
        Some(duration_matrix.tour_cost(tour))
    }
}
