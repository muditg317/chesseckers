use std::{fmt::{Debug, Display}, error::Error};

use super::{utils::{BoardCoord, BoardCoordS}, moves::{Moveable, MovementEntry}, Player};

#[derive(Debug)]
pub(crate) enum Piece {
  // ChessPiece(BoardCoord,ChessPiece),
  ChessPiece(ChessPiece),
  CheckersPiece(CheckersPiece)
}

#[derive(Debug)]
pub(crate) struct PieceMovementError {
  pub(crate) reason: String
}

impl Display for PieceMovementError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "PieceMovementError: {}", self.reason)
  }
}
impl Error for PieceMovementError {}
pub(crate) struct MoveResult {

}
type MoveTestResult = Result<(), PieceMovementError>; // optional coords of piece to capture

// type BoardInspectFn = impl Fn(BoardCoord) -> &'static Option<Box<Piece>>;

pub(crate) trait PieceTrait<InitArgs>: Moveable {
  fn new_piece(origin: BoardCoord, args: InitArgs) -> Piece;
  fn get_forward_dir(&self) -> isize;
  // fn move_test<'a>(&self, from: BoardCoord, to: BoardCoord, inspect_board: impl Fn(BoardCoord) -> Result<&'a Option<Box<Piece>>, Box<dyn Error>>) -> MoveTestResult;
}

#[derive(Debug)]
pub(crate) enum ChessPiece {
  Pawn {
    origin: BoardCoord,
  },
}

#[derive(Debug)]
pub(crate) enum CheckersPiece {
  Stone {
    origin: BoardCoord,
  },
  King {
    origin: BoardCoord,
  }
}

impl Piece {
  // pub(crate) fn move_test<'a>(&self, from: (usize, usize), to: (usize, usize), inspect_board: impl Fn(BoardCoord) -> Result<&'a Option<Box<Piece>>, Box<dyn Error>>) -> MoveTestResult {
  //   match *self {
  //     Piece::ChessPiece(ref chess_piece) => chess_piece.move_test(from, to, inspect_board),
  //     Piece::CheckersPiece(ref checkers_piece) => checkers_piece.move_test(from, to, inspect_board)
  //   }
  // }
  fn get_owner(&self) -> Player {
    match *self {
      Self::ChessPiece(_) => Player::Chess,
      Self::CheckersPiece(_) => Player::Checkers
    }
  }
  pub(crate) fn on_moved(&self, from: BoardCoord, to: BoardCoord) {
    println!("{self} moved from {from:?} to {to:?}");
  }
  pub(crate) fn on_taken(&self, death_loc: BoardCoord, by: &Box<Piece>) {
    println!("{self} at {death_loc:?} taken by {by}");
  }
}

impl PieceTrait<(&str,)> for ChessPiece {
  fn new_piece(origin: BoardCoord, (piece_type,): (&str,)) -> Piece {
    match piece_type {
      "pawn" => Piece::ChessPiece(ChessPiece::Pawn { origin }),
      _ => panic!("unkown piece type! got: {piece_type}")
    }
  }

  fn get_forward_dir(&self) -> isize {
    match *self {
      Self::Pawn { origin } => if origin.0 <= 2 { 1 } else { -1 }
    }
  }

  // fn move_test<'a>(&self, from: (usize, usize), to: (usize, usize), inspect_board: impl Fn(BoardCoord) -> Result<&'a Option<Box<Piece>>, Box<dyn Error>>) -> MoveTestResult {
  //   let piece_at_dest = inspect_board(to);
  //   match *self {
  //     Self::Pawn { origin } => {
  //       let dir = if origin.0 <= 2 { 1 } else { -1 };
  //       let rows_moved = to.0 as i32 - from.0 as i32;
  //       match piece_at_dest {
  //         None => {
  //           if from.1 != to.1 {
  //             return Err(PieceMovementError{reason: format!("Pawn cannot change columns {} -> {}", from.1, to.1)});
  //           }
  //           match rows_moved {
  //             diff if diff == dir => return Ok(()),
  //             diff if from == origin && diff == dir*2 => return Ok(()),
  //             _ => return Err(PieceMovementError { reason: format!("Pawn cannot move {} spaces", from.0.abs_diff(to.0)) })
  //           }
  //         },
  //         Some(_) => {
  //           if rows_moved == dir && from.1.abs_diff(to.1) == 1 {
  //             return Ok(());
  //           } else {
  //             return Err(PieceMovementError { reason: format!("Pawn cannot capture enemy at {to:?}") });
  //           }
  //         }
  //       }
  //     }
  //   }
  // }
}

impl PieceTrait<()> for CheckersPiece {
  fn new_piece(origin: BoardCoord, _args: ()) -> Piece {
    Piece::CheckersPiece(CheckersPiece::Stone { origin })
  }

  fn get_forward_dir(&self) -> isize {
      match *self {
        Self::Stone { origin } | Self::King { origin } => if origin.0 <= 2 { 1 } else { -1 }
      }
  }
  
