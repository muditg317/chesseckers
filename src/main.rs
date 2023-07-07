mod board;

use std::{error::Error, thread::sleep, time::Duration};

use rand;

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
    let num_opts = moves.num_moves();
    if num_opts == 0 {
      break;
    }
    let choice = rand::random::<usize>() % num_opts;
    moves = board.exec_move(&moves[choice].clone())?;
    board.print();
    sleep(Duration::from_millis(700));
  }

  Ok(())
}
