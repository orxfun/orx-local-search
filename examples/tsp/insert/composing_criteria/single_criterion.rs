use crate::{Tour, criteria::DurationMatrix, insert::criteria::duration::DurationInsert};
use orx_local_search::{CriterionWithNeighborhood, LocalSearchOnNeighborhood};

fn print(tour: &Tour, input_duration: &DurationMatrix) {
    let cost_duration = DurationInsert.evaluate(&tour, &input_duration).unwrap();

    println!("tour: {:?}", &tour);
    println!("cost - Duration: {:?}", cost_duration);
}

pub fn run() {
    println!("\n\nRunning with single explicit criterion for (Duration).");

    let my_tsp = DurationInsert;

    let input_duration = DurationMatrix::example_input();

    let initial_tour = Tour::example_solution();

    println!("\nInitial Solution");
    print(&initial_tour, &input_duration);

    let mut local_search = LocalSearchOnNeighborhood::new(my_tsp);
    let (tour, _) = local_search
        .optimize(initial_tour, &input_duration, None)
        .into_local_optimum()
        .unwrap();

    println!("\nOptimized Solution");
    print(&tour, &input_duration);
}
