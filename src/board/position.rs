use crate::board::piece::Piece;


use std::fmt::Display;

// #[derive(Debug)]
pub struct Position {
  r: usize,
  c: usize,
  piece: Option<Box<dyn Piece>>
}

impl Position {
  pub(super) fn new(r:usize,c:usize) -> Self {
    Position { r, c, piece: None }
  }
}

impl Display for Position {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "<{r: >2},{c: >2}>: {piece:#?}", r=self.r, c=self.c, piece=self.piece)
  }
}