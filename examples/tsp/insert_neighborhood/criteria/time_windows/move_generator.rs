use crate::{
    criteria::TimeWindowsInput,
    insert_neighborhood::{
        criteria::time_windows::iter::TimeWindowsMoves, neighborhood::InsertNeighborhood,
    },
    tsp::Tsp,
};
use orx_local_search::{EvalMove, MoveGenerator, Problem};
use orx_meta::queue::{NonEmptyQueue, Single};

#[derive(Default)]
pub struct TimeWindowsMoveGenerator;

impl<'i> MoveGenerator<'i> for TimeWindowsMoveGenerator {
    type Problem = Tsp;

    type Neighborhood = InsertNeighborhood;

    type Input = Single<&'i TimeWindowsInput<'i>>;

    fn moves<'a>(
        &'a mut self,
        input: Self::Input,
        tour: &'a <Self::Problem as Problem>::Solution,
    ) -> impl Iterator<Item = EvalMove<Self::Neighborhood>> + 'a
    where
        'i: 'a,
    {
        TimeWindowsMoves::new(input.front(), tour)
    }
}
