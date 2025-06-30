use orx_local_search::MoveGenerator;

use crate::insert_move::InsertMove;

pub struct Neighborhood;

// impl MoveGenerator for Neighborhood {
//     type X = ;

//     fn moves<'a>(
//         &mut self,
//         solution: &SolutionOf<Self::X>,
//         input: InputOf<'a, Self::X>,
//     ) -> impl Iterator<Item = CandidateMoveOf<Self::X>> {
//         todo!()
//     }
// }

pub struct AllInsertMovesIter {
    n: usize,
    current_position: usize,
    target_position: usize,
}

impl AllInsertMovesIter {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            current_position: 0,
            target_position: 0,
        }
    }

    fn next_current_position(&mut self) -> Option<InsertMove> {
        self.current_position += 1;
        match self.current_position == self.n {
            false => {
                self.target_position = 0;
                self.next()
            }
            true => None,
        }
    }
}

impl Iterator for AllInsertMovesIter {
    type Item = InsertMove;

    fn next(&mut self) -> Option<Self::Item> {
        match self.target_position == self.n {
            false => {
                let mv = InsertMove::new(self.current_position, self.target_position);
                self.target_position += 1;
                Some(mv)
            }
            true => self.next_current_position(),
        }
    }
}

pub fn test_all_insert_moves_iter() {
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
