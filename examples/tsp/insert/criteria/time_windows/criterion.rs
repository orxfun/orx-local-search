use crate::{
    criteria::TimeWindows,
    insert::{
        criteria::time_windows::move_generator::TimeWindowMoveGenerator,
        neighborhood::InsertNeighborhood,
    },
};
use orx_local_search::CriterionWithNeighborhood;

#[derive(Default, Clone, Copy)]
pub struct TimeWindowsInsert;

impl CriterionWithNeighborhood for TimeWindowsInsert {
    type Criterion = TimeWindows;

    type Neighborhood = InsertNeighborhood;

    type MoveGenerator<'i> = TimeWindowMoveGenerator;

    fn move_generator<'i>(self) -> Self::MoveGenerator<'i> {
        TimeWindowMoveGenerator
    }
}
