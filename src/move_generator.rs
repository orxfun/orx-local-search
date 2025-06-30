use crate::criterion::{CandidateMoveOf, Criterion, InputOf, SolutionOf};

pub trait MoveGenerator {
    type X: Criterion;

    fn moves<'a>(
        &mut self,
        object: &SolutionOf<Self::X>,
        input: InputOf<'a, Self::X>,
    ) -> impl Iterator<Item = CandidateMoveOf<Self::X>>;
}

pub type ProblemOf<M> = <<M as MoveGenerator>::X as Criterion>::Problem;
