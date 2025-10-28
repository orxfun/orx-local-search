use crate::composition::inputs::{Inputs, InputsQueue, SingleInput};
use crate::core::{Criterion, EvalSoln, Objective, Problem};
use core::marker::PhantomData;

// traits

pub trait CriteriaQueue<P>
where
    P: Problem,
{
    // queue

    type PushBack<'i, X>: CriteriaQueue<P, Input<'i> = <Self::Input<'i> as InputsQueue>::PushBack<X::Input<'i>>>
    where
        X: Criterion<P>;

    type Front: Criterion<P>;

    type Back: CriteriaQueue<P>;

    fn push<'i, X>(self, criterion: X) -> Self::PushBack<'i, X>
    where
        X: Criterion<P>;

    // criterion

    type Input<'i>: InputsQueue;

    fn evaluate(&self, input: &Self::Input<'_>, solution: &P::Solution) -> EvalSoln<P>;
}

// single

pub struct CriteriaSingle<P, F>
where
    P: Problem,
    F: Criterion<P>,
{
    front: F,
    phantom: PhantomData<P>,
}

impl<P, F> CriteriaQueue<P> for CriteriaSingle<P, F>
where
    P: Problem,
    F: Criterion<P>,
{
    type PushBack<'i, X>
        = Criteria<P, F, CriteriaSingle<P, X>>
    where
        X: Criterion<P>;

    type Front = F;

    type Back = Self;

    fn push<'i, X>(self, criterion: X) -> Self::PushBack<'i, X>
    where
        X: Criterion<P>,
    {
        Criteria {
            front: self.front,
            back: CriteriaSingle {
                front: criterion,
                phantom: PhantomData,
            },
            phantom: PhantomData,
        }
    }

    type Input<'i> = SingleInput<F::Input<'i>>;

    fn evaluate(
        &self,
        input: &Self::Input<'_>,
        solution: &<P as Problem>::Solution,
    ) -> EvalSoln<P> {
        self.front.evaluate(input.front(), solution)
    }
}

// multi

pub struct Criteria<P, F, B>
where
    P: Problem,
    F: Criterion<P>,
    B: CriteriaQueue<P>,
{
    front: F,
    back: B,
    phantom: PhantomData<P>,
}

impl<P, F, B> CriteriaQueue<P> for Criteria<P, F, B>
where
    P: Problem,
    F: Criterion<P>,
    B: CriteriaQueue<P>,
{
    type PushBack<'i, X>
        = Criteria<P, F, B::PushBack<'i, X>>
    where
        X: Criterion<P>;

    type Front = F;

    type Back = B;

    fn push<'i, X>(self, criterion: X) -> Self::PushBack<'i, X>
    where
        X: Criterion<P>,
    {
        Criteria {
            front: self.front,
            back: self.back.push(criterion),
            phantom: PhantomData,
        }
    }

    type Input<'i> = Inputs<F::Input<'i>, B::Input<'i>>;

    fn evaluate(
        &self,
        input: &Self::Input<'_>,
        solution: &<P as Problem>::Solution,
    ) -> EvalSoln<P> {
        let eval1 = self.front.evaluate(input.front(), solution);
        let eval2 = self.back.evaluate(input.back(), solution);
        match (eval1, eval2) {
            (EvalSoln::Feasible(val1), EvalSoln::Feasible(val2)) => {
                let val = <P::Objective as Objective>::compose(val1, val2);
                EvalSoln::Feasible(val)
            }
            _ => EvalSoln::Infeasible,
        }
    }
}
