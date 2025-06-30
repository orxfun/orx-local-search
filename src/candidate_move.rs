use crate::{objective_value::ObjectiveValue, problem::Problem};
use std::cmp::Ordering;

pub struct CandidateMove<P>
where
    P: Problem,
{
    pub r#move: P::Move,
    pub objective_value: <P::ObjectiveValue as ObjectiveValue>::Unit,
}

impl<P> CandidateMove<P>
where
    P: Problem,
{
    pub fn new(
        r#move: P::Move,
        objective_value: <P::ObjectiveValue as ObjectiveValue>::Unit,
    ) -> Self {
        Self {
            r#move,
            objective_value,
        }
    }

    pub fn compose(self, other: CandidateMove<P>) -> Self {
        debug_assert_eq!(&self.r#move, &other.r#move);
        let objective_value = <P::ObjectiveValue as ObjectiveValue>::reduce(
            self.objective_value,
            other.objective_value,
        );
        Self::new(self.r#move, objective_value)
    }
}

impl<P: Problem> PartialEq for CandidateMove<P> {
    fn eq(&self, other: &Self) -> bool {
        self.r#move == other.r#move
    }
}

impl<P: Problem> Eq for CandidateMove<P> {}

impl<P: Problem> PartialOrd for CandidateMove<P> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.r#move.partial_cmp(&other.r#move)
    }
}

impl<P: Problem> Ord for CandidateMove<P> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.r#move.cmp(&other.r#move)
    }
}
