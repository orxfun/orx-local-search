use crate::{Criterion, Objective, Problem};
use orx_meta::queue::{NonEmptyQueue, Pair};

#[derive(Default, Clone, Copy)]
pub struct ComposedCriteria<X1, X2>(X1, X2)
where
    X1: Criterion,
    X2: Criterion<Problem = X1::Problem>;

impl<X1, X2> Criterion for ComposedCriteria<X1, X2>
where
    X1: Criterion,
    X2: Criterion<Problem = X1::Problem>,
{
    type Problem = X1::Problem;

    type Input<'i> = Pair<X1::Input<'i>, X2::Input<'i>>;

    fn evaluate(
        self,
        input: Self::Input<'_>,
        solution: &<Self::Problem as Problem>::Solution,
    ) -> Option<<Self::Problem as Problem>::ObjectiveUnit> {
        let (input0, input1) = input.pop_front();
        let value0 = self.0.evaluate(input0, solution);
        let value1 = self.1.evaluate(input1, solution);
        match (value0, value1) {
            (Some(a), Some(b)) => {
                Some(<<Self::Problem as Problem>::Objective as Objective>::reduce(a, b))
            }
            _ => None,
        }
    }
}
