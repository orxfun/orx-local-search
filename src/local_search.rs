use crate::{
    CandidateMoveOf, Criterion, InputOf, LocalSearchResult, Move, MoveGenerator, SolutionOf,
    criterion::ObjectiveUnitOf,
};

pub struct LocalSearch<X>
where
    X: Criterion,
{
    move_generator: X::MoveGenerator,
}

impl<X: Criterion> LocalSearch<X> {
    pub fn new() -> Self {
        let move_generator = X::move_generator();
        Self { move_generator }
    }

    fn next_best_move(
        &mut self,
        solution: &SolutionOf<X>,
        input: &InputOf<X>,
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
        &mut self,
        initial_solution: &SolutionOf<X>,
        input: &InputOf<X>,
        initial_objective_value: Option<ObjectiveUnitOf<X>>,
    ) -> LocalSearchResult<X> {
        let initial_value = match initial_objective_value.is_some() {
            true => {
                debug_assert_eq!(
                    &initial_objective_value,
                    &X::evaluate(&initial_solution, input)
                );
                initial_objective_value
            }
            false => X::evaluate(&initial_solution, input),
        };

        match initial_value {
            None => LocalSearchResult::InfeasibleInitialSolution {
                initial_solution: initial_solution.clone(),
            },
            Some(mut best_value) => {
                let mut solution = initial_solution.clone();
                // {
                //     // let solution = solution.clone();

                //     let moves = self.move_generator.moves(&solution, input);

                //     // let x = self.next_best_move(&solution, input, best_value);
                //     // drop(x);
                // }
                while let Some(candidate) = self.next_best_move(&solution, input, best_value) {
                    if candidate.objective_value < best_value {
                        solution = candidate.r#move.apply(solution);
                        best_value = candidate.objective_value;
                    }
                }
                LocalSearchResult::LocalOptimum {
                    solution,
                    value: best_value,
                }
            }
        }
    }
}
