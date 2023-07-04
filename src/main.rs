use std::{ops::Index, fmt::{Display, Debug}};


const BOARD_SIZE: usize = 8;

trait Piece {

}

impl Debug for dyn Piece {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "piece")
  }
}

// #[derive(Debug)]
struct Position {
  r: usize,
  c: usize,
  piece: Option<Box<dyn Piece>>
}

impl Position {
  fn new(r:usize,c:usize) -> Self {
    Position { r, c, piece: None }
  }
}

impl Display for Position {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "<{r: >2},{c: >2}>: {piece:#?}", r=self.r, c=self.c, piece=self.piece)
  }
}

// type Board<'a> = &'a [&'a [Position;BOARD_SIZE];BOARD_SIZE];
struct Board {
  size: usize,
  data: [[Position; BOARD_SIZE]; BOARD_SIZE]
  // piece_at: fn(r:usize,c:usize) -> Option<Box<dyn Piece>>
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


fn main() {
  let mut board: Board = Default::default();
  for r in 0..board.size {
    for c in 0..board.size {
      println!("{}", board[(r,c)]);
    }
  }

}
