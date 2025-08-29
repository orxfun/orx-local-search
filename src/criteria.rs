use crate::{Criterion, EvalSoln, Objective, Problem};
use core::marker::PhantomData;
use orx_meta::queue::*;

pub trait Criteria {
    type Problem: Problem;

    type Front: Criterion<Problem = Self::Problem>;

    type Back: Criteria<Problem = Self::Problem>;

    type PushBack<Y>: Criteria<Problem = Self::Problem>
    where
        Y: Criterion<Problem = Self::Problem>;

    type XInput<'i>: Queue + Copy;

    fn evaluate(
        input: Self::XInput<'_>,
        solution: &<Self::Problem as Problem>::Solution,
    ) -> EvalSoln<Self::Problem>;
}

#[derive(Default)]
pub struct SingleX<F>(PhantomData<F>)
where
    F: Criterion;

impl<F> Criteria for SingleX<F>
where
    F: Criterion,
{
    type Problem = F::Problem;

    type Front = F;

    type Back = Self;

    type PushBack<Y>
        = Self
    where
        Y: Criterion<Problem = Self::Problem>;

    type XInput<'i> = Single<F::Input<'i>>;

    fn evaluate(
        input: Self::XInput<'_>,
        solution: &<Self::Problem as Problem>::Solution,
    ) -> EvalSoln<Self::Problem> {
        F::evaluate(*input.front(), solution)
    }
}

#[derive(Default)]
pub struct PairX<F, B>(PhantomData<(F, B)>)
where
    F: Criterion,
    B: Criteria<Problem = F::Problem>;

impl<F, B> Criteria for PairX<F, B>
where
    F: Criterion,
    B: Criteria<Problem = F::Problem>,
{
    type Problem = F::Problem;

    type Front = F;

    type Back = B;

    type PushBack<Y>
        = PairX<F, B::PushBack<Y>>
    where
        Y: Criterion<Problem = Self::Problem>;

    type XInput<'i> = Pair<F::Input<'i>, B::XInput<'i>>;

    fn evaluate(
        input: Self::XInput<'_>,
        solution: &<Self::Problem as Problem>::Solution,
    ) -> EvalSoln<Self::Problem> {
        let (in1, in2) = input.pop_front();
        let eval1 = F::evaluate(in1, solution);
        let eval2 = B::evaluate(in2, solution);
        match (eval1, eval2) {
            (EvalSoln::Feasible(val1), EvalSoln::Feasible(val2)) => EvalSoln::Feasible(
                <<Self::Problem as Problem>::Objective as Objective>::compose(val1, val2),
            ),
            _ => EvalSoln::Infeasible,
        }
    }
}
