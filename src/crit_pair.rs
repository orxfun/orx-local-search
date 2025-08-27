use crate::{Criterion, Problem};

#[derive(Default, Clone, Copy)]
pub struct ComposedCriteria<X1, X2>(X1, X2)
where
    X1: Criterion,
    X2: Criterion;

impl<X1, X2> Criterion for ComposedCriteria<X1, X2>
where
    X1: Criterion,
    X2: Criterion,
{
    type Problem = X1::Problem;

    type Input<'i> = usize;

    fn evaluate(
        self,
        input: &Self::Input<'_>,
        solution: &<Self::Problem as Problem>::Solution,
    ) -> Option<<Self::Problem as Problem>::ObjectiveUnit> {
        todo!()
    }
}
