use crate::{crit::Criterion, move_gen::MoveGenerator, neighborhood::Neighborhood};
use core::marker::PhantomData;

pub struct LocalSearch<'i, N, X, M>(PhantomData<&'i (N, X, M)>)
where
    N: Neighborhood,
    X: Criterion<Problem = N::Problem>,
    M: MoveGenerator<'i, Neighborhood = N, X = X>;
