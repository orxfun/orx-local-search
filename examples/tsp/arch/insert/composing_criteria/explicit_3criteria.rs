use super::super::criteria::{
    capacity::CapacityInsert, duration::DurationInsert, time_windows::TimeWindowsInsert,
};
use crate::{
    Tour,
    criteria::{Capacity, CapacityInput, Duration, DurationMatrix, TimeWindowInput, TimeWindows},
};
use orx_local_search::{
    ComposedCriteria, ComposedCriteriaWithNeighborhood, CriterionWithNeighborhood, LocalSearch,
};

type MyCriteria = ComposedCriteria<ComposedCriteria<Duration, Capacity>, TimeWindows>;
type MyCriteriaWithNeighborhood = ComposedCriteriaWithNeighborhood<
    ComposedCriteriaWithNeighborhood<DurationInsert, CapacityInsert>,
    TimeWindowsInsert,
>;

fn print(
    tour: &Tour,
    ((input_duration, input_capacity), input_time_windows): &(
        (&DurationMatrix, &CapacityInput),
        &TimeWindowInput,
    ),
) {
    let cost_duration = DurationInsert.evaluate(&tour, &input_duration).unwrap();
    let cost_capacity = CapacityInsert.evaluate(&tour, &input_capacity).unwrap();
    let cost_time_windows = TimeWindowsInsert
        .evaluate(&tour, &input_time_windows)
        .unwrap();
    let cost = cost_duration + cost_capacity + cost_time_windows;

    println!("tour: {:?}", &tour);
    println!("cost - Duration: {:?}", cost_duration);
    println!("cost - Capacity: {:?}", cost_capacity);
    println!("cost - TimeWindows: {:?}", cost_time_windows);
    println!("cost: {:?}", cost);
}

pub fn run() {
    println!("\n\nRunning with explicit criteria for (Duration, Capacity, TimeWindows).");

    let criteria = MyCriteria::new();
    let with_neighborhood = MyCriteriaWithNeighborhood::new();

    let input_duration = DurationMatrix::example_input();
    let input_capacity = CapacityInput::example_input();
    let input_time_windows = TimeWindowInput::example_input(&input_duration);
    let input = ((&input_duration, &input_capacity), &input_time_windows);

    let initial_tour = Tour::example_solution();

    println!("\nInitial Solution");
    print(&initial_tour, &input);

    let mut local_search = LocalSearch::new(criteria).with_neighborhood(with_neighborhood);
    let (tour, _) = local_search
        .optimize(initial_tour, &input, None)
        .into_local_optimum()
        .unwrap();

    println!("\nOptimized Solution");
    print(&tour, &input);
}
