use crate::{
    composition3::criteria::CriterionUp, criterion::Criterion, eval_move::EvalMove,
    neighborhood::Neighborhood, problem::Problem,
};

pub trait MovesUp<'i, P, N, X>: Default
where
    P: Problem,
    N: Neighborhood<P>,
    X: CriterionUp<P>,
{
    fn moves<'a>(
        &'a mut self,
        input: &'i X::Input<'i>,
        solution: &'a P::Solution,
    ) -> impl Iterator<Item = EvalMove<P, N>> + 'a
    where
        'i: 'a;
}
