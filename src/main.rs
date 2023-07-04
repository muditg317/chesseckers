mod board;

use crate::board::Board;

fn main() {
  let mut board: Board = Default::default();
  board.print();
}
