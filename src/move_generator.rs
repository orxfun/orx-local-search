use crate::{CandidateMoveOf, Neighborhood, Problem};

pub trait MoveGenerator<'i> {
    type Neighborhood: Neighborhood;

    type Input;

    fn moves<'a>(
        &'a mut self,
        solution: &'a <<Self::Neighborhood as Neighborhood>::Problem as Problem>::Solution,
        input: &'a Self::Input,
    ) -> impl Iterator<Item = CandidateMoveOf<Self::Neighborhood>> + 'a;
}
