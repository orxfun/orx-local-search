use crate::{
    ObjectiveValue, candidate_move::CandidateMove, move_generator::MoveGenerator, problem::Problem,
};

pub trait Criterion {
    type Problem: Problem;

    type Input<'a>;

    type MoveGenerator: MoveGenerator<X = Self>;

    fn move_generator(&self) -> Self::MoveGenerator;

    fn evaluate<'a>(
        &self,
        solution: &'a SolutionOf<Self>,
        input: Self::Input<'a>,
    ) -> Option<ObjectiveUnitOf<Self>>;
}

pub type CandidateMoveOf<X> = CandidateMove<<X as Criterion>::Problem>;

pub type SolutionOf<X> = <<X as Criterion>::Problem as Problem>::Solution;

pub type InputOf<'a, X> = <X as Criterion>::Input<'a>;

pub type ObjectiveUnitOf<X> =
    <<<X as Criterion>::Problem as Problem>::ObjectiveValue as ObjectiveValue>::Unit;
