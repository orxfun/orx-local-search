use std::marker::PhantomData;

use crate::{
    ObjectiveValue, Problem, SolutionOf,
    composed_move_generator::ComposedMoveGenerator,
    criterion::{Criterion, ObjectiveUnitOf},
};

pub struct ComposedCriteria<X1, X2>(PhantomData<(X1, X2)>)
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

    type Input = (X1::Input, X2::Input);

    type MoveGenerator = ComposedMoveGenerator<X1, X2>;

    fn move_generator() -> Self::MoveGenerator {
        ComposedMoveGenerator::new(X1::move_generator(), X2::move_generator())
    }

    fn evaluate(
        solution: &SolutionOf<Self>,
        (input1, input2): &Self::Input,
    ) -> Option<ObjectiveUnitOf<Self>> {
        X1::evaluate(solution, input1).and_then(|value1| {
            X2::evaluate(solution, input2).map(|value2| {
                <<X1::Problem as Problem>::ObjectiveValue as ObjectiveValue>::reduce(value1, value2)
            })
        })
    }
}
