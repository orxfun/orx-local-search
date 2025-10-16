use crate::{
    Criterion, EvalSoln, Objective, Problem,
    composition::inputs::{EmptyInputs, InputsQueue, PairOfInputs},
};
use core::marker::PhantomData;

// traits

pub trait Criteria<P>
where
    P: Problem,
{
    // queue

    type PushBack<'i, X>: Criteria<P, Input<'i> = <Self::Input<'i> as InputsQueue>::PushBack<X::Input<'i>>>
    where
        X: Criterion<P>;

    type Front: Criterion<P>;

    type Back: Criteria<P>;

    // criterion

    type Input<'i>: InputsQueue;

    fn evaluate(input: &Self::Input<'_>, solution: &P::Solution) -> EvalSoln<P>;
}

// empty

pub struct EmptyCriterion;

impl<P> Criterion<P> for EmptyCriterion
where
    P: Problem,
{
    type Input<'i> = ();

    fn evaluate(_: &Self::Input<'_>, _: &<P as Problem>::Solution) -> EvalSoln<P> {
        EvalSoln::Feasible(<P::Objective as Objective>::identity())
    }
}
impl<P> Criteria<P> for EmptyCriterion
where
    P: Problem,
{
    type PushBack<'i, X>
        = PairOfCrit<P, X, Self>
    where
        X: Criterion<P>;

    type Front = Self;

    type Back = Self;

    type Input<'i> = EmptyInputs;

    fn evaluate(_: &Self::Input<'_>, _: &<P as Problem>::Solution) -> EvalSoln<P> {
        EvalSoln::Feasible(<P::Objective as Objective>::identity())
    }
}

// pair

pub struct PairOfCrit<P, F, B>(PhantomData<(P, F, B)>)
where
    P: Problem,
    F: Criterion<P>,
    B: Criteria<P>;

impl<P, F, B> Criteria<P> for PairOfCrit<P, F, B>
where
    P: Problem,
    F: Criterion<P>,
    B: Criteria<P>,
{
    type PushBack<'i, X>
        = PairOfCrit<P, F, B::PushBack<'i, X>>
    where
        X: Criterion<P>;

    type Front = F;

    type Back = B;

    type Input<'i> = PairOfInputs<F::Input<'i>, B::Input<'i>>;

    fn evaluate(input: &Self::Input<'_>, solution: &P::Solution) -> EvalSoln<P> {
        let eval1 = F::evaluate(input.front(), solution);
        let eval2 = B::evaluate(input.back(), solution);
        match (eval1, eval2) {
            (EvalSoln::Feasible(val1), EvalSoln::Feasible(val2)) => {
                let val = <P::Objective as Objective>::compose(val1, val2);
                EvalSoln::Feasible(val)
            }
            _ => EvalSoln::Infeasible,
        }
    }
}
