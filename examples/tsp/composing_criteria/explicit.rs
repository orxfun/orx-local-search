use crate::criteria::{capacity::Capacity, duration::Duration, time_windows::TimeWindows};
use orx_local_search::ComposedCriteria;

type TspCriteriaA = ComposedCriteria<ComposedCriteria<Duration, Capacity>, TimeWindows>;
