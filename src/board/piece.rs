use std::fmt::{Debug, Display};

#[derive(Debug)]
pub(super) enum Piece {
  ChessPiece(ChessPiece),
  CheckersPiece(CheckersPiece)
}

#[derive(Debug)]
pub(super) enum ChessPiece {
  Pawn,
}

#[derive(Debug)]
pub(super) enum CheckersPiece {
  Stone,
  King
}

impl Display for Piece {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match *self {
      Self::ChessPiece(ref chess_piece) => write!(f, "C{}", chess_piece),
      Self::CheckersPiece(ref checkers_piece) => write!(f, "X{}", checkers_piece)
    }
  }
}

impl Display for ChessPiece {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", match *self {
      Self::Pawn => "p"
    })
  }
}

impl Display for CheckersPiece {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", match *self {
      Self::Stone => "o",
      Self::King => "O"
    })
  }
}