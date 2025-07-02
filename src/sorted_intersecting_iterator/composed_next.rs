use crate::{CandidateMove, Move, ObjectiveValue};
use std::cmp::Ordering;

pub enum ComposedNext<M, O>
where
    M: Move,
    O: ObjectiveValue,
{
    OneIteratorConsumed,
    BothYieldedSameValue(Option<CandidateMove<M, O>>),
    FirstIteratorYieldedSmaller { next2: Option<CandidateMove<M, O>> },
    FirstIteratorYieldedGreater { next1: Option<CandidateMove<M, O>> },
}

impl<M, O> ComposedNext<M, O>
where
    M: Move,
    O: ObjectiveValue,
{
    pub fn new(next1: Option<CandidateMove<M, O>>, next2: Option<CandidateMove<M, O>>) -> Self {
        match (next1, next2) {
            (Some(value1), Some(value2)) => match value1.cmp(&value2) {
                Ordering::Equal => {
                    let objective_value = O::reduce(value1.objective_value, value2.objective_value);
                    let composed_next = CandidateMove::new(value1.r#move, objective_value);
                    Self::BothYieldedSameValue(Some(composed_next))
                }
                Ordering::Greater => Self::FirstIteratorYieldedGreater {
                    next1: Some(value1),
                },
                Ordering::Less => Self::FirstIteratorYieldedSmaller {
                    next2: Some(value2),
                },
            },
            _ => Self::OneIteratorConsumed,
        }
    }
}
