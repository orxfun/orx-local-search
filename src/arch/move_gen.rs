use crate::{
    Criterion, EvalMove, Moves, Neighborhood, Problem,
    criteria::{Criteria, NoCriteria},
};

// orx_meta::define_queue!(
//     lt => ['i];
//     generics => [P: Problem, N: Neighborhood<P>];
//     elements => [ Moves<'i, P, N> ];
//     queue => [ MoveGenQueue ; EmptyMoveGen, MoveGens ];
//     queue_of => xyz;
//     builder => Xyz;
// );

// impl<'i, P, N> Moves<'i, P, N> for EmptyMoveGen<'i, P, N>
// where
//     P: Problem,
//     N: Neighborhood<P>,
// {
//     type X = NoCriteria<P>;

//     fn moves<'a>(
//         &'a mut self,
//         input: &'i <Self::X as Criterion<P>>::Input<'i>,
//         solution: &'a <P as Problem>::Solution,
//     ) -> impl Iterator<Item = EvalMove<P, N>> + 'a
//     where
//         'i: 'a,
//     {
//         core::iter::empty()
//     }
// }

// impl<'i, P, N, F, B> Moves<'i, P, N> for MoveGens<'i, P, N, F, B>
// where
//     P: Problem,
//     N: Neighborhood<P>,
//     F: Moves<'i, P, N>,
//     B: MoveGenQueue<'i, P, N>,
// {
//     type X = Criteria<P, F::X, B::X>;

//     fn moves<'a>(
//         &'a mut self,
//         input: &'i <Self::X as Criterion<P>>::Input<'i>,
//         solution: &'a <P as Problem>::Solution,
//     ) -> impl Iterator<Item = EvalMove<P, N>> + 'a
//     where
//         'i: 'a,
//     {
//         core::iter::empty()
//     }
// }
