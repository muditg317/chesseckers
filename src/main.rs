mod board;

use std::error::Error;

use board::Player;

use crate::board::Board;

fn main() -> Result<(), Box<dyn Error + 'static>> {
  let mut board: Board = Default::default();
  board.reset(board::Player::Chess).expect("failed to reset board");
  // board.debug();
  board.print();
  board.make_move(Player::Chess, (6,0), (5,0))?;
  board.print();
  Ok(())
}
