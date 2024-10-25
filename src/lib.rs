pub struct Coordinate {
    x: usize,
    y: usize
}

impl Coordinate {

    pub fn new(x: usize, y: usize) -> Coordinate {
        Coordinate{ x, y }
    }

    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
    }

    pub fn value(&self, matrix: &Matrix) -> bool {
        matrix[self.x][self.y]
    }

    pub fn as_tuple(&self) -> (&usize, &usize) {
        (&self.x, &self.y)
    }
}


#[cfg(test)]
mod coordinate_tests {
    use super::Coordinate;

    #[test]
    fn return_x_and_y() {
        let coord = Coordinate::new(0, 0);

        assert_eq!(coord.x(), 0);
        assert_eq!(coord.y(), 0);
    }

    #[test]
    fn return_value_when_calling_value() {
        let coord = Coordinate::new(1, 0);
        let matrix = vec![vec![false, true], vec![true, false]];

        assert!(coord.value(&matrix));
    }

    #[test]
    fn returns_x_and_y_in_a_tuple() {
        let coord = Coordinate::new(1, 2);

        let (x, y) = coord.as_tuple();

        assert!(*x == 1 && *y == 2);
    }

}

type Matrix = Vec<Vec<bool>>;

pub struct Board {
    matrix: Matrix,
    max_x: usize,
    max_y: usize,
    pub current_position: Coordinate,
}

impl Board {

    pub fn new(board: Matrix) -> Board {
        let max_y = board.len();
        let max_x = Board::get_max_x(&board);
        Board{matrix: board, current_position: Coordinate::new(0, 0), max_y, max_x}

    }

    fn get_max_x(board: &Matrix) -> usize {
        let length = board.into_iter().map(|row| row.len()).max().unwrap();

        length
    }

    pub fn next_position(&mut self) {
        let (old_x, old_y) = self.current_position.as_tuple();

        if *old_x + 1 > (self.max_x - 1) {

            if *old_y < (self.max_y - 1) {
                self.current_position = Coordinate::new(0,old_y + 1);
            } else {
                self.current_position = Coordinate::new(0,0);
            }
            
        } else {
            self.current_position = Coordinate::new(old_x + 1, old_y.clone());
        }
        
    }

    pub fn current_positions_value(&self) -> bool {
        self.current_position.value(&self.matrix)
    }
}

#[cfg(test)]
mod board_tests {
    use super::Board;

    #[test]
    fn new_board_with_initial_position() {
        let matrix = vec![vec![false, true], vec![true, false]];
        let board = Board::new(matrix);

        let x = board.current_position.x(); 
        let y = board.current_position.y();

        assert_eq!((x, y), (0, 0));
        assert!(!board.current_positions_value());
    }

    #[test]
    fn next_position_moves_current_position_by_one_on_x() {
        let matrix = vec![vec![false, true], vec![true, false]];
        let mut board = Board::new(matrix);

        board.next_position();

        let (x, y) = board.current_position.as_tuple();

        assert_eq!((*x, *y), (1, 0));
        assert!(board.current_positions_value());
    }

    #[test]
    fn next_position_moves_position_by_one_on_y() {
        let matrix = vec![vec![false, true], vec![true, false]];
        let mut board = Board::new(matrix);

        for _ in 0..3 {
            board.next_position();
        }

        let (x, y) = board.current_position.as_tuple();

        assert_eq!((*x, *y), (1, 1));
        assert!(!board.current_positions_value());
    }

    #[test]
    fn next_position_resets_position_after_max_y() {
        let matrix = vec![vec![false, true], vec![true, false]];
        let mut board = Board::new(matrix);

        for _ in 0..4 {
            board.next_position();
        }

        let (x, y) = board.current_position.as_tuple();

        assert_eq!((*x, *y), (0, 0));
        assert!(!board.current_positions_value());
    }
}