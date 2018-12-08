use board::{Board, BoardSpace, Position};

pub struct Player {
    player_space: BoardSpace,
}
impl Player {
    pub fn new(player_space: BoardSpace) -> Player {
        Player { player_space }
    }

    pub fn make_move(self, board: &mut Board, post: Position) {
        board.set(self.player_space, post)
    }
}
