use crate::{
    criteria::time_windows::{TimeWindowInput, move_generator::TimeWindowMoveGenerator},
    problem::Tsp,
};
use orx_local_search::{ComposedCriteria, Criterion, ObjectiveUnitOf, SolutionOf};
use orx_meta::queue::One;

#[derive(Default, Clone, Copy)]
pub struct TimeWindows;

impl Criterion for TimeWindows {
    type Problem = Tsp;

    type Input = TimeWindowInput;

    type MoveGenerator = TimeWindowMoveGenerator;

    type InputQueue = One<Self::Input>;

    type ComposeWith<X>
        = ComposedCriteria<Self, X>
    where
        X: Criterion<Problem = Self::Problem>;

    fn move_generator(self) -> Self::MoveGenerator {
        TimeWindowMoveGenerator
    }

    fn evaluate(
        self,
        tour: &SolutionOf<Self>,
        input: &Self::Input,
    ) -> Option<ObjectiveUnitOf<Self>> {
        input.tour_cost(tour)
    }
}
