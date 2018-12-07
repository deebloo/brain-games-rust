// use std::io::{stdin};

#[derive(Debug, PartialEq)]
enum BoardSpace {
    X,
    Y,
    Blank,
}

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Board {
    spaces: Vec<BoardSpace>,
}

impl Board {
    fn empty() -> Board {
        Board {
            spaces: vec![
                BoardSpace::Blank,
                BoardSpace::Blank,
                BoardSpace::Blank,
                BoardSpace::Blank,
                BoardSpace::Blank,
                BoardSpace::Blank,
                BoardSpace::Blank,
                BoardSpace::Blank,
                BoardSpace::Blank,
            ],
        }
    }

    fn set(&mut self, space: BoardSpace, point: Position) {
        // cast i32 to ussize. could probably just use usize in the spaces
        let position = (point.x * 3 + point.y) as usize;

        if self.spaces[position] == BoardSpace::Blank {
            self.spaces[position] = space;
        }
    }

    // TODO: print actual representation of board
    fn print(&self) {
        println!("{:?}", &self.spaces[0..=2]);
        println!("{:?}", &self.spaces[3..=5]);
        println!("{:?}", &self.spaces[6..=8]);
    }
}

struct Bot {
    player_space: BoardSpace,
}

impl Bot {
    fn new(player_space: BoardSpace) -> Bot {
        Bot { player_space }
    }

    fn make_move(self, board: &mut Board, point: Position) {
        board.set(self.player_space, point)
    }
}

fn main() {
    let mut board: Board = Board::empty();
    let bot = Bot::new(BoardSpace::X);

    bot.make_move(&mut board, Position { x: 0, y: 1 });

    board.set(BoardSpace::Y, Position { x: 0, y: 0 });

    board.print();
}
