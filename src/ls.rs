// use crate::{
//     composition::{InputsQueue, MovesComposition, MovesQueue, SingleInput, SingleMoves},
//     criterion::Criterion,
//     eval_move::EvalMove,
//     moves::Moves,
//     neighborhood::Neighborhood,
//     objective::Objective,
//     problem::Problem,
// };
// use core::marker::PhantomData;

// // types

// type InputOf<'i, M> = <<M as Moves<'i>>::X as Criterion>::Input<'i>;

// // builder

// pub struct LocalSearch;

// impl LocalSearch {
//     pub fn on<N: Neighborhood>(self) -> LocalSearchOn<N> {
//         LocalSearchOn(PhantomData)
//     }
// }

// // empty

// pub struct LocalSearchOn<N: Neighborhood>(PhantomData<N>);

// impl<N: Neighborhood> LocalSearchOn<N> {
//     pub fn for_criterion<'i, M>(
//         self,
//     ) -> LocalSearchOf<'i, N, SingleMoves<'i, M>, SingleInput<'i, InputOf<'i, M>>>
//     where
//         M: Moves<'i, Neighborhood = N>,
//     {
//         LocalSearchOf {
//             move_generator: MovesComposition::single(Default::default()),
//             phantom: PhantomData,
//         }
//     }
// }

// // non-empty

// pub struct LocalSearchOf<'i, N, M, I>
// where
//     N: Neighborhood,
//     M: MovesQueue<'i>,
//     I: InputsQueue<'i>,
// {
//     move_generator: M,
//     phantom: PhantomData<&'i (N, I)>,
// }

// impl<'i, N, M, I> LocalSearchOf<'i, N, M, I>
// where
//     N: Neighborhood,
//     M: MovesQueue<'i>,
//     I: InputsQueue<'i>,
// {
//     pub fn for_criterion<Q>(
//         self,
//     ) -> LocalSearchOf<'i, N, M::PushBack<Q>, I::PushBack<InputOf<'i, Q>>>
//     where
//         Q: Moves<'i, Neighborhood = N>,
//     {
//         LocalSearchOf {
//             move_generator: self.move_generator.push_back(Q::default()),
//             phantom: PhantomData,
//         }
//     }

//     fn next_best_move(
//         &mut self,
//         input: I,
//         solution: &<N::Problem as Problem>::Solution,
//         mut value: <<N::Problem as Problem>::Objective as Objective>::Unit,
//     ) -> Option<EvalMove<N>> {
//         let mut best_move = None;

//         // for eval_move in self.move_generator.moves(input, solution) {
//         //     if eval_move.value < value {
//         //         value = eval_move.value;
//         //         best_move = Some(eval_move);
//         //     }
//         // }

//         best_move
//     }
// }
