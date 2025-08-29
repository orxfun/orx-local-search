use crate::{
    Criteria, composed::ComposedCriteria, eval_soln::EvalSoln, problem::Problem, symbolic::Symbolic,
};
use orx_meta::queue::{NonEmptyQueue, Single};

pub trait Criterion: Symbolic {
    type Problem: Problem;

    type Input<'i>: Copy;

    fn evaluate(
        input: Self::Input<'_>,
        solution: &<Self::Problem as Problem>::Solution,
    ) -> EvalSoln<Self::Problem>;
}

impl<X> Criteria for X
where
    X: Criterion,
{
    type Problem = <Self as Criterion>::Problem;

    type Input<'i> = Single<<Self as Criterion>::Input<'i>>;

    type Compose<Y>
        = ComposedCriteria<Self, Y>
    where
        Y: Criteria<Problem = Self::Problem>;

    fn evaluate(
        input: Self::Input<'_>,
        solution: &<Self::Problem as Problem>::Solution,
    ) -> EvalSoln<Self::Problem> {
        <Self as Criterion>::evaluate(*input.front(), solution)
    }
}
