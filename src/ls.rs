use crate::{composed::ComposedMoveGenerator, move_gen::MoveGenerator, neighborhood::Neighborhood};
use core::marker::PhantomData;

// builder

pub struct LocalSearch;

impl LocalSearch {
    pub fn on<N: Neighborhood>(self) -> LocalSearchOn<N> {
        LocalSearchOn(PhantomData)
    }
}

// empty

pub struct LocalSearchOn<N: Neighborhood>(PhantomData<N>);

impl<N: Neighborhood> LocalSearchOn<N> {
    pub fn for_criterion<'i, M>(self) -> LocalSearchOf<'i, N, M>
    where
        M: MoveGenerator<'i, Neighborhood = N>,
    {
        Default::default()
    }
}

// non-empty

pub struct LocalSearchOf<'i, N, M>
where
    N: Neighborhood,
    M: MoveGenerator<'i, Neighborhood = N>,
{
    move_generator: M,
    phantom: PhantomData<&'i N>,
}

impl<'i, N, M> Default for LocalSearchOf<'i, N, M>
where
    N: Neighborhood,
    M: MoveGenerator<'i, Neighborhood = N>,
{
    fn default() -> Self {
        Self {
            move_generator: Default::default(),
            phantom: PhantomData,
        }
    }
}

impl<'i, N, M> LocalSearchOf<'i, N, M>
where
    N: Neighborhood,
    M: MoveGenerator<'i, Neighborhood = N>,
{
    pub fn for_criterion<Q>(self) -> LocalSearchOf<'i, N, ComposedMoveGenerator<'i, M, Q>>
    where
        Q: MoveGenerator<'i, Neighborhood = N>,
    {
        LocalSearchOf {
            move_generator: ComposedMoveGenerator::new(self.move_generator, Q::default()),
            phantom: PhantomData,
        }
    }
}
