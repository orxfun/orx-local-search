use crate::{
    ObjectiveValue, candidate_move::CandidateMove, move_generator::MoveGenerator, problem::Problem,
};

pub trait Criterion {
    type Problem: Problem;

    type Input;

    type MoveGenerator: MoveGenerator<Problem = Self::Problem, Input = Self::Input>;

    fn move_generator() -> Self::MoveGenerator;

    fn evaluate(solution: &SolutionOf<Self>, input: &Self::Input) -> Option<ObjectiveUnitOf<Self>>;
}

pub type SolutionOf<X> = <<X as Criterion>::Problem as Problem>::Solution;

pub type InputOf<X> = <X as Criterion>::Input;

pub type ObjectiveUnitOf<X> =
    <<<X as Criterion>::Problem as Problem>::ObjectiveValue as ObjectiveValue>::Unit;

type MoveOf<X> = <<X as Criterion>::Problem as Problem>::Move;

type ObjectiveValueOf<X> = <<X as Criterion>::Problem as Problem>::ObjectiveValue;

pub(super) type CandidateMoveOf<X> = CandidateMove<MoveOf<X>, ObjectiveValueOf<X>>;
