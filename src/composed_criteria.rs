use crate::{
    Neighborhood, ObjectiveValue, Problem, SolutionOf,
    composed_move_generator::ComposedMoveGenerator,
    criterion::{Criterion, ObjectiveUnitOf},
};
use orx_meta::queue::MetaQueue;

#[derive(Clone, Copy)]
pub struct ComposedCriteria<X1, X2>(X1, X2)
where
    X1: Criterion,
    X2: Criterion<Neighborhood = X1::Neighborhood>;

impl<X1, X2> Default for ComposedCriteria<X1, X2>
where
    X1: Criterion,
    X2: Criterion<Neighborhood = X1::Neighborhood>,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<X1, X2> ComposedCriteria<X1, X2>
where
    X1: Criterion,
    X2: Criterion<Neighborhood = X1::Neighborhood>,
{
    pub fn new() -> Self {
        Self(Default::default(), Default::default())
    }
}

impl<X1, X2> Criterion for ComposedCriteria<X1, X2>
where
    X1: Criterion,
    X2: Criterion<Neighborhood = X1::Neighborhood>,
{
    type Neighborhood = X1::Neighborhood;

    type Input<'i> = (X1::Input<'i>, X2::Input<'i>);

    type MoveGenerator<'i> = ComposedMoveGenerator<'i, X1, X2>;

    type InputQueue<'i> = <X1::InputQueue<'i> as MetaQueue>::Extend<X2::InputQueue<'i>>;

    fn move_generator<'i>(self) -> Self::MoveGenerator<'i> {
        ComposedMoveGenerator::new(X1::move_generator(self.0), X2::move_generator(self.1))
    }

    fn evaluate(
        self,
        solution: &SolutionOf<Self>,
        (input1, input2): &Self::Input<'_>,
    ) -> Option<ObjectiveUnitOf<Self>> {
        self.0.evaluate(solution, input1).and_then(|value1| {
            self.1.evaluate(solution, input2).map(|value2| {
                <<<X1::Neighborhood as Neighborhood>::Problem as Problem>::ObjectiveValue as ObjectiveValue>::reduce(
                    value1, value2,
                )
            })
        })
    }
}
