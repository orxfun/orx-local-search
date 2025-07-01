use crate::criterion::{CandidateMoveOf, Criterion, InputOf, SolutionOf};

pub trait MoveGenerator {
    type X: Criterion;

    fn moves(
        &mut self,
        solution: &SolutionOf<Self::X>,
        input: &InputOf<Self::X>,
    ) -> impl Iterator<Item = CandidateMoveOf<Self::X>>;
}
