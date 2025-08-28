use crate::{Criterion, Neighborhood, Problem, crit_empty::EmptyCriterion};

pub struct Ls<P, N, X>
where
    P: Problem,
    N: Neighborhood<Problem = P>,
    X: Criterion,
{
    problem: P,
    neighborhood: N,
    criterion: X,
}

impl<P, N> Ls<P, N, EmptyCriterion<P>>
where
    P: Problem,
    N: Neighborhood<Problem = P>,
{
    pub fn new() -> Self {
        Self {
            problem: Default::default(),
            neighborhood: Default::default(),
            criterion: Default::default(),
        }
    }
}

impl<P, N, X> Ls<P, N, X>
where
    P: Problem,
    N: Neighborhood<Problem = P>,
    X: Criterion,
{
}
