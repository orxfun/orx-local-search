use crate::{criterion::Criterion, moves::Moves, neighborhood::Neighborhood, problem::Problem};
use core::marker::PhantomData;

pub struct MX<'i, M, X>(PhantomData<&'i (M, X)>)
where
    M: Moves<'i, X = X>,
    X: Criterion<Problem = <M::Neighborhood as Neighborhood>::Problem>;
