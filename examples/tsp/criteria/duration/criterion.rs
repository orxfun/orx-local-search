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
        solution: &'a SolutionOf<Self>,
        input: Self::Input<'a>,
    ) -> Option<ObjectiveUnitOf<Self>> {
    }
}
