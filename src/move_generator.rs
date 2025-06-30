use crate::criterion::{CandidateMoveOf, Criterion, InputOf, SolutionOf};

pub trait MoveGenerator {
    type X: Criterion;

    fn moves<'a, 'b, 'c>(
        &'a mut self,
        solution: &'b SolutionOf<Self::X>,
        input: InputOf<'c, Self::X>,
    ) -> impl Iterator<Item = CandidateMoveOf<Self::X>> + 'a + 'b + 'c;
}
