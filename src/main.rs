mod board;

use std::{error::Error, thread::sleep, time::Duration, fs};

use itertools::Itertools;
use rand;

use crate::board::{Player,Board,MoveData};

fn _test() -> Result<(), Box<dyn Error>> {
  let board_data = fs::read_to_string("test.txt")?;
  for board_stat in &board_data.split("\n").chunks(8) {
    for line in board_stat {
      println!("{line}");
    }
    println!();
    sleep(Duration::from_millis(250));
  }
  Ok(())
}

fn main() -> Result<(), Box<dyn Error + 'static>> {
  // test()?;
  let mut board: Board = Default::default();
  // TODO: add "render" callback to board
  board.set_render_callback(|board| {
    sleep(Duration::from_millis(750));
    println!();
    board.print();
  });
  // let mut moves = &mut &(Default::default());
  let mut moves: MoveData = board.reset(Player::Chess).expect("failed to reset board");//.clone();
  // board.debug();
  // board.print();
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
    println!("\n\n");
    sleep(Duration::from_millis(2000));
    let num_opts = moves.num_moves();
    if num_opts == 0 {
      break;
    }
    let choice = rand::random::<usize>() % num_opts;
    moves = board.exec_move(&moves[choice].clone())?;

    // board.print();
    // if moves.player == Player::Chess { break; }
  }

  Ok(())
}
