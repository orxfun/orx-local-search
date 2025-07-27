use crate::{
    crit::Criterion, crit_on_neighborhood::CriterionOnNeighborhood, eval_move::EvalMove,
    move_gen::MoveGenerator, problem::Problem,
};

pub struct LocalSearchOnNeighborhood<'i, X>
where
    X: CriterionOnNeighborhood,
{
    criterion: X::Criterion,
    move_generator: X::MoveGenerator<'i>,
}

impl<'i, X> LocalSearchOnNeighborhood<'i, X>
where
    X: CriterionOnNeighborhood,
{
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

    fn optimize(
        &mut self,
        input: &<X::Criterion as Criterion>::Input<'i>,
        solution: <X::Problem as Problem>::Solution,
        value: Option<<X::Problem as Problem>::ObjectiveUnit>,
    ) {
        let value = match value.is_some() {
            true => {
                debug_assert_eq!(&value, &self.criterion.evaluate(input, &solution));
                value
            }
            false => self.criterion.evaluate(input, &solution),
        };
    }
}

// pub type InputOf<'i, X> = <<X as CriterionOnNeighborhood>::Criterion as Criterion>::Input<'i>;
// pub type SolutionOf<X> = <<X as Criterion>::Problem as Problem>::Solution;
