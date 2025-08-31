// use crate::{
//     composition::PairOfCriteria, criterion::Criterion, eval_move::EvalMove, moves::Moves,
//     problem::Problem,
// };
// use orx_meta::define_non_empty_queue;

// define_non_empty_queue!(
//     MovesQueue,
//     MultiMovesQueue,
//     SingleMoves,
//     PairOfMoves,
//     MovesComposition,
//     Moves,
//     'i
// );

// impl<'i, F> Default for SingleMoves<'i, F>
// where
//     F: Moves<'i>,
// {
//     fn default() -> Self {
//         MovesComposition::single(Default::default())
//     }
// }

// impl<'i, F> Moves<'i> for SingleMoves<'i, F>
// where
//     F: Moves<'i>,
// {
//     type Neighborhood = F::Neighborhood;

//     type X = F::X;

//     fn moves<'a>(
//         &'a mut self,
//         input: <Self::X as Criterion>::Input<'i>,
//         solution: &'a <<Self::X as Criterion>::Problem as Problem>::Solution,
//     ) -> impl Iterator<Item = EvalMove<Self::Neighborhood>> + 'a
//     where
//         'i: 'a,
//     {
//         self.0.moves(input, solution)
//     }
// }
