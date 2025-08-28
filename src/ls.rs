use crate::{neighborhood::Neighborhood, problem::Problem};
use core::marker::PhantomData;

pub struct LocalSearch<N>(PhantomData<N>)
where
    N: Neighborhood;
