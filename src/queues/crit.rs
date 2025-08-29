use crate::{Criterion, EvalSoln, Objective, Problem};
use core::marker::PhantomData;
use orx_meta::queue::*;

pub trait QueueX {
    type Problem: Problem;

    type Front: Criterion<Problem = Self::Problem>;

    type Back: QueueX<Problem = Self::Problem>;

    type PushBack<Y>: QueueX<Problem = Self::Problem>
    where
        Y: Criterion<Problem = Self::Problem>;

    type XInput<'i>: Queue + Copy;

    fn x_evaluate(
        input: Self::XInput<'_>,
        solution: &<Self::Problem as Problem>::Solution,
    ) -> EvalSoln<Self::Problem>;
}

#[derive(Default)]
struct SingleX<F>(PhantomData<F>)
where
    F: Criterion;

impl<F> QueueX for SingleX<F>
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

    fn x_evaluate(
        input: Self::XInput<'_>,
        solution: &<Self::Problem as Problem>::Solution,
    ) -> EvalSoln<Self::Problem> {
        F::evaluate(*input.front(), solution)
    }
}

#[derive(Default)]
struct PairX<F, B>(PhantomData<(F, B)>)
where
    F: Criterion,
    B: QueueX<Problem = F::Problem>;

impl<F, B> QueueX for PairX<F, B>
where
    F: Criterion,
    B: QueueX<Problem = F::Problem>,
{
    type Problem = F::Problem;

    type Front = F;

    type Back = B;

    type PushBack<Y>
        = PairX<F, B::PushBack<Y>>
    where
        Y: Criterion<Problem = Self::Problem>;

    type XInput<'i> = Pair<F::Input<'i>, B::XInput<'i>>;

    fn x_evaluate(
        input: Self::XInput<'_>,
        solution: &<Self::Problem as Problem>::Solution,
    ) -> EvalSoln<Self::Problem> {
        let (in1, in2) = input.pop_front();
        let eval1 = F::evaluate(in1, solution);
        let eval2 = B::x_evaluate(in2, solution);
        match (eval1, eval2) {
            (EvalSoln::Feasible(val1), EvalSoln::Feasible(val2)) => EvalSoln::Feasible(
                <<Self::Problem as Problem>::Objective as Objective>::compose(val1, val2),
            ),
            _ => EvalSoln::Infeasible,
        }
    }
}

// // impl Criterion for all x-queues

// impl<Q> Criterion for Q
// where
//     Q: QueueX + Default + Clone + Copy + 'static,
// {
//     type Problem = <Q as QueueX>::Problem;

//     type Input<'i> = <Q as QueueX>::XInput<'i>;

//     fn evaluate(
//         input: Self::Input<'_>,
//         solution: &<Self::Problem as Problem>::Solution,
//     ) -> EvalSoln<Self::Problem> {
//         <Q as QueueX>::x_evaluate(input, solution)
//     }
// }
