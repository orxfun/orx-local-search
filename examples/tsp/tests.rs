use crate::{all_moves::test_all_insert_moves_iter, insert_move::test_insert_move};

pub fn run() {
    test_insert_move();
    test_all_insert_moves_iter();
}
