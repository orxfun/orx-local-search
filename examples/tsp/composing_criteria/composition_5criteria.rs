use crate::{
    Tour,
    criteria::{
        capacity::{Capacity, CapacityInput},
        duration::{Duration, DurationMatrix},
        time_windows::{TimeWindowInput, TimeWindows},
    },
};
use orx_local_search::{Criterion, LocalSearch};

fn print(
    tour: &Tour,
    ((((input_duration, input_capacity), input_time_windows), _input_duration2), _input_capacity2): &(
        (
            ((&DurationMatrix, &CapacityInput), &TimeWindowInput),
            &DurationMatrix,
        ),
        &CapacityInput,
    ),
) {
    let cost_duration = Duration.evaluate(&tour, &input_duration).unwrap();
    let cost_capacity = Capacity.evaluate(&tour, &input_capacity).unwrap();
    let cost_time_windows = TimeWindows.evaluate(&tour, &input_time_windows).unwrap();
    let cost = cost_duration + cost_capacity + cost_time_windows;

    println!("tour: {:?}", &tour);
    println!("cost - Duration: {:?}", cost_duration);
    println!("cost - Capacity: {:?}", cost_capacity);
    println!("cost - TimeWindows: {:?}", cost_time_windows);
    println!("cost: {:?}", cost);
}

pub fn run() {
    println!(
        "\n\nRunning with composed criteria for (Duration, Capacity, TimeWindows, Duration, Capacity)."
    );

    let my_tsp = Duration
        .compose(Capacity)
        .compose(TimeWindows)
        .compose(Duration)
        .compose(Capacity);

    let input_duration = DurationMatrix::example_input();
    let input_capacity = CapacityInput::example_input();
    let input_time_windows = TimeWindowInput::example_input(&input_duration);
    let input_duration2 = DurationMatrix::example_input();
    let input_capacity2 = CapacityInput::example_input();

    let input = my_tsp
        .input_builder()
        .add(&input_duration)
        .add(&input_capacity)
        .add(&input_time_windows)
        .add(&input_duration2)
        .add(&input_capacity2)
        .value();

    let initial_tour = Tour::example_solution();
    println!("\nInitial Solution");
    print(&initial_tour, &input);

    let mut local_search = LocalSearch::new(my_tsp);
    let (tour, _) = local_search
        .optimize(initial_tour, &input, None)
        .into_local_optimum()
        .unwrap();
    println!("\nOptimized Solution");
    print(&tour, &input);
}
