use crate::{
    composition2::criteria::CriterionUp, eval_move::EvalMove, moves::Moves,
    neighborhood::Neighborhood, problem::Problem,
};
use core::marker::PhantomData;
use orx_meta::define_queue;

// traits

pub trait NeighborhoodUp<P>
where
    P: Problem,
{
}

pub trait MovesUp<'i, P, N, X>
where
    P: Problem,
    N: Neighborhood<Problem = P>,
    X: CriterionUp<P>,
{
    fn moves<'a>(
        &'a mut self,
        input: &'i X::Input<'i>,
        solution: &'a <N::Problem as Problem>::Solution,
    ) -> impl Iterator<Item = EvalMove<N>> + 'a
    where
        'i: 'a;
}

define_queue!(
    lifetimes => ['i];
    generics => [P: Problem & N: NeighborhoodUp<P> & X: CriterionUp<P>];
    elements => [];
    names => {
        traits: {
            queue: MovesQueue,
            non_empty_queue: NonEmptyMovesQueue,
        },
        structs: {
            empty: EmptyMoves,
            single: SingleMove,
            pair: PairOfMoves,
            composition: MoveComposition,
            builder: MoveBuilder,
        },
    };
);
