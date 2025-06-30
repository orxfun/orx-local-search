use crate::{
    CandidateMoveOf, Criterion, InputOf, LocalSearchResult, MoveGenerator, SolutionOf,
    criterion::ObjectiveUnitOf,
};

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

    fn next_best_move<'a>(
        &mut self,
        solution: &'a SolutionOf<X>,
        input: InputOf<'a, X>,
        mut best_value: ObjectiveUnitOf<X>,
    ) -> Option<CandidateMoveOf<X>> {
        let mut best_move = None;
        for candidate in self.move_generator.moves(solution, input) {
            if candidate.objective_value < best_value {
                best_value = candidate.objective_value;
                best_move = Some(candidate);
            }
        }
        best_move
    }

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
