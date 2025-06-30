use crate::r#move::Move;

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
    pub fn compose(self, other: CandidateMove<M, V>) {
        debug_assert_eq!(&self.r#move, &other.r#move);
        todo!()
    }
}
