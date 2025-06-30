use crate::criterion::{CandidateMoveOf, Criterion, InputOf, ObjectOf};

pub trait MoveGenerator {
    type X: Criterion;

    fn moves<'a>(
        &mut self,
        object: &ObjectOf<Self::X>,
        input: InputOf<'a, Self::X>,
    ) -> impl Iterator<Item = CandidateMoveOf<Self::X>>;
}
