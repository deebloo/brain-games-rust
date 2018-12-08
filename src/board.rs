use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq)]
pub enum BoardSpace {
    X,
    O,
    Blank,
}
impl Display for BoardSpace {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{}",
            match self {
                BoardSpace::Blank => " - ",
                BoardSpace::X => " X ",
                BoardSpace::O => " O ",
            }
        )
    }
}

#[derive(Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug)]
pub struct Board {
    spaces: Vec<BoardSpace>,
}
impl Board {
    pub fn empty() -> Board {
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

    pub fn set(&mut self, space: BoardSpace, pos: Position) {
        // cast i32 to ussize. could probably just use usize in the spaces
        let position = (pos.x * 3 + pos.y) as usize;

        if self.spaces[position] == BoardSpace::Blank {
            self.spaces[position] = space;
        }
    }

    pub fn print(&self) {
        let mut index = 0;

        for space in &self.spaces {
            if index % 3 == 0 {
                println!();
            }

            print!("{}", space);

            index = index + 1
        }

        println!();
    }
}
