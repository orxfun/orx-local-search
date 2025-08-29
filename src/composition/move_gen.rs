use crate::{criteria::Criteria, eval_move::EvalMove, moves::Moves, problem::Problem};
use core::marker::PhantomData;

#[derive(Default)]
pub struct ComposedMoveGenerator<'i, M1, M2>
where
    M1: Moves<'i>,
    M2: Moves<'i, Neighborhood = M1::Neighborhood>,
{
    m1: M1,
    m2: M2,
    phantom: PhantomData<&'i ()>,
}

// impl<'i, M1, M2> MoveGenerator<'i> for ComposedMoveGenerator<'i, M1, M2>
// where
//     M1: MoveGenerator<'i>,
//     M2: MoveGenerator<'i, Neighborhood = M1::Neighborhood>,
// {
//     type Neighborhood = M1::Neighborhood;

//     type X;
//     // type X = ComposedCriteria<M1::X, M2::X>;

//     fn moves<'a>(
//         &'a mut self,
//         input: <Self::X as crate::Criterion>::Input<'i>,
//         solution: &'a <<Self::X as crate::Criterion>::Problem as Problem>::Solution,
//     ) -> impl Iterator<Item = EvalMove<Self::Neighborhood>> + 'a
//     where
//         'i: 'a,
//     {
//         todo!()
//     }
// }
