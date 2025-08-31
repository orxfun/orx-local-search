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

pub trait CriteriaQueue: Criteria {
    type PushBack<X: Criterion<Problem = Self::Problem>>: CriteriaQueue<Problem = Self::Problem>;

    type Front: Criterion<Problem = Self::Problem>;

    type Back: CriteriaQueue;
}

// single

pub struct SingleCriterion<F>(PhantomData<F>)
where
    F: Criterion;

impl<F> CriteriaQueue for SingleCriterion<F>
where
    F: Criterion,
{
    type PushBack<X: Criterion<Problem = Self::Problem>> = PairOfCriteria<F, SingleCriterion<X>>;

    type Front = F;

    type Back = Self;
}

impl<F> Criteria for SingleCriterion<F>
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

pub struct PairOfCriteria<F, B>(PhantomData<(F, B)>)
where
    F: Criterion,
    B: CriteriaQueue + Criteria<Problem = F::Problem>;

impl<F, B> CriteriaQueue for PairOfCriteria<F, B>
where
    F: Criterion,
    B: CriteriaQueue + Criteria<Problem = F::Problem>,
{
    type PushBack<X: Criterion<Problem = Self::Problem>> = PairOfCriteria<F, B::PushBack<X>>;

    type Front = F;

    type Back = B;
}

impl<F, B> Criteria for PairOfCriteria<F, B>
where
    F: Criterion,
    B: CriteriaQueue + Criteria<Problem = F::Problem>,
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
