use crate::{
    composition::{
        EmptyInputs,
        inputs::{InputsQueue, PairOfInputs},
    },
    criterion::Criterion,
    eval_soln::EvalSoln,
    objective::Objective,
    problem::Problem,
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
        = SingleCrit<P, X>
    where
        X: Criterion<P>;

    type Front = Self;

    type Back = Self;

    type Input<'i> = EmptyInputs;

    fn evaluate(_: &Self::Input<'_>, _: &<P as Problem>::Solution) -> EvalSoln<P> {
        EvalSoln::Feasible(<P::Objective as Objective>::identity())
    }
}

// single

pub struct SingleCrit<P, F>(PhantomData<(P, F)>)
where
    P: Problem,
    F: Criterion<P>;

impl<P, F> Criteria<P> for SingleCrit<P, F>
where
    P: Problem,
    F: Criterion<P>,
{
    type PushBack<'i, X>
        = PairOfCrit<P, F, SingleCrit<P, X>>
    where
        X: Criterion<P>;

    type Front = F;

    type Back = Self;

    type Input<'i> = SingleInput<F::Input<'i>>;

    fn evaluate(input: &Self::Input<'_>, solution: &P::Solution) -> EvalSoln<P> {
        F::evaluate(input.front(), solution)
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
        let (in1, in2) = input.front_back();
        let eval1 = F::evaluate(in1, solution);
        let eval2 = B::evaluate(in2, solution);
        match (eval1, eval2) {
            (EvalSoln::Feasible(val1), EvalSoln::Feasible(val2)) => {
                let val = <P::Objective as Objective>::compose(val1, val2);
                EvalSoln::Feasible(val)
            }
            _ => EvalSoln::Infeasible,
        }
    }
}
