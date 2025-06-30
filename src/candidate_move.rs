use crate::{criterion::Criterion, r#move::Move, problem::Problem};

pub struct CandidateMove<M, V>
where
    M: Move,
{
    r#move: M,
    value: V,
}

impl<M, V> CandidateMove<M, V>
where
    M: Move,
{
    pub fn compose<N>(self, other: CandidateMove<N, V>)
    where
        N: Move<On = M::On>,
    {
        todo!()
    }
}
