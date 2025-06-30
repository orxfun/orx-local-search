use crate::criterion::Criterion;

pub struct ComposedCriteria<X1, X2>(X1, X2)
where
    X1: Criterion,
    X2: Criterion<Problem = X1::Problem>;

impl<X1, X2> ComposedCriteria<X1, X2>
where
    X1: Criterion,
    X2: Criterion<Problem = X1::Problem>,
{
}

impl<X1, X2> Criterion for ComposedCriteria<X1, X2>
where
    X1: Criterion,
    X2: Criterion<Problem = X1::Problem>,
{
    type Problem = X1::Problem;

    type Input<'a> = (X1::Input<'a>, X2::Input<'a>);

    type MoveGenerator = todo!();
}
