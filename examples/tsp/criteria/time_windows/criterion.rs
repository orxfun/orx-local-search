use crate::{
    criteria::time_windows::{TimeWindowInput, move_generator::TimeWindowMoveGenerator},
    problem::Tsp,
};
use orx_local_search::{Criterion, ObjectiveUnitOf, SolutionOf};

pub struct TimeWindows;

impl Criterion for TimeWindows {
    type Problem = Tsp;

    type Input = TimeWindowInput;

    type MoveGenerator = TimeWindowMoveGenerator;

    fn move_generator() -> Self::MoveGenerator {
        TimeWindowMoveGenerator
    }

    fn evaluate(tour: &SolutionOf<Self>, input: &Self::Input) -> Option<ObjectiveUnitOf<Self>> {
        input.tour_cost(tour)
    }
}
