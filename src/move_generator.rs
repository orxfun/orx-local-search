use crate::criterion::{CandidateMoveOf, Criterion, InputOf, SolutionOf};

pub trait MoveGenerator {
    type X: Criterion;

    fn moves<'s, 'i>(
        &mut self,
        solution: &'s SolutionOf<Self::X>,
        input: InputOf<'i, Self::X>,
    ) -> impl Iterator<Item = CandidateMoveOf<Self::X>> + '_ + 's + 'i;
}
