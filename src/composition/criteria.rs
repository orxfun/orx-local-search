use crate::{
    composition::inputs::{InputsQueue, NonEmptyInputsQueue, PairOfInputs, SingleInput},
    criterion::Criterion,
    eval_soln::EvalSoln,
    objective::Objective,
    problem::Problem,
};
use core::marker::PhantomData;

// traits

pub trait Criteria {
    // queue

    type PushBack<'i, X>: Criteria<
            Problem = Self::Problem,
            Input<'i> = <Self::Input<'i> as InputsQueue<'i>>::PushBack<X::Input<'i>>,
        >
    where
        X: Criterion<Problem = Self::Problem>;

    type Front: Criterion<Problem = Self::Problem>;

    type Back: Criteria<Problem = Self::Problem>;

    // criterion

    type Problem: Problem;

    type Input<'i>: InputsQueue<'i>;

    fn evaluate(
        input: &Self::Input<'_>,
        solution: &<Self::Problem as Problem>::Solution,
    ) -> EvalSoln<Self::Problem>;
}

// single

pub struct SingleCrit<F>(PhantomData<F>)
where
    F: Criterion;

impl<F> Criteria for SingleCrit<F>
where
    F: Criterion,
{
    type PushBack<'i, X>
        = PairOfCrit<F, SingleCrit<X>>
    where
        X: Criterion<Problem = Self::Problem>;

    type Front = F;

    type Back = Self;

    type Problem = F::Problem;

    type Input<'i> = SingleInput<'i, F::Input<'i>>;

    fn evaluate(
        input: &Self::Input<'_>,
        solution: &<Self::Problem as Problem>::Solution,
    ) -> EvalSoln<Self::Problem> {
        F::evaluate(input.front(), solution)
    }
}

// pair

pub struct PairOfCrit<F, B>(PhantomData<(F, B)>)
where
    F: Criterion,
    B: Criteria<Problem = F::Problem>;

impl<F, B> Criteria for PairOfCrit<F, B>
where
    F: Criterion,
    B: Criteria<Problem = F::Problem>,
{
    type PushBack<'i, X>
        = PairOfCrit<F, B::PushBack<'i, X>>
    where
        X: Criterion<Problem = Self::Problem>;

    type Front = F;

    type Back = B;

    type Problem = F::Problem;

    type Input<'i> = PairOfInputs<'i, F::Input<'i>, B::Input<'i>>;

    fn evaluate(
        input: &Self::Input<'_>,
        solution: &<Self::Problem as Problem>::Solution,
    ) -> EvalSoln<Self::Problem> {
        let (in1, in2) = input.front_back();
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
