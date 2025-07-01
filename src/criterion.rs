use crate::{
    Move, ObjectiveValue, candidate_move::CandidateMove, move_generator::MoveGenerator,
    problem::Problem,
};

pub trait Criterion {
    type Problem: Problem;

    type Input;

    type MoveGenerator: MoveGenerator<X = Self>;

    fn move_generator() -> Self::MoveGenerator;

    fn evaluate(solution: &SolutionOf<Self>, input: &Self::Input) -> Option<ObjectiveUnitOf<Self>>;
}

pub type CandidateMoveOf<X> = CandidateMove<<X as Criterion>::Problem>;

pub type SolutionOf<X> = <<<X as Criterion>::Problem as Problem>::Move as Move>::Solution;

pub type InputOf<X> = <X as Criterion>::Input;

pub type ObjectiveUnitOf<X> =
    <<<X as Criterion>::Problem as Problem>::ObjectiveValue as ObjectiveValue>::Unit;
