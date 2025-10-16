use crate::{
    criteria::CriteriaQueue, criterion::Criterion, eval_move::EvalMove, neighborhood::Neighborhood,
    problem::Problem,
};

pub trait Moves<'i, P, N>: Default
where
    P: Problem,
    N: Neighborhood<P>,
{
    type X: CriteriaQueue<P>;

    fn moves<'a>(
        &'a mut self,
        input: &'i <Self::X as Criterion<P>>::Input<'i>,
        solution: &'a P::Solution,
    ) -> impl Iterator<Item = EvalMove<P, N>> + 'a
    where
        'i: 'a;
}
