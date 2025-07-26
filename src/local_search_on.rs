use crate::{
    cand_move::CandidateMove, crit::Criterion, crit_on::CriterionOnNeighborhood,
    move_generator::MoveGenerator, neighborhood::Neighborhood, obj::Objective, problem::Problem,
};

type MoveGen<'i, X> = <X as CriterionOnNeighborhood>::MoveGenerator<'i>;
type Crit<'i, X> = <MoveGen<'i, X> as MoveGenerator>::Criterion;
type N<'i, X> = <MoveGen<'i, X> as MoveGenerator>::Neighborhood;
type Mv<'i, X> = <N<'i, X> as Neighborhood>::Move;
type Input<'i, X> = <Crit<'i, X> as Criterion>::Input;
type Prob<'i, X> = <Crit<'i, X> as Criterion>::Problem;
type Soln<'i, X> = <Prob<'i, X> as Problem>::Solution;
type Obj<'i, X> = <Prob<'i, X> as Problem>::Objective;
type ObjUnit<'i, X> = <Obj<'i, X> as Objective>::Unit;

pub struct LocalSearchOnNeighborhood<'i, X>
where
    X: CriterionOnNeighborhood,
{
    criterion: Crit<'i, X>,
    move_generator: MoveGen<'i, X>,
}

impl<'i, X> Default for LocalSearchOnNeighborhood<'i, X>
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

impl<'i, X> LocalSearchOnNeighborhood<'i, X>
where
    X: CriterionOnNeighborhood,
{
    pub fn new() -> Self {
        Self::default()
    }

    fn next_best_move(
        &mut self,
        input: &Input<'i, X>,
        solution: &Soln<'i, X>,
        mut best_value: ObjUnit<'i, X>,
    ) -> Option<CandidateMove<Mv<'i, X>, Obj<'i, X>>> {
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
        initial_solution: Soln<'i, X>,
        input: &Input<'i, X>,
        initial_objective_value: Option<ObjUnit<'i, X>>,
    ) {
        let initial_value = match initial_objective_value.is_some() {
            true => {
                debug_assert_eq!(
                    &initial_objective_value,
                    &self.criterion.evaluate(input, &initial_solution)
                );
                initial_objective_value
            }
            false => self.criterion.evaluate(input, &initial_solution),
        };

        // match initial_value {
        //     None => LocalSearchResult::InfeasibleInitialSolution {
        //         initial_solution: initial_solution,
        //     },
        //     Some(mut best_value) => {
        //         let mut solution = initial_solution;
        //         while let Some(mv) = self.next_best_move(&solution, input, best_value) {
        //             solution = mv.r#move.apply(solution);
        //             best_value = mv.objective_value;
        //         }

        //         LocalSearchResult::LocalOptimum {
        //             solution,
        //             value: best_value,
        //         }
        //     }
        // }
    }
}
