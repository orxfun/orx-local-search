use crate::criterion::{CandidateMoveOf, Criterion, InputOf, SolutionOf};

pub trait MoveGenerator {
    type X: Criterion;

    fn moves<'a>(
        &mut self,
        solution: &'a SolutionOf<Self::X>,
        input: &'a InputOf<Self::X>,
    ) -> impl Iterator<Item = CandidateMoveOf<Self::X>> + 'a;
}
