use crate::{
    criterion::Criterion, eval_move::EvalMove, neighborhood::Neighborhood, problem::Problem,
};

pub trait Moves<'i, P>: Default
where
    P: Problem,
{
    type Neighborhood: Neighborhood<P>;

    type X: Criterion<P>;

    fn moves<'a>(
        &'a mut self,
        input: &'i <Self::X as Criterion<P>>::Input<'i>,
        solution: &'a P::Solution,
    ) -> impl Iterator<Item = EvalMove<P, Self::Neighborhood>> + 'a
    where
        'i: 'a;
}
