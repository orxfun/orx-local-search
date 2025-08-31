use crate::{
    eval_move::EvalMove, neighborhood::Neighborhood, objective::Objective, problem::Problem,
};
use core::cmp::Ordering;

pub enum ComposedNext<N>
where
    N: Neighborhood,
{
    OneIteratorConsumed,
    BothYieldedSameValue { composed: EvalMove<N> },
    FirstIteratorYieldedSmaller { next2: EvalMove<N> },
    FirstIteratorYieldedGreater { next1: EvalMove<N> },
}

fn compose<N: Neighborhood>(
    mv1: &EvalMove<N>,
    mv2: &EvalMove<N>,
) -> <<N::Problem as Problem>::Objective as Objective>::Unit {
    <<N::Problem as Problem>::Objective as Objective>::compose(mv1.value, mv2.value)
}

impl<N> ComposedNext<N>
where
    N: Neighborhood,
{
    pub fn new(next1: Option<EvalMove<N>>, next2: Option<EvalMove<N>>) -> Self {
        match (next1, next2) {
            (Some(mv1), Some(mv2)) => match mv1.mv.cmp(&mv2.mv) {
                Ordering::Equal => {
                    let objective_value = compose(&mv1, &mv2);
                    let composed = EvalMove::new(mv1.mv, objective_value);
                    Self::BothYieldedSameValue { composed }
                }
                Ordering::Greater => Self::FirstIteratorYieldedGreater { next1: mv1 },
                Ordering::Less => Self::FirstIteratorYieldedSmaller { next2: mv2 },
            },
            _ => Self::OneIteratorConsumed,
        }
    }

    pub fn new_with_left(mv1: EvalMove<N>, next2: Option<EvalMove<N>>) -> Self {
        match next2 {
            Some(mv2) => match mv1.mv.cmp(&mv2.mv) {
                Ordering::Equal => {
                    let objective_value = compose(&mv1, &mv2);
                    let composed = EvalMove::new(mv1.mv, objective_value);
                    Self::BothYieldedSameValue { composed }
                }
                Ordering::Greater => Self::FirstIteratorYieldedGreater { next1: mv1 },
                Ordering::Less => Self::FirstIteratorYieldedSmaller { next2: mv2 },
            },
            _ => Self::OneIteratorConsumed,
        }
    }

    pub fn new_with_right(next1: Option<EvalMove<N>>, mv2: EvalMove<N>) -> Self {
        match next1 {
            Some(mv1) => match mv1.mv.cmp(&mv2.mv) {
                Ordering::Equal => {
                    let objective_value = compose(&mv1, &mv2);
                    let composed = EvalMove::new(mv1.mv, objective_value);
                    Self::BothYieldedSameValue { composed }
                }
                Ordering::Greater => Self::FirstIteratorYieldedGreater { next1: mv1 },
                Ordering::Less => Self::FirstIteratorYieldedSmaller { next2: mv2 },
            },
            _ => Self::OneIteratorConsumed,
        }
    }
}
