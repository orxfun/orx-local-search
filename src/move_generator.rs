use crate::{cand_move::CandidateMove, r#move::Move, obj::Objective};

pub trait MoveGenerator {
    type Input;

    type Solution;

    type Move: Move;

    type Objective: Objective;

    fn new() -> Self;

    fn moves<'a>(
        &'a mut self,
        solution: &'a Self::Solution,
        input: &'a Self::Input,
    ) -> impl Iterator<Item = CandidateMove<Self::Move, Self::Objective>> + 'a;
}
