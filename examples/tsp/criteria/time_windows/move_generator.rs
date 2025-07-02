use crate::{
    criteria::time_windows::{TimeWindowInput, iter::TimeWindowMoves},
    problem::Tsp,
};
use orx_local_search::{CandidateMoveOf, MoveGenerator, Problem};

pub struct TimeWindowMoveGenerator;

impl<'i> MoveGenerator<'i> for TimeWindowMoveGenerator {
    type Problem = Tsp;

    type Input = &'i TimeWindowInput<'i>;

    fn moves<'a>(
        &'a mut self,
        tour: &'a <Self::Problem as Problem>::Solution,
        input: &'a Self::Input,
    ) -> impl Iterator<Item = CandidateMoveOf<Self::Problem>> + 'a {
        TimeWindowMoves::new(tour, input)
    }
}
