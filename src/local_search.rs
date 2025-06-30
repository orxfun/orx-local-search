use crate::{Criterion, InputOf, LocalSearchResult, SolutionOf, criterion::ObjectiveUnitOf};

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

    // fn next_best_move(&self)

    pub fn local_optimum(
        &self,
        initial_solution: SolutionOf<X>,
        input: InputOf<'_, X>,
        initial_objective_value: Option<ObjectiveUnitOf<X>>,
    ) -> LocalSearchResult<X> {
        let initial_value = match initial_objective_value.is_some() {
            true => {
                debug_assert_eq!(
                    &initial_objective_value,
                    &self.criterion.evaluate(&initial_solution, input)
                );
                initial_objective_value
            }
            false => self.criterion.evaluate(&initial_solution, input),
        };

        match initial_value {
            None => LocalSearchResult::InfeasibleInitialSolution { initial_solution },
            Some(mut best_value) => {
                // abc
                todo!()
            }
        }
    }
}
