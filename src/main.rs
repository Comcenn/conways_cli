
type Board = [[bool; 5]; 6];

const INITIAL_BOARD: Board = [
    [false, false, false, false, false],
    [false, true, false, false, false],
    [false, false, false, false, false],
    [false, false, true, true, false],
    [false, false, false, false, false],
    [false, false, false, false, false],
];

struct BoardCrawler {
    x: i32,
    y: i32,
    board: Board
}

impl BoardCrawler {
    fn new(initial_board: Board) -> Self{
        Self {
            x:0,
            y:0,
            board:initial_board
        }
    }
}

fn main() {
    let crawler = BoardCrawler::new(INITIAL_BOARD);
}
