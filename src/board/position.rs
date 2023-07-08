use super::piece::Piece;


use std::fmt::Display;

use super::Player;

#[derive(Debug)]
pub struct Position {
  // r: usize,
  // c: usize,
  pub(super) piece: Option<Box<Piece>>
}

#[derive(Debug)]
pub struct PieceCreationError;

impl Position {
  pub(super) fn new() -> Self {
    Position { piece: None }
  }
  pub(super) fn clear(&mut self) {
    self.piece.take();
  }
  pub(super) fn create_piece(&mut self, piece: Box<Piece>) -> Result<(), PieceCreationError> {
    if self.piece.is_some() {
      return Err(PieceCreationError);
    }
    let _ = self.piece.insert(piece);
    Ok(())
  }
  pub(super) fn remove(&mut self) -> Box<Piece> {
    self.piece.take().unwrap()
  }
  // pub(super) fn set(&mut self, piece: Box<Piece>) -> &Box<Piece> {
  //   self.piece.insert(piece)
  // }
  pub(super) fn replace(&mut self, piece: Box<Piece>) -> Option<Box<Piece>> {
    self.piece.replace(piece)
  }
  pub(super) const fn is_empty(&self) -> bool {
    self.piece.is_none()
  }
  pub(super) fn piece_ref(&self) -> &Box<Piece> {
    self.piece.as_ref().unwrap()
  }
  pub(super) fn piece_owner(&self) -> Player {
    match *self.piece.as_ref().unwrap().as_ref() {
      Piece::ChessPiece(_) => Player::Chess,
      Piece::CheckersPiece(_) => Player::Checkers
    }
  }
  pub(super) fn owned_by(&self, player: Player) -> bool {
    self.piece_owner() == player
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