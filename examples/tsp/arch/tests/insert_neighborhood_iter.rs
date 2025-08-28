#[test]
fn test_all_insert_moves_iter() {
    use crate::*;
    use orx_local_search::*;

    let moves: Vec<_> = AllInsertMovesIter::new(3).collect();
    assert_eq!(
        moves,
        vec![
            InsertMove::new(0, 0),
            InsertMove::new(0, 1),
            InsertMove::new(0, 2),
            InsertMove::new(1, 0),
            InsertMove::new(1, 1),
            InsertMove::new(1, 2),
            InsertMove::new(2, 0),
            InsertMove::new(2, 1),
            InsertMove::new(2, 2),
        ]
    );
}
