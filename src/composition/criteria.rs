use crate::{
    composition::{InputsQueue, MultiInputsQueue, PairOfInputs, SingleInput},
    criterion::Criterion,
    eval_soln::EvalSoln,
    objective::Objective,
    problem::Problem,
};
use core::marker::PhantomData;

// traits

pub trait Criteria {
    type Problem: Problem;

    type Input<'i>: InputsQueue<'i>;

    fn evaluate(
        input: Self::Input<'_>,
        solution: &<Self::Problem as Problem>::Solution,
    ) -> EvalSoln<Self::Problem>;
}

pub trait Queue: Criteria {
    type PushBack<X: Criterion<Problem = Self::Problem>>: Queue<Problem = Self::Problem>;

    type Front: Criterion;

    type Back: Queue;

    fn len() -> usize;
}

// single

pub struct Single<F>(PhantomData<F>)
where
    F: Criterion;

impl<F> Queue for Single<F>
where
    F: Criterion,
{
    type PushBack<X: Criterion<Problem = Self::Problem>> = Pair<F, Single<X>>;

    type Front = F;

    type Back = Self;

    fn len() -> usize {
        1
    }
}

impl<F> Criteria for Single<F>
where
    F: Criterion,
{
    type Problem = F::Problem;

    type Input<'i> = SingleInput<'i, F::Input<'i>>;

    fn evaluate(
        input: Self::Input<'_>,
        solution: &<Self::Problem as Problem>::Solution,
    ) -> EvalSoln<Self::Problem> {
        F::evaluate(input.into_front(), solution)
    }
}

// pair

pub struct Pair<F, B>(PhantomData<(F, B)>)
where
    F: Criterion,
    B: Queue + Criteria<Problem = F::Problem>;

impl<F, B> Queue for Pair<F, B>
where
    F: Criterion,
    B: Queue + Criteria<Problem = F::Problem>,
{
    type PushBack<X: Criterion<Problem = Self::Problem>> = Pair<F, B::PushBack<X>>;

    type Front = F;

    type Back = B;

    fn len() -> usize {
        1 + B::len()
    }
}

impl<F, B> Criteria for Pair<F, B>
where
    F: Criterion,
    B: Queue + Criteria<Problem = F::Problem>,
{
    type Problem = F::Problem;

    type Input<'i> = PairOfInputs<'i, F::Input<'i>, B::Input<'i>>;

    fn evaluate(
        input: Self::Input<'_>,
        solution: &<Self::Problem as Problem>::Solution,
    ) -> EvalSoln<Self::Problem> {
        let (in1, in2) = input.pop_front();
        let eval1 = F::evaluate(in1, solution);
        let eval2 = B::evaluate(in2, solution);
        match (eval1, eval2) {
            (EvalSoln::Feasible(val1), EvalSoln::Feasible(val2)) => {
                let val = <<F::Problem as Problem>::Objective as Objective>::compose(val1, val2);
                EvalSoln::Feasible(val)
            }
            _ => EvalSoln::Infeasible,
        }
    }
}
