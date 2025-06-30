use crate::{Criterion, InputOf, SolutionOf};

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
        // initial_objective_value: Option<ObjectiveUnitOf<X>>,
        input: InputOf<X>,
    ) -> SolutionOf<X> {
        initial_solution
    }
}
