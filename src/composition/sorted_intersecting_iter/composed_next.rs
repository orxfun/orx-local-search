use crate::{EvalMove, Neighborhood, Objective, Problem};
use core::cmp::Ordering::*;

pub enum ComposedNext<P, N>
where
    P: Problem,
    N: Neighborhood<P>,
{
    OneIteratorConsumed,
    BothYieldedSameValue { composed: EvalMove<P, N> },
    FirstIteratorYieldedSmaller { next2: EvalMove<P, N> },
    FirstIteratorYieldedGreater { next1: EvalMove<P, N> },
}

type Obj<P> = <<P as Problem>::Objective as Objective>::Unit;

fn compose<P: Problem>(val1: Obj<P>, val2: Obj<P>) -> Obj<P> {
    <P::Objective as Objective>::compose(val1, val2)
}

impl<P, N> ComposedNext<P, N>
where
    P: Problem,
    N: Neighborhood<P>,
{
    pub fn new(next1: Option<EvalMove<P, N>>, next2: Option<EvalMove<P, N>>) -> Self {
        match (next1, next2) {
            (Some(mv1), Some(mv2)) => match mv1.mv.cmp(&mv2.mv) {
                Equal => {
                    let (mv, val1, val2) = (mv1.mv, mv1.value, mv2.value);
                    let objective_value = compose::<P>(val1, val2);
                    let composed = EvalMove::new(mv, objective_value);
                    Self::BothYieldedSameValue { composed }
                }
                Greater => Self::FirstIteratorYieldedGreater { next1: mv1 },
                Less => Self::FirstIteratorYieldedSmaller { next2: mv2 },
            },
            _ => Self::OneIteratorConsumed,
        }
    }

    pub fn new_with_left(mv1: EvalMove<P, N>, next2: Option<EvalMove<P, N>>) -> Self {
        match next2 {
            Some(mv2) => match mv1.mv.cmp(&mv2.mv) {
                Equal => {
                    let (mv, val1, val2) = (mv1.mv, mv1.value, mv2.value);
                    let objective_value = compose::<P>(val1, val2);
                    let composed = EvalMove::new(mv, objective_value);
                    Self::BothYieldedSameValue { composed }
                }
                Greater => Self::FirstIteratorYieldedGreater { next1: mv1 },
                Less => Self::FirstIteratorYieldedSmaller { next2: mv2 },
            },
            _ => Self::OneIteratorConsumed,
        }
    }

    pub fn new_with_right(next1: Option<EvalMove<P, N>>, mv2: EvalMove<P, N>) -> Self {
        match next1 {
            Some(mv1) => match mv1.mv.cmp(&mv2.mv) {
                Equal => {
                    let (mv, val1, val2) = (mv1.mv, mv1.value, mv2.value);
                    let objective_value = compose::<P>(val1, val2);
                    let composed = EvalMove::new(mv, objective_value);
                    Self::BothYieldedSameValue { composed }
                }
                Greater => Self::FirstIteratorYieldedGreater { next1: mv1 },
                Less => Self::FirstIteratorYieldedSmaller { next2: mv2 },
            },
            _ => Self::OneIteratorConsumed,
        }
    }
}
