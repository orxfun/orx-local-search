#[test]
fn test_insert_move() {
    use crate::*;
    use orx_local_search::*;

    let tour = Tour::new((0..7).collect::<Vec<_>>());
    let mv = InsertMove {
        current_position: 2,
        target_position: 2,
    };
    let tour = mv.apply(tour);
    assert_eq!(tour, Tour::new((0..7).collect::<Vec<_>>()));

    let tour = Tour::new((0..7).collect::<Vec<_>>());
    let mv = InsertMove {
        current_position: 1,
        target_position: 4,
    };
    let tour = mv.apply(tour);
    assert_eq!(tour, Tour::new(vec![0, 2, 3, 4, 1, 5, 6]));

    let tour = Tour::new((0..7).collect::<Vec<_>>());
    let mv = InsertMove {
        current_position: 4,
        target_position: 1,
    };
    let tour = mv.apply(tour);
    assert_eq!(tour, Tour::new(vec![0, 4, 1, 2, 3, 5, 6]));
}
