use crate::{
    crit::Criterion, crit_on_neighborhood::CriterionOnNeighborhood, eval_move::EvalMove,
    r#move::Move, move_gen::MoveGenerator, problem::Problem, solution::Solution,
};

pub struct LocalSearch<'i, X>
where
    X: CriterionOnNeighborhood,
{
    criterion: X::Criterion,
    move_generator: X::MoveGenerator<'i>,
}

impl<'i, X> Default for LocalSearch<'i, X>
where
    X: CriterionOnNeighborhood,
{
    fn default() -> Self {
        Self {
            criterion: Default::default(),
            move_generator: Default::default(),
        }
    }
}

impl<'i, X> LocalSearch<'i, X>
where
    X: CriterionOnNeighborhood,
{
    pub fn new() -> Self {
        Default::default()
    }

    fn next_best_move(
        &mut self,
        input: &<X::Criterion as Criterion>::Input<'i>,
        solution: &<X::Problem as Problem>::Solution,
        mut value: <X::Problem as Problem>::ObjectiveUnit,
    ) -> Option<EvalMove<X::Neighborhood>> {
        let mut best_move = None;
        for candidate in self.move_generator.moves(solution, input) {
            if candidate.value < value {
                value = candidate.value;
                best_move = Some(candidate);
            }
        }
        best_move
    }

    pub fn optimize(
        &mut self,
        input: &<X::Criterion as Criterion>::Input<'i>,
        mut solution: <X::Problem as Problem>::Solution,
        value: Option<<X::Problem as Problem>::ObjectiveUnit>,
    ) -> Solution<X::Problem> {
        let value = match value.is_some() {
            true => {
                debug_assert_eq!(&value, &self.criterion.evaluate(input, &solution));
                value
            }
            false => self.criterion.evaluate(input, &solution),
        };

        match value {
            None => Solution::InfeasibleSolution { solution },
            Some(mut value) => {
                while let Some(eval_move) = self.next_best_move(input, &solution, value) {
                    solution = eval_move.mv.apply(solution);
                    value = eval_move.value;
                }

                Solution::LocalOptimum { solution, value }
            }
        }
    }
}
