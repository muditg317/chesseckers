mod board;

use std::error::Error;


use crate::board::Player;

use crate::board::Board;

fn main() -> Result<(), Box<dyn Error + 'static>> {
  let mut board: Board = Default::default();
  board.reset(Player::Chess).expect("failed to reset board");
  // board.debug();
  board.print();
  board.make_move(Player::Chess, (6,0), (4,0))?;
  board.print();
  // board.make_move(Player::Chess, (5,0), (4,0))?;
  board.print();
  board.make_move(Player::Chess, (4,0), (3,0))?;
  board.print();
  board.make_move(Player::Chess, (3,0), (2,1))?;
  board.print();
  board.make_move(Player::Chess, (2,1), (1,0))?;
  board.print();
  Ok(())
}
