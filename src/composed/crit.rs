use crate::{criteria::Criteria, eval_soln::EvalSoln, objective::Objective, problem::Problem};
use core::marker::PhantomData;
use orx_meta::queue::{NonEmptyQueue, Pair};

#[derive(Default, Clone, Copy)]
pub struct ComposedCriteria<X1, X2>(PhantomData<(X1, X2)>)
where
    X1: Criteria,
    X2: Criteria<Problem = X1::Problem>;

impl<X1, X2> Criteria for ComposedCriteria<X1, X2>
where
    X1: Criteria,
    X2: Criteria<Problem = X1::Problem>,
{
    type Problem = X1::Problem;

    // type Input<'i> = <X1::Input<'i> as Queue>::PushBack<X2::Input<'i>>;

    type Input<'i> = Pair<X1::Input<'i>, X2::Input<'i>>;

    type Compose<Y>
        = ComposedCriteria<X1, X2::Compose<Y>>
    where
        Y: Criteria<Problem = Self::Problem>;

    fn evaluate(
        input: Self::Input<'_>,
        solution: &<Self::Problem as Problem>::Solution,
    ) -> EvalSoln<Self::Problem> {
        let (input1, input2) = input.pop_front();
        let val1 = X1::evaluate(input1, solution);
        let val2 = X2::evaluate(input2, solution);
        match (val1, val2) {
            (EvalSoln::Feasible(a), EvalSoln::Feasible(b)) => EvalSoln::Feasible(
                <<X1::Problem as Problem>::Objective as Objective>::compose(a, b),
            ),
            _ => EvalSoln::Infeasible,
        }
    }
}
