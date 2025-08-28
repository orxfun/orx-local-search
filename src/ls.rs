use crate::{
    Criterion, EvalMove, EvalSoln, Objective, Problem, Solution, composed::ComposedMoveGenerator,
    move_gen::MoveGenerator, neighborhood::Neighborhood,
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
    pub fn for_criterion<'i, M>(self) -> LocalSearchOf<'i, N, M>
    where
        M: MoveGenerator<'i, Neighborhood = N>,
    {
        Default::default()
    }
}

// non-empty

pub struct LocalSearchOf<'i, N, M>
where
    N: Neighborhood,
    M: MoveGenerator<'i, Neighborhood = N>,
{
    move_generator: M,
    phantom: PhantomData<&'i N>,
}

impl<'i, N, M> Default for LocalSearchOf<'i, N, M>
where
    N: Neighborhood,
    M: MoveGenerator<'i, Neighborhood = N>,
{
    fn default() -> Self {
        Self {
            move_generator: Default::default(),
            phantom: PhantomData,
        }
    }
}

impl<'i, N, M> LocalSearchOf<'i, N, M>
where
    N: Neighborhood,
    M: MoveGenerator<'i, Neighborhood = N>,
{
    pub fn for_criterion<Q>(self) -> LocalSearchOf<'i, N, ComposedMoveGenerator<'i, M, Q>>
    where
        Q: MoveGenerator<'i, Neighborhood = N>,
    {
        LocalSearchOf {
            move_generator: ComposedMoveGenerator::new(self.move_generator, Q::default()),
            phantom: PhantomData,
        }
    }

    fn next_best_move(
        &mut self,
        input: <M::X as Criterion>::Input<'i>,
        solution: &<<M::Neighborhood as Neighborhood>::Problem as Problem>::Solution,
        mut value:
            <<<M::Neighborhood as Neighborhood>::Problem as Problem>::Objective as Objective>::Unit,
    ) -> Option<EvalMove<N>> {
        None
    }

    pub fn run(
        &mut self,
        input: <M::X as Criterion>::Input<'i>,
        initial_solution: <<M::Neighborhood as Neighborhood>::Problem as Problem>::Solution,
        initial_value: Option<
            <<<M::Neighborhood as Neighborhood>::Problem as Problem>::Objective as Objective>::Unit,
        >,
    ) -> Solution<<M::Neighborhood as Neighborhood>::Problem> {
        let initial_value = match initial_value {
            Some(v) => {
                debug_assert_eq!(
                    &EvalSoln::Feasible(v),
                    &<M::X as Criterion>::evaluate(input, &initial_solution)
                );
                EvalSoln::Feasible(v)
            }
            None => <M::X as Criterion>::evaluate(input, &initial_solution),
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
