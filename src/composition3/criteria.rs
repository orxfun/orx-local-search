use crate::{
    Criterion, Objective,
    composition3::inputs::{
        EmptyInputs, InputsQueue, NonEmptyInputsQueue, PairOfInputs, SingleInput,
    },
    eval_soln::EvalSoln,
    problem::Problem,
};
use core::marker::PhantomData;
use orx_meta::define_queue;

pub struct CriterionOf<P, C>
where
    P: Problem,
    C: Criterion<P>,
{
    c: C,
    phantom: PhantomData<P>,
}
impl<P, C> From<C> for CriterionOf<P, C>
where
    P: Problem,
    C: Criterion<P>,
{
    fn from(c: C) -> Self {
        Self {
            c,
            phantom: PhantomData,
        }
    }
}
impl<P, C> CriterionUp<P> for CriterionOf<P, C>
where
    P: Problem,
    C: Criterion<P>,
{
    type Input<'i> = SingleInput<C::Input<'i>>;

    fn evaluate(input: &Self::Input<'_>, solution: &<P as Problem>::Solution) -> EvalSoln<P> {
        C::evaluate(input.front(), solution)
    }
}

pub trait CriterionUp<P>
where
    P: Problem,
{
    type Input<'i>: InputsQueue;

    fn evaluate(input: &Self::Input<'_>, solution: &P::Solution) -> EvalSoln<P>;
}

define_queue!(
    lifetimes => [];
    generics => [P: Problem];
    elements => [CriterionUp<P>];
    names => {
        traits: {
            queue: CriteriaQueue,
            non_empty_queue: NonEmptyCriteriaQueue,
        },
        structs: {
            empty: EmptyCriteria,
            single: SingleCriterion,
            pair: PairOfCriteria,
            composition: CriteriaComposition,
            builder: CriteriaBuilder,
        },
    };
);

impl<P: Problem> CriterionUp<P> for EmptyCriteria<P> {
    type Input<'i> = EmptyInputs;
    fn evaluate(_: &Self::Input<'_>, _: &<P as Problem>::Solution) -> EvalSoln<P> {
        EvalSoln::Infeasible
    }
}

impl<P: Problem, F: CriterionUp<P>> CriterionUp<P> for SingleCriterion<F, P> {
    type Input<'i> = SingleInput<F::Input<'i>>;
    fn evaluate(input: &Self::Input<'_>, solution: &<P as Problem>::Solution) -> EvalSoln<P> {
        F::evaluate(input.front(), solution)
    }
}

impl<P: Problem, F: CriterionUp<P>, B: CriteriaQueue<P>> CriterionUp<P>
    for PairOfCriteria<F, B, P>
{
    type Input<'i> = PairOfInputs<F::Input<'i>, B::Input<'i>>;
    fn evaluate(input: &Self::Input<'_>, solution: &<P as Problem>::Solution) -> EvalSoln<P> {
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
