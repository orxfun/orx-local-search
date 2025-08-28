use crate::{MoveGenerator, Neighborhood, Problem};
use core::marker::PhantomData;

pub struct MoveGeneratorEmpty<'i, P, N>(PhantomData<&'i (P, N)>)
where
    P: Problem,
    N: Neighborhood<Problem = P>;

impl<'i, P, N> Default for MoveGeneratorEmpty<'i, P, N>
where
    P: Problem,
    N: Neighborhood<Problem = P>,
{
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<'i, P, N> MoveGenerator<'i> for MoveGeneratorEmpty<'i, P, N>
where
    P: Problem,
    N: Neighborhood<Problem = P>,
{
    type Problem = P;

    type Neighborhood = N;

    type Input = ();

    fn moves<'a>(
        &'a mut self,
        _: Self::Input,
        _: &'a <Self::Problem as Problem>::Solution,
    ) -> impl Iterator<Item = crate::EvalMove<Self::Neighborhood>> + 'a
    where
        'i: 'a,
    {
        core::iter::empty()
    }
}
