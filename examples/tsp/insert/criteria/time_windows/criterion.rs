use crate::insert::{
    criteria::time_windows::{TimeWindowInput, move_generator::TimeWindowMoveGenerator},
    problem::Tsp,
};
use orx_local_search::{Criterion, ObjectiveUnitOf, SolutionOf};
use orx_meta::queue::One;

#[derive(Default, Clone, Copy)]
pub struct TimeWindows;

impl Criterion for TimeWindows {
    type Problem = Tsp;

    type Input<'i> = &'i TimeWindowInput<'i>;

    type MoveGenerator<'i> = TimeWindowMoveGenerator;

    type InputQueue<'i> = One<Self::Input<'i>>;

    fn move_generator<'i>(self) -> Self::MoveGenerator<'i> {
        TimeWindowMoveGenerator
    }

    fn evaluate(
        self,
        tour: &SolutionOf<Self>,
        input: &Self::Input<'_>,
    ) -> Option<ObjectiveUnitOf<Self>> {
        input.tour_cost(tour)
    }
}
