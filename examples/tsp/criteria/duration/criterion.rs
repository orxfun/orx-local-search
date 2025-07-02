use crate::{
    criteria::duration::{input::DurationMatrix, move_generator::DurationMoveGenerator},
    problem::Tsp,
};
use orx_local_search::{ComposedCriteria, Criterion, ObjectiveUnitOf, SolutionOf};
use orx_meta::queue::One;

#[derive(Default, Clone, Copy)]
pub struct Duration;

impl Criterion for Duration {
    type Problem = Tsp;

    type Input<'i> = &'i DurationMatrix;

    type MoveGenerator<'i> = DurationMoveGenerator;

    type InputQueue<'i> = One<Self::Input<'i>>;

    type ComposeWith<X>
        = ComposedCriteria<Self, X>
    where
        X: Criterion<Problem = Self::Problem>;

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
