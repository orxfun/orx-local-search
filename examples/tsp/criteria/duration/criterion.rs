use crate::{
    criteria::duration::{input::DurationMatrix, move_generator::DurationMoveGenerator},
    problem::Tsp,
};
use orx_local_search::Criterion;

pub struct Duration;

impl Criterion for Duration {
    type Problem = Tsp;

    type Input<'a> = &'a DurationMatrix;

    type MoveGenerator = DurationMoveGenerator;

    fn move_generator(&self) -> Self::MoveGenerator {
        todo!()
    }
}
