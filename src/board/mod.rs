mod position;
mod piece;

use std::ops::Index;

use crate::board::position::Position;


const BOARD_SIZE: usize = 8;

pub struct Board {
  size: usize,
  data: [[Position; BOARD_SIZE]; BOARD_SIZE]
  // piece_at: fn(r:usize,c:usize) -> Option<Box<dyn Piece>>
}

impl Board {
  pub fn print(&self) -> () {
    for r in 0..self.size {
      for c in 0..self.size {
        println!("{}", self[(r,c)]);
      }
    }
  }
}

impl Default for Board {
  fn default() -> Self {
    Board {
      size: BOARD_SIZE,
      data: std::array::from_fn::<_,BOARD_SIZE,_>(|r| std::array::from_fn::<_,BOARD_SIZE,_>(|c| Position::new(r,c)))
    }
  }
}

impl Index<(usize,usize)> for Board {
  type Output = Position;

  fn index(&self, (r,c): (usize,usize)) -> &Self::Output {
    &self.data[r][c]
  }
}
