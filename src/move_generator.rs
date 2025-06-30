use crate::{criterion::Criterion, r#move::Move};

pub trait MoveGenerator {
    type X: Criterion;

    type Move: Move;

    fn moves<'a>(
        &mut self,
        object: &<Self::X as Criterion>::On,
        input: <Self::X as Criterion>::Input<'a>,
    ) -> impl Iterator<Item = Self::Move>;
}
