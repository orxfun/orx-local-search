use crate::{criterion::Criterion, problem::Problem};

pub trait MoveGenerator {
    type X: Criterion;

    fn moves<'a>(
        &mut self,
        object: &<<Self::X as Criterion>::Problem as Problem>::On,
        input: <Self::X as Criterion>::Input<'a>,
    ) -> impl Iterator<Item = <<Self::X as Criterion>::Problem as Problem>::Move>;
}
