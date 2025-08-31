use core::marker::PhantomData;

use crate::{Criteria, Criterion, Moves, Neighborhood, neighborhood};
use orx_meta::impl_meta_queue;

// impl_meta_queue!(
//     'i,
//     Moves,
//     Never,
//     MaybeMoveGenerators,
//     MoveGenerators,
//     EmptyMoves,
//     SingleMoves,
//     PairOfMoves
// );

// pub struct Never<N, X>
// where
//     N: Neighborhood,
//     X: Criterion<Problem = N::Problem>,
// {
//     n: N,
// }

// impl Default for Never
// where
//     N: Neighborhood,
//     X: Criterion<Problem = N::Problem>,
// {
//     fn default() -> Self {
//         todo!()
//     }
// }

// impl<'i, N, X> Moves<'i> for Never<N>
// where
//     N: Neighborhood,
//     X: Criterion<Problem = N::Problem>,
// {
//     type Neighborhood = N;

//     type X: X;

//     fn moves<'a>(
//         &'a mut self,
//         input: <Self::X as crate::Criterion>::Input<'i>,
//         solution: &'a <<Self::X as crate::Criterion>::Problem as crate::Problem>::Solution,
//     ) -> impl Iterator<Item = crate::EvalMove<Self::Neighborhood>> + 'a
//     where
//         'i: 'a,
//     {
//         todo!()
//     }
// }

trait MoveGenQueue<'a> {
    type PushBack<X>: MoveGenNonEmptyQueue<'a>
    where
        X: Moves<'a>;

    type Front: Moves<'a>;

    type Back: MoveGenQueue<'a>;

    fn push_back<X>(self, x: X) -> Self::PushBack<X>
    where
        X: Moves<'a>;
}

trait MoveGenNonEmptyQueue<'a>: MoveGenQueue<'a> {
    fn front(&self) -> &Self::Front;

    fn pop_front(self) -> (Self::Front, Self::Back);
}

// // empty

// struct Empty;

// impl<'a> MoveGenQueue<'a> for Empty {
//     type PushBack<X>
//         = Single<'a, X>
//     where
//         X: Moves<'a>;

//     type Front;

//     type Back;

//     fn push_back<X>(self, x: X) -> Self::PushBack<X>
//     where
//         X: Moves<'a>,
//     {
//         todo!()
//     }
// }

// // single

// struct Single<'a, F>(F, PhantomData<&'a ()>)
// where
//     F: Moves<'a>;

// impl<'a, F> MoveGenQueue<'a> for Single<'a, F>
// where
//     F: Moves<'a>,
// {
//     type PushBack<X>
//         = Pair<'a, F, Single<'a, X>>
//     where
//         X: Moves<'a>;

//     type Front = F;

//     type Back = ();

//     fn push_back<X>(self, x: X) -> Self::PushBack<X>
//     where
//         X: Moves<'a>,
//     {
//         todo!()
//     }
// }

// impl<'a, F> MoveGenNonEmptyQueue<'a> for Single<'a, F>
// where
//     F: Moves<'a>,
// {
//     fn front(&self) -> &Self::Front {
//         todo!()
//     }

//     fn pop_front(self) -> (Self::Front, Self::Back) {
//         todo!()
//     }
// }

// // pair

// struct Pair<'a, F, B>(F, B, PhantomData<&'a ()>)
// where
//     F: Moves<'a>,
//     B: MoveGenNonEmptyQueue<'a>;

// impl<'a, F, B> MoveGenQueue<'a> for Pair<'a, F, B>
// where
//     F: Moves<'a>,
//     B: MoveGenNonEmptyQueue<'a>,
// {
//     type PushBack<X>
//         = Pair<'a, F, B::PushBack<X>>
//     where
//         X: Moves<'a>;

//     type Front = F;

//     type Back = B;

//     fn push_back<X>(self, x: X) -> Self::PushBack<X>
//     where
//         X: Moves<'a>,
//     {
//         Pair(self.0, self.1.push_back(x), PhantomData)
//     }
// }

// impl<'a, F, B> MoveGenNonEmptyQueue<'a> for Pair<'a, F, B>
// where
//     F: Moves<'a>,
//     B: MoveGenNonEmptyQueue<'a>,
// {
//     fn front(&self) -> &Self::Front {
//         &self.0
//     }

//     fn pop_front(self) -> (Self::Front, Self::Back) {
//         (self.0, self.1)
//     }
// }