  // fn move_test<'a>(&self, from: BoardCoord, to: BoardCoord, inspect_board: impl Fn(BoardCoord) -> Result<&'a Option<Box<Piece>>, Box<dyn Error>>) -> MoveTestResult {
  //   let piece_at_dest = inspect_board(to);
  //   match piece_at_dest {
  //     Some(piece) => return Err(PieceMovementError { reason: format!("Checkers piece cannot move onto enemy {piece:?} at {to:?}, must jump over") }),
  //     None => ()
  //   }
  //   match *self {
  //     Self::Stone { origin } => {
  //       let dir = if origin.0 <= 2 { 1 } else { -1 };
  //       let rows_moved = to.0 as i32 - from.0 as i32;
  //       let cols_moved = to.1 as i32 - from.1 as i32;
  //       if rows_moved == 0 || cols_moved == 0 {
  //         return Err(PieceMovementError { reason: format!("Stones must move diagonally forward by exactly 1 tile - not {} forward and {} lateral", from.0.abs_diff(to.0), from.1.abs_diff(to.1)) });
  //       }
  //       if (rows_moved < 0) != (dir < 0) {
  //         return Err(PieceMovementError { reason: format!("Stone cannot move backwards {} spaces ({from:?} -> {to:?})", from.0.abs_diff(to.0)) });
  //       }
  //       if rows_moved != dir {
  //         return Err(PieceMovementError { reason: format!("Stone cannot move {} spaces forward ({from:?} -> {to:?})", from.0.abs_diff(to.0)) });
  //       }
  //       if from.1.abs_diff(to.1) != 1 {
  //         return Err(PieceMovementError { reason: format!("Stone must move exactly 1 space left/right, not {} ({from:?} -> {to:?})", from.1.abs_diff(to.1)) })
  //       }
  //       Ok(())
  //     },
  //     Self::King { .. } => {
  //       let rows_moved = from.0.abs_diff(to.0);
  //       let cols_moved = from.1.abs_diff(to.1);
  //       if rows_moved == 0 || cols_moved == 0 {
  //         return Err(PieceMovementError { reason: format!("King must move diagonally by exactly 1 tile - not {} forward/backward and {} lateral", rows_moved, cols_moved) });
  //       }
  //       if rows_moved != 1 {
  //         return Err(PieceMovementError { reason: format!("King must move exactly 1 space forward/backward (tried {})", rows_moved) });
  //       }
  //       if cols_moved != 1 {
  //         return Err(PieceMovementError { reason: format!("Stone must move exactly 1 space left/right (tried {})", cols_moved) })
  //       }
  //       Ok(())
  //     }
  //   }
  // }
}

impl Moveable for Piece {
  fn get_valid_moves<'a>(&self, from: BoardCoord, inspect_board: impl Fn(BoardCoord) -> Result<&'a Option<Box<Piece>>, Box<dyn Error>>) -> Vec<MovementEntry> {
    match self {
      Self::ChessPiece(chess_piece) => chess_piece.get_valid_moves(from, inspect_board),
      Self::CheckersPiece(checkers_piece) => checkers_piece.get_valid_moves(from, inspect_board)
    }
  }
}


fn add_direct_movement_if_free<'a>(moves: &mut Vec<MovementEntry>, from: BoardCoord, offset: (isize,isize), inspect_board: &impl Fn(BoardCoord) -> Result<&'a Option<Box<Piece>>, Box<dyn Error>>) -> bool {
  let target = BoardCoordS::from(from) + offset;
  if inspect_board(target.into()).is_ok_and(Option::is_none) {
    moves.push(MovementEntry { from, movements: vec![(target.into(),None)] });
    return true;
  }
  return false;
}

fn add_capture_if_owned_by<'a>(moves: &mut Vec<MovementEntry>, from: BoardCoord, offset: (isize,isize), enemy: Player, inspect_board: &impl Fn(BoardCoord) -> Result<&'a Option<Box<Piece>>, Box<dyn Error>>) -> bool {
  let target = BoardCoordS::from(from) + offset;
  match inspect_board(target.into()).unwrap_or(&None) {
    Some(piece) if piece.get_owner() == enemy => {
      moves.push(MovementEntry { from, movements: vec![(target.into(),Some(target.into()))] });
      return true;
    },
    Some(_) | None => false
  }

}

impl Moveable for ChessPiece {
  fn get_valid_moves<'a>(&self, from: BoardCoord, inspect_board: impl Fn(BoardCoord) -> Result<&'a Option<Box<Piece>>, Box<dyn Error>>) -> Vec<MovementEntry> {
    let dir = self.get_forward_dir();
    let mut result: Vec<MovementEntry> = Default::default();
    match *self {
      Self::Pawn { origin } => {
        let can_step = add_direct_movement_if_free(&mut result, from, (dir,0), &inspect_board);
        if can_step && from == origin {
          add_direct_movement_if_free(&mut result, from, (dir*2,0), &inspect_board);
        }
        add_capture_if_owned_by(&mut result, from, (dir, 1), Player::Checkers, &inspect_board);
        add_capture_if_owned_by(&mut result, from, (dir,-1), Player::Checkers, &inspect_board);
      }
    }
    result
  }
}
impl Moveable for CheckersPiece {
  fn get_valid_moves<'a>(&self, from: BoardCoord, inspect_board: impl Fn(BoardCoord) -> Result<&'a Option<Box<Piece>>, Box<dyn Error>>) -> Vec<MovementEntry> {
    let dir = self.get_forward_dir();
    let mut result: Vec<MovementEntry> = Default::default();
    // stone moves (added for king as well)
    add_direct_movement_if_free(&mut result, from, (dir,1), &inspect_board);
    add_direct_movement_if_free(&mut result, from, (dir,-1), &inspect_board);
    match *self {
      Self::King { origin } => {
        // vec![]
      },
      _ => ()
    }
    result
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