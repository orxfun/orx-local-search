use crate::{
    CandidateMoveOf, Criterion, CriterionWithNeighborhood, InputOf, LocalSearchResult, Move,
    MoveGenerator, SolutionOf, criterion::ObjectiveUnitOf,
};
use orx_self_or::SoR;

pub struct LocalSearchOnNeighborhood<'i, X>
where
    X: CriterionWithNeighborhood,
{
    criterion: X::Criterion,
    move_generator: X::MoveGenerator<'i>,
}

impl<'i, X> LocalSearchOnNeighborhood<'i, X>
where
    X: CriterionWithNeighborhood,
{
    pub fn new(criterion_with_neighborhood: X) -> Self {
        let criterion = X::Criterion::default();
        Self {
            criterion,
            move_generator: criterion_with_neighborhood.move_generator(),
        }
    }

    fn next_best_move(
        &mut self,
        solution: &SolutionOf<X::Criterion>,
        input: &InputOf<'i, X>,
        mut best_value: ObjectiveUnitOf<X::Criterion>,
    ) -> Option<CandidateMoveOf<X::Neighborhood>> {
        let mut best_move = None;
        for candidate in self.move_generator.moves(solution, input) {
            if candidate.objective_value < best_value {
                best_value = candidate.objective_value;
                best_move = Some(candidate);
            }
        }
        best_move
    }

    pub fn optimize(
        &mut self,
        initial_solution: SolutionOf<X::Criterion>,
        input: impl SoR<InputOf<'i, X>>,
        initial_objective_value: Option<ObjectiveUnitOf<X::Criterion>>,
    ) -> LocalSearchResult<X::Criterion> {
        let input = input.get_ref();

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
            None => LocalSearchResult::InfeasibleInitialSolution {
                initial_solution: initial_solution,
            },
            Some(mut best_value) => {
                let mut solution = initial_solution;
                while let Some(mv) = self.next_best_move(&solution, input, best_value) {
                    solution = mv.r#move.apply(solution);
                    best_value = mv.objective_value;
                }

                LocalSearchResult::LocalOptimum {
                    solution,
                    value: best_value,
                }
            }
        }
    }
}
