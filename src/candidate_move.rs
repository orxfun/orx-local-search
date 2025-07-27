use crate::{Move, objective::Objective};
use std::cmp::Ordering;

pub struct CandidateMove<M, O>
where
    M: Move,
    O: Objective,
{
    pub r#move: M,
    pub objective_value: O::Unit,
}

impl<M, O> CandidateMove<M, O>
where
    M: Move,
    O: Objective,
{
    pub fn new(r#move: M, objective_value: O::Unit) -> Self {
        Self {
            r#move,
            objective_value,
        }
    }

    pub fn compose(self, other: CandidateMove<M, O>) -> Self {
        debug_assert_eq!(&self.r#move, &other.r#move);
        let objective_value = O::reduce(self.objective_value, other.objective_value);
        Self::new(self.r#move, objective_value)
    }
}

impl<M: Move, O: Objective> PartialEq for CandidateMove<M, O> {
    fn eq(&self, other: &Self) -> bool {
        self.r#move == other.r#move
    }
}

impl<M: Move, O: Objective> Eq for CandidateMove<M, O> {}

impl<M: Move, O: Objective> PartialOrd for CandidateMove<M, O> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.r#move.partial_cmp(&other.r#move)
    }
}

impl<M: Move, O: Objective> Ord for CandidateMove<M, O> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.r#move.cmp(&other.r#move)
    }
}
