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

    pub fn as_tuple(&self) -> (usize, usize) {
        (self.x, self.y)
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

        assert!(x == 1 && y == 2);
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

        if old_x + 1 > (self.max_x - 1) {

            if old_y < (self.max_y - 1) {
                self.current_position = Coordinate::new(0,old_y + 1);
            } else {
                self.current_position = Coordinate::new(0,0);
            }
            
        } else {
            self.current_position = Coordinate::new(old_x + 1, old_y);
        }
        
    }

    pub fn current_positions_value(&self) -> bool {
        self.current_position.value(&self.matrix)
    }

    pub fn current_positions_neighbours(&self) -> Vec<bool> {
        let mut neighbours: Vec<bool> = Vec::new();
        let (x, y) = self.current_position.as_tuple();
        
        // casting to isize to get -ne number for comparison
        let int_x = x as isize;
        let int_y = y as isize;

        if int_y > 0 {
            if int_x - 1 >= 0 {
                // north west value
                neighbours.push(self.matrix[y - 1][x - 1]);
            }
            // north value
            neighbours.push(self.matrix[y - 1][x]);

            if (int_x + 1) <= (self.max_x - 1) as isize {
                // north east value
                neighbours.push(self.matrix[y - 1][x + 1]);
            } 
        }

        if (int_x + 1) <= (self.max_x - 1) as isize {
            // east value
            neighbours.push(self.matrix[y][x + 1]);
        }

        if (int_y + 1) <= (self.max_y - 1) as isize {

            if (int_x + 1) <= (self.max_x - 1) as isize {
                // south east value
                neighbours.push(self.matrix[y + 1][x + 1]);
            }

            // south value
            neighbours.push(self.matrix[y + 1][x]);

            if int_x - 1 >= 0 {
                // south west value
                neighbours.push(self.matrix[y + 1][x - 1]);
            }
        }

        if int_x - 1 >= 0 {
            // west value
            neighbours.push(self.matrix[y][x - 1]);
        }


        neighbours
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

        assert_eq!((x, y), (1, 0));
        assert!(board.current_positions_value());
    }

    #[test]
    fn next_position_moves_position_by_one_on_y() {
        let matrix = vec![
            vec![false, true],
            vec![true, false]
            ];
        let mut board = Board::new(matrix);

        for _ in 0..3 {
            board.next_position();
        }

        let (x, y) = board.current_position.as_tuple();

        assert_eq!((x, y), (1, 1));
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

        assert_eq!((x, y), (0, 0));
        assert!(!board.current_positions_value());
    }

    #[test]
    fn current_positions_neighbours_returns_vec() {
        let matrix = vec![
            vec![false, true, false],
            vec![true, false, true],
            vec![false, true, false]
        ];
        let mut board = Board::new(matrix);

        for _ in 0..4 {
            board.next_position();
        }

        let (x, y) = board.current_position.as_tuple();

        assert_eq!((x, y), (1, 1));

        assert_eq!(
            board.current_positions_neighbours(),
            vec![false, true, false, true, false, true, false, true]
        );

    }

    #[test]
    fn current_positions_returns_vec_when_y_and_x_eq_zero() {
        let matrix = vec![
            vec![false, true, false],
            vec![true, false, true],
            vec![false, true, false]
        ];
        let board = Board::new(matrix);

        let (x, y) = board.current_position.as_tuple();

        assert_eq!((x, y), (0, 0));

        assert_eq!(
            board.current_positions_neighbours(),
            vec![true, false, true]
        );
    }

    #[test]
    fn current_positions_returns_vec_when_y_is_one_and_x_is_zero() {
        let matrix = vec![
            vec![false, true, false],
            vec![true, false, true],
            vec![false, true, false]
        ];
        let mut board = Board::new(matrix);

        for _ in 0..3 {
            board.next_position();
        }

        let (x, y) = board.current_position.as_tuple();

        assert_eq!((x, y), (0, 1));

        assert_eq!(
            board.current_positions_neighbours(),
            vec![false, true, false, true, false]
        );
    }

    #[test]
    fn current_positions_returns_vec_when_y_is_two_and_x_is_two() {
        let matrix = vec![
            vec![false, true, false],
            vec![true, false, true],
            vec![false, true, false]
        ];
        let mut board = Board::new(matrix);

        for _ in 0..8 {
            board.next_position();
        }

        let (x, y) = board.current_position.as_tuple();

        assert_eq!((x, y), (2, 2));

        assert_eq!(
            board.current_positions_neighbours(),
            vec![false, true, true]
        );
    }
}

pub struct Game {
    current_board: Board
}

impl Game {

    pub fn new(matrix: Matrix) -> Game {
        Game {current_board: Board::new(matrix)}
    }

    pub fn board(&self) -> &Board {
        &self.current_board
    }
}

#[cfg(test)]
mod game_tests {
    use super::Game;

    #[test]
    fn new_works() {
        let matrix = vec![
            vec![false, true, false],
            vec![true, false, true],
            vec![false, true, false]
        ];

        let new_game = Game::new(matrix);

        let (x, y) = new_game.board().current_position.as_tuple();

        assert_eq!((x, y), (0, 0));
    }

    
}

