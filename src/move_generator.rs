use crate::{CandidateMoveOf, Problem};

pub trait MoveGenerator {
    type Problem: Problem;

    type Input;

    fn moves<'a>(
        &'a mut self,
        solution: &'a <Self::Problem as Problem>::Solution,
        input: &'a Self::Input,
    ) -> impl Iterator<Item = CandidateMoveOf<Self::Problem>> + 'a;
}
