use crate::{
    criteria::Duration,
    insert::{
        criteria::duration::move_generator::DurationMoveGenerator, neighborhood::InsertNeighborhood,
    },
};
use orx_local_search::CriterionWithNeighborhood;

#[derive(Default, Clone, Copy)]
pub struct DurationInsert;

impl CriterionWithNeighborhood for DurationInsert {
    type Criterion = Duration;

    type Neighborhood = InsertNeighborhood;

    type MoveGenerator<'i> = DurationMoveGenerator;

    fn move_generator<'i>(self) -> Self::MoveGenerator<'i> {
        DurationMoveGenerator
    }
}
