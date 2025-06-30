use crate::{Criterion, InputOf, SolutionOf, criterion::ObjectiveUnitOf};

pub struct LocalSearch<X>
where
    X: Criterion,
{
    move_generator: X::MoveGenerator,
}

impl<X: Criterion> LocalSearch<X> {
    pub fn new(move_generator: X::MoveGenerator) -> Self {
        Self { move_generator }
    }

    pub fn local_optimum(
        &self,
        initial_solution: SolutionOf<X>,
        input: InputOf<X>,
        initial_objective_value: Option<ObjectiveUnitOf<X>>,
    ) -> SolutionOf<X> {
        // let mut best_value = initial_objective_value
        initial_solution
    }
}
