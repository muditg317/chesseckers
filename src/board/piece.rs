use std::{fmt::{Debug, Display}, error::Error};

#[derive(Debug)]
pub(super) enum Piece {
  // ChessPiece((usize,usize),ChessPiece),
  ChessPiece(ChessPiece),
  CheckersPiece(CheckersPiece)
}

#[derive(Debug)]
pub(super) struct PieceMovementError {
  pub(super) reason: String
}

impl Display for PieceMovementError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "PieceMovementError: {}", self.reason)
  }
}
impl Error for PieceMovementError {}
type MoveTestResult = Result<(), PieceMovementError>;

pub(super) trait PieceTrait<InitArgs> {
  fn new_piece(origin: (usize,usize), args: InitArgs) -> Piece;
  fn move_test(&self, from: (usize,usize), to: (usize,usize), to_take_opt: &Option<Box<Piece>>) -> MoveTestResult;
}

#[derive(Debug)]
pub(super) enum ChessPiece {
  Pawn {
    origin: (usize,usize),
  },
}

#[derive(Debug)]
pub(super) enum CheckersPiece {
  Stone {
    origin: (usize,usize),
  },
  King {
    origin: (usize,usize),
  }
}

impl Piece {
  pub(crate) fn move_test(&self, from: (usize, usize), to: (usize, usize), to_take_opt: &Option<Box<Piece>>) -> MoveTestResult {
    match *self {
      Piece::ChessPiece(ref chess_piece) => chess_piece.move_test(from, to, to_take_opt),
      Piece::CheckersPiece(ref checkers_piece) => checkers_piece.move_test(from, to, to_take_opt)
    }
  }
  pub(super) fn on_moved(&self, from: (usize,usize), to: (usize,usize)) {
    println!("{self} moved from {from:?} to {to:?}");
  }
  pub(super) fn on_taken(&self, death_loc: (usize,usize), by: &Box<Piece>) {
    println!("{self} at {death_loc:?} taken by {by}");
  }
}

impl PieceTrait<(&str,)> for ChessPiece {
  fn new_piece(origin: (usize,usize), (piece_type,): (&str,)) -> Piece {
    match piece_type {
      "pawn" => Piece::ChessPiece(ChessPiece::Pawn { origin }),
      _ => panic!("unkown piece type! got: {piece_type}")
    }
  }

  fn move_test(&self, from: (usize, usize), to: (usize, usize), to_take_opt: &Option<Box<Piece>>) -> MoveTestResult {
    match *self {
      Self::Pawn { origin } => {
        let dir = if origin.0 <= 2 { 1 } else { -1 };
        let rows_moved = to.0 as i32 - from.0 as i32;
        match to_take_opt {
          None => {
            if from.1 != to.1 {
              return Err(PieceMovementError{reason: format!("Pawn cannot change columns {} -> {}", from.1, to.1)});
            }
            match rows_moved {
              diff if diff == dir => return Ok(()),
              diff if from == origin && diff == dir*2 => return Ok(()),
              _ => return Err(PieceMovementError { reason: format!("Pawn cannot move {} spaces", from.0.abs_diff(to.0)) })
            }
          },
          Some(_) => {
            if rows_moved == dir && from.1.abs_diff(to.1) == 1 {
              return Ok(());
            } else {
              return Err(PieceMovementError { reason: format!("Pawn cannot capture enemy at {to:?}") });
            }
          }
        }
      }
    }
  }
}

impl PieceTrait<()> for CheckersPiece {
  fn new_piece(origin: (usize,usize), _args: ()) -> Piece {
    Piece::CheckersPiece(CheckersPiece::Stone { origin })
  }
  fn move_test(&self, from: (usize,usize), to: (usize,usize), to_take_opt: &Option<Box<Piece>>) -> MoveTestResult {
    // todo!("move for checkers not impl");
    // Ok(())
    match *self {
      Self::Stone { origin } => {
        let dir = if origin.0 <= 2 { 1 } else { -1 };
        let rows_moved = to.0 as i32 - from.0 as i32;
        let cols_moved = to.1 as i32 - from.1 as i32;
        if rows_moved == 0 || cols_moved == 0 {
          return Err(PieceMovementError { reason: format!("Stones must move diagonally forward by exactly 1 tile - not {} forward and {} lateral", from.0.abs_diff(to.0), from.1.abs_diff(to.1)) });
        }
        if (rows_moved < 0) != (dir < 0) {
          return Err(PieceMovementError { reason: format!("Stone cannot move backwards ({} spaces)", from.0.abs_diff(to.0)) });
        }
        if rows_moved != dir {
          return Err(PieceMovementError { reason: format!("Stone cannot move {} spaces forward", from.0.abs_diff(to.0)) });
        }
        if from.1.abs_diff(to.1) != 1 {
          return Err(PieceMovementError { reason: format!("Stone must move exactly 1 space left/right (tried {})", from.1.abs_diff(to.1)) })
        }
        Ok(())
      },
      Self::King { .. } => {
        let rows_moved = from.0.abs_diff(to.0);
        let cols_moved = from.1.abs_diff(to.1);
        if rows_moved == 0 || cols_moved == 0 {
          return Err(PieceMovementError { reason: format!("King must move diagonally by exactly 1 tile - not {} forward/backward and {} lateral", rows_moved, cols_moved) });
        }
        if rows_moved != 1 {
          return Err(PieceMovementError { reason: format!("King must move exactly 1 space forward/backward (tried {})", rows_moved) });
        }
        if cols_moved != 1 {
          return Err(PieceMovementError { reason: format!("Stone must move exactly 1 space left/right (tried {})", cols_moved) })
        }
        Ok(())
      }
    }
  }
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
      Self::Pawn { .. } => "p"
    })
  }
}

impl Display for CheckersPiece {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", match *self {
      Self::Stone { .. } => "o",
      Self::King { .. } => "O"
    })
  }
}