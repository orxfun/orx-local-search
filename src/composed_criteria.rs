use crate::{
    ObjectiveValue, Problem, SolutionOf,
    composed_move_generator::ComposedMoveGenerator,
    criterion::{Criterion, ObjectiveUnitOf},
};

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

    type MoveGenerator = ComposedMoveGenerator<X1, X2>;

    fn move_generator(&self) -> Self::MoveGenerator {
        ComposedMoveGenerator::new(self.0.move_generator(), self.1.move_generator())
    }

    fn evaluate<'a>(
        &self,
        solution: &SolutionOf<Self>,
        (input1, input2): Self::Input<'a>,
    ) -> Option<ObjectiveUnitOf<Self>> {
        self.0.evaluate(solution, input1).and_then(|value1| {
            self.1.evaluate(solution, input2).map(|value2| {
                <<X1::Problem as Problem>::ObjectiveValue as ObjectiveValue>::reduce(value1, value2)
            })
        })
    }
}
