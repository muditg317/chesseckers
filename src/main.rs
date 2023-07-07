mod board;

use std::error::Error;


use crate::board::Player;

use crate::board::Board;

fn main() -> Result<(), Box<dyn Error + 'static>> {
  let mut board: Board = Default::default();
  board.reset(Player::Chess).expect("failed to reset board");
  // board.debug();
  board.print();
  // board.make_move(Player::Chess, (6,0), (4,0))?;
  // board.print();
  // board.make_move(Player::Checkers, (2,1), (3,0))?;
  // board.print();
  // board.make_move(Player::Chess, (6,1), (4,1))?;
  // board.print();
  // board.make_move(Player::Checkers, (3,0), (4,1))?;
  // board.print();
  // board.make_move(Player::Chess, (2,1), (1,0))?;
  // board.print();
  let moves = board.get_next_move_set();
  println!("next available moves: {:?}", moves);
  // board.exec_move(&moves[0]);

  Ok(())
}
