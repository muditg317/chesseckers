use crate::board::piece::Piece;


use std::fmt::Display;

#[derive(Debug)]
pub struct Position {
  r: usize,
  c: usize,
  pub(super) piece: Option<Box<Piece>>
}

#[derive(Debug)]
pub struct PieceCreationError;

impl Position {
  pub(super) fn new(r:usize,c:usize) -> Self {
    Position { r, c, piece: None }
  }
  pub(super) fn clear(&mut self) -> () {
    self.piece.take();
  }
  pub(super) fn create_piece(&mut self, piece: Box<Piece>) -> Result<(), PieceCreationError> {
    if self.piece.is_some() {
      return Err(PieceCreationError);
    }
    let _ = self.piece.insert(piece);
    Ok(())
  }
  pub(super) const fn is_empty(&self) -> bool {
    self.piece.is_none()
  }
}

impl Display for Position {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self.piece {
      None => write!(f, "  "),
      Some(ref piece_box) => write!(f, "{}", piece_box)
    }
  }
}