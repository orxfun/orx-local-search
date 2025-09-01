use crate::{
    composition::{Criteria, EmptyInputs, InputBuilder, MoveGen, SingleMoveGen},
    eval_move::EvalMove,
    eval_soln::EvalSoln,
    moves::Moves,
    neighborhood::Neighborhood,
    objective::Objective,
    problem::Problem,
    solution::Solution,
};
use core::marker::PhantomData;

// builder

pub struct LocalSearch;

impl LocalSearch {
    pub fn on<N: Neighborhood>(self) -> LocalSearchOn<N> {
        LocalSearchOn(PhantomData)
    }
}

// empty

pub struct LocalSearchOn<N: Neighborhood>(PhantomData<N>);

impl<N: Neighborhood> LocalSearchOn<N> {
    pub fn for_criterion<'i, M>(self) -> LocalSearchOf<'i, SingleMoveGen<'i, M>>
    where
        M: Moves<'i, Neighborhood = N>,
    {
        Default::default()
    }
}

// non-empty

#[derive(Default)]
pub struct LocalSearchOf<'i, M>(M, PhantomData<&'i ()>)
where
    M: MoveGen<'i>;

impl<'i, M> LocalSearchOf<'i, M>
where
    M: MoveGen<'i>,
{
    pub fn for_criterion<Q>(self) -> LocalSearchOf<'i, M::PushBack<Q>>
    where
        Q: Moves<'i, Neighborhood = M::Neighborhood>,
    {
        Default::default()
    }

    pub fn input_buidler(&self) -> InputBuilder<<M::X as Criteria>::Input<'i>, EmptyInputs> {
        InputBuilder::new()
    }

    fn next_best_move(
        &mut self,
        input: &'i <M::X as Criteria>::Input<'i>,
        solution: &<<M::Neighborhood as Neighborhood>::Problem as Problem>::Solution,
        mut value:
            <<<M::Neighborhood as Neighborhood>::Problem as Problem>::Objective as Objective>::Unit,
    ) -> Option<EvalMove<M::Neighborhood>> {
        let mut best_move = None;

        for eval_move in self.0.moves(input, solution) {
            if eval_move.value < value {
                value = eval_move.value;
                best_move = Some(eval_move);
            }
        }

        best_move
    }

    pub fn evaluate(
        &self,
        input: &<M::X as Criteria>::Input<'i>,
        solution: &<<M::Neighborhood as Neighborhood>::Problem as Problem>::Solution,
    ) -> EvalSoln<<<M as MoveGen<'i>>::Neighborhood as Neighborhood>::Problem> {
        <M::X as Criteria>::evaluate(input, solution)
    }

    pub fn run(
        &mut self,
        input: &'i <M::X as Criteria>::Input<'i>,
        initial_solution: <<M::Neighborhood as Neighborhood>::Problem as Problem>::Solution,
        initial_value: Option<
            <<<M::Neighborhood as Neighborhood>::Problem as Problem>::Objective as Objective>::Unit,
        >,
    ) -> Solution<<M::Neighborhood as Neighborhood>::Problem> {
        let initial_value = match initial_value {
            Some(v) => {
                debug_assert_eq!(
                    &EvalSoln::Feasible(v),
                    &<M::X as Criteria>::evaluate(input, &initial_solution)
                );
                EvalSoln::Feasible(v)
            }
            None => <M::X as Criteria>::evaluate(input, &initial_solution),
        };

        match initial_value {
            EvalSoln::Infeasible => Solution::InfeasibleSolution {
                solution: initial_solution,
            },
            EvalSoln::Feasible(mut value) => {
                let mut solution = initial_solution;
                while let Some(eval_move) = self.next_best_move(input, &solution, value) {
                    <M::Neighborhood as Neighborhood>::apply_move(&eval_move.mv, &mut solution);
                    value = eval_move.value;
                }

                Solution::LocalOptimum { solution, value }
            }
        }
    }
}
