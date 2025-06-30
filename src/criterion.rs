use crate::{candidate_move::CandidateMove, move_generator::MoveGenerator, problem::Problem};

pub trait Criterion {
    type Problem: Problem;

    type Input<'a>;

    type MoveGenerator: MoveGenerator<X = Self>;

    fn move_generator(&self) -> Self::MoveGenerator;
}

pub type CandidateMoveOf<X> = CandidateMove<<X as Criterion>::Problem>;

pub type ObjectOf<X> = <<X as Criterion>::Problem as Problem>::Object;

pub type InputOf<'a, X> = <X as Criterion>::Input<'a>;
