use crate::{
    composition::criteria::Criteria, eval_move::EvalMove, moves::Moves, neighborhood::Neighborhood,
    problem::Problem,
};
use core::marker::PhantomData;

// traits

// pub trait MoveGen<'i>: Default {
//     // queue

//     type PushBack<M>: MoveGen<'i, N = Self::N, X = <Self::X as Criteria>::PushBack<M::X>>
//     where
//         M: Moves<'i, Neighborhood = Self::N>;

//     type Front;

//     type Back;

//     // moves

//     type N: Neighborhood;

//     type X: Criteria<Problem = <Self::N as Neighborhood>::Problem>;

//     fn moves<'a>(
//         &'a mut self,
//         input: <Self::X as Criteria>::Input<'i>,
//         solution: &'a <<Self::X as Criteria>::Problem as Problem>::Solution,
//     ) -> impl Iterator<Item = EvalMove<Self::N>> + 'a
//     where
//         'i: 'a;
// }
