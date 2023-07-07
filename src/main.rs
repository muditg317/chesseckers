mod board;

use std::{error::Error, thread::sleep, time::Duration};


use crate::board::{Player,Board,MoveData};

fn main() -> Result<(), Box<dyn Error + 'static>> {
  let mut board: Board = Default::default();
  // TODO: add "render" callback to board
  // let mut moves = &mut &(Default::default());
  let mut moves: MoveData = board.reset(Player::Chess).expect("failed to reset board");//.clone();
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
  loop {
    // println!("next available moves: {:?}", moves);
    moves = board.exec_move(&moves[0].clone())?;
    board.print();
    sleep(Duration::from_millis(700));
  }

  // Ok(())
}
