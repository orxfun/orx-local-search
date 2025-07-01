use crate::{
    Tour,
    criteria::{
        capacity::{Capacity, CapacityInput},
        duration::{Duration, DurationMatrix},
        time_windows::{TimeWindowInput, TimeWindows},
    },
};
use orx_local_search::{ComposedCriteria, Criterion, LocalSearch};

type TspCriteriaA = ComposedCriteria<ComposedCriteria<Duration, Capacity>, TimeWindows>;

fn print(
    tour: &Tour,
    input_duration: &DurationMatrix,
    input_capacity: &CapacityInput,
    input_time_windows: &TimeWindowInput,
) {
    let cost_duration = Duration::evaluate(&tour, &input_duration).unwrap();
    let cost_capacity = Capacity::evaluate(&tour, &input_capacity).unwrap();
    let cost_time_windows = TimeWindows::evaluate(&tour, &input_time_windows).unwrap();
    let cost = cost_duration + cost_capacity + cost_time_windows;

    println!("tour: {:?}", &tour);
    println!("cost - Duration: {:?}", cost_duration);
    println!("cost - Capacity: {:?}", cost_capacity);
    println!("cost - TimeWindows: {:?}", cost_time_windows);
    println!("cost: {:?}", cost);
}

pub fn run() {
    println!("\n\nRunning with explicit criteria for (Duration, Capacity, TimeWindows).");

    let initial_tour = Tour::example_solution();

    let input_duration = DurationMatrix::example_input();
    let input_capacity = CapacityInput::example_input();
    let input_time_windows = TimeWindowInput::example_input();

    println!("\nInitial Solution");
    print(
        &initial_tour,
        &input_duration,
        &input_capacity,
        &input_time_windows,
    );

    let input = ((input_duration, input_capacity), input_time_windows);
    let mut local_search = LocalSearch::<TspCriteriaA>::new();
    let (tour, cost) = local_search
        .optimize(initial_tour, &input, None)
        .into_local_optimum()
        .unwrap();

    let ((input_duration, input_capacity), input_time_windows) = input;
    println!("\nOptimized Solution");
    print(&tour, &input_duration, &input_capacity, &input_time_windows);
}
