use crate::{
    EvalMove, EvalSoln, Moves, Neighborhood, Objective, Problem, Solution,
    composition::{Criteria, EmptyInputs, EmptyMoveGen, InputBuilder, MoveGen},
};
use core::marker::PhantomData;

pub struct LocalSearch<'i, P, N, M>
where
    P: Problem,
    N: Neighborhood<P>,
    M: MoveGen<'i, P, N>,
{
    move_gen: M,
    phantom: PhantomData<&'i (P, N)>,
}

impl<'i, P, N, M> Default for LocalSearch<'i, P, N, M>
where
    P: Problem,
    N: Neighborhood<P>,
    M: MoveGen<'i, P, N>,
{
    fn default() -> Self {
        Self {
            move_gen: Default::default(),
            phantom: PhantomData,
        }
    }
}

impl<'i, P, N> LocalSearch<'i, P, N, EmptyMoveGen>
where
    P: Problem,
    N: Neighborhood<P>,
{
    pub fn new() -> Self {
        Self::default()
    }
}

impl<'i, P, N, M> LocalSearch<'i, P, N, M>
where
    P: Problem,
    N: Neighborhood<P>,
    M: MoveGen<'i, P, N>,
{
    pub fn with<Q>(self) -> LocalSearch<'i, P, N, M::Compose<Q>>
    where
        Q: Moves<'i, P, N>,
    {
        LocalSearch {
            move_gen: self.move_gen.compose(Q::default()),
            phantom: PhantomData,
        }
    }

    pub fn input_builder(&self) -> InputBuilder<<M::X as Criteria<P>>::Input<'i>, EmptyInputs> {
        InputBuilder::new()
    }

    // algorithm

    pub fn evaluate(
        &self,
        input: &<M::X as Criteria<P>>::Input<'i>,
        solution: &P::Solution,
    ) -> EvalSoln<P> {
        <M::X as Criteria<P>>::evaluate(input, solution)
    }

    fn next_best_move(
        &mut self,
        input: &'i <M::X as Criteria<P>>::Input<'i>,
        solution: &P::Solution,
        mut value: <P::Objective as Objective>::Unit,
    ) -> Option<EvalMove<P, N>> {
        let mut best_move = None;

        for eval_move in self.move_gen.moves(input, solution) {
            if eval_move.value < value {
                value = eval_move.value;
                best_move = Some(eval_move);
            }
        }

        best_move
    }

    pub fn run(
        &mut self,
        input: &'i <M::X as Criteria<P>>::Input<'i>,
        initial_solution: P::Solution,
        initial_value: Option<<P::Objective as Objective>::Unit>,
    ) -> Solution<P> {
        let initial_value = match initial_value {
            Some(v) => {
                debug_assert_eq!(
                    &EvalSoln::Feasible(v),
                    &<M::X as Criteria<P>>::evaluate(input, &initial_solution)
                );
                EvalSoln::Feasible(v)
            }
            None => <M::X as Criteria<P>>::evaluate(input, &initial_solution),
        };

        match initial_value {
            EvalSoln::Infeasible => Solution::InfeasibleSolution {
                solution: initial_solution,
            },
            EvalSoln::Feasible(mut value) => {
                let mut solution = initial_solution;
                while let Some(eval_move) = self.next_best_move(input, &solution, value) {
                    N::apply_move(&eval_move.mv, &mut solution);
                    value = eval_move.value;
                }

                Solution::LocalOptimum { solution, value }
            }
        }
    }
}
