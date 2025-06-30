use crate::{Criterion, InputOf, SolutionOf, criterion::ObjectiveUnitOf};

pub struct LocalSearch<X>
where
    X: Criterion,
{
    criterion: X,
    move_generator: X::MoveGenerator,
}

impl<X: Criterion> LocalSearch<X> {
    pub fn new(criterion: X) -> Self {
        let move_generator = criterion.move_generator();
        Self {
            criterion,
            move_generator,
        }
    }

    pub fn local_optimum(
        &self,
        initial_solution: SolutionOf<X>,
        input: InputOf<'_, X>,
        initial_objective_value: Option<ObjectiveUnitOf<X>>,
    ) -> SolutionOf<X> {
        let mut best_value = match initial_objective_value {
            Some(x) => {
                // abc
                Some(x)
            }
            None => {
                // abcdef
                todo!()
            }
        };
        // let mut best_value = initial_objective_value
        //     .unwrap_or_else(|| self.criterion.evaluate(&initial_solution, input));
        initial_solution
    }
}
