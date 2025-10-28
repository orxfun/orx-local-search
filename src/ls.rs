use crate::composition::{CriteriaQueue, InputBuilder, MoveGenQueue, MoveGenSingle};
use crate::core::{EvalMove, EvalSoln, Moves, Neighborhood, Objective, Problem, Solution};
use core::marker::PhantomData;

pub struct LocalSearch<'i, P, N, M>
where
    P: Problem,
    N: Neighborhood<P>,
    M: MoveGenQueue<'i, P, N>,
{
    move_gen: M,
    phantom: PhantomData<&'i (P, N)>,
}

impl<'i, P, N, M> LocalSearch<'i, P, N, MoveGenSingle<'i, P, N, M>>
where
    P: Problem,
    N: Neighborhood<P>,
    M: Moves<'i, P, N>,
{
    pub fn new(move_generator: M) -> Self {
        Self {
            move_gen: MoveGenSingle::new(move_generator),
            phantom: PhantomData,
        }
    }
}

impl<'i, P, N, M> LocalSearch<'i, P, N, M>
where
    P: Problem,
    N: Neighborhood<P>,
    M: MoveGenQueue<'i, P, N>,
{
    pub fn and_with<X>(self, move_generator: X) -> LocalSearch<'i, P, N, M::PushBack<X>>
    where
        X: Moves<'i, P, N>,
    {
        let move_gen = self.move_gen.push(move_generator);
        LocalSearch {
            move_gen,
            phantom: PhantomData,
        }
    }

    pub fn input_builder(&self) -> InputBuilder<<M::X as CriteriaQueue<P>>::Input<'i>> {
        Default::default()
    }

    // algorithm

    // pub fn evaluate(
    //     &self,
    //     input: &<M::X as CriteriaQueue<P>>::Input<'i>,
    //     solution: &P::Solution,
    // ) -> EvalSoln<P> {
    //     <M::X as CriteriaQueue<P>>::evaluate(input, solution)
    // }

    fn next_best_move(
        &mut self,
        input: &'i <M::X as CriteriaQueue<P>>::Input<'i>,
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
        input: &'i <M::X as CriteriaQueue<P>>::Input<'i>,
        initial_solution: P::Solution,
        initial_value: Option<<P::Objective as Objective>::Unit>,
    ) -> Solution<P> {
        // let initial_value = match initial_value {
        //     Some(v) => {
        //         debug_assert_eq!(
        //             &EvalSoln::Feasible(v),
        //             &<M::X as CriteriaQueue<P>>::evaluate(input, &initial_solution)
        //         );
        //         EvalSoln::Feasible(v)
        //     }
        //     None => <M::X as CriteriaQueue<P>>::evaluate(input, &initial_solution),
        // };
        let v = initial_value.unwrap();
        let initial_value: EvalSoln<P> = EvalSoln::Feasible(v);

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
