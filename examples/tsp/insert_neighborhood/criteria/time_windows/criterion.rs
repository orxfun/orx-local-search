use crate::{
    criteria::TimeWindows,
    insert_neighborhood::{
        criteria::time_windows::move_generator::TimeWindowsMoveGenerator,
        neighborhood::InsertNeighborhood,
    },
    tsp::Tsp,
};
use orx_local_search::CriterionOnNeighborhood;

pub struct TimeWindowsOnInsert;

impl CriterionOnNeighborhood for TimeWindowsOnInsert {
    type Problem = Tsp;

    type Criterion = TimeWindows;

    type Neighborhood = InsertNeighborhood;

    type MoveGenerator<'i> = TimeWindowsMoveGenerator;
}
