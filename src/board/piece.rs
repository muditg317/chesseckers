use std::{fmt::{Debug, Display}, error::Error};

use super::{utils::{BoardCoord, BoardCoordS}, moves::{Moveable, MovementEntry}, Player};

#[derive(Debug, PartialEq, Eq, Clone)]
pub(super) enum Piece {
  // ChessPiece(BoardCoord,ChessPiece),
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

pub(super) trait PieceTrait<InitArgs>: Moveable {
  fn new_piece(origin: BoardCoord, forward_dir: isize, args: InitArgs) -> Piece;
  fn get_forward_dir(&self) -> isize;
  // fn move_test<'a>(&self, from: BoardCoord, to: BoardCoord, inspect_board: impl Fn(BoardCoord) -> Result<&'a Option<Box<Piece>>, Box<dyn Error>>) -> MoveTestResult;
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub(super) enum ChessPiece {
  Pawn {
    origin: BoardCoord,
    forward_dir: isize
  },
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub(super) enum CheckersPiece {
  Stone {
    origin: BoardCoord,
    forward_dir: isize
  },
  King {
    origin: BoardCoord,
    forward_dir: isize
  }
}

impl Piece {
  // pub(super) fn move_test<'a>(&self, from: (usize, usize), to: (usize, usize), inspect_board: impl Fn(BoardCoord) -> Result<&'a Option<Box<Piece>>, Box<dyn Error>>) -> MoveTestResult {
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
  pub(super) fn on_moved(&self, _from: BoardCoord, _to: BoardCoord) {
    // println!("{self} moved from {from:?} to {to:?}");
  }
  pub(super) fn on_taken(&self, _death_loc: BoardCoord, _by: &Box<Piece>) {
    // println!("{self} at {death_loc:?} taken by {by}");
  }
}

impl PieceTrait<(&str,)> for ChessPiece {
  fn new_piece(origin: BoardCoord, forward_dir: isize, (piece_type,): (&str,)) -> Piece {
    match piece_type {
      "pawn" => Piece::ChessPiece(ChessPiece::Pawn { origin, forward_dir }),
      _ => panic!("unkown piece type! got: {piece_type}")
    }
  }

  fn get_forward_dir(&self) -> isize {
    match *self {
      Self::Pawn { forward_dir, .. } => forward_dir
    }
  }
}

impl PieceTrait<()> for CheckersPiece {
  fn new_piece(origin: BoardCoord, forward_dir: isize, _args: ()) -> Piece {
    Piece::CheckersPiece(CheckersPiece::Stone { origin, forward_dir })
  }
  fn get_forward_dir(&self) -> isize {
    match *self {
      Self::Stone { forward_dir, .. } | Self::King { forward_dir, .. } => forward_dir
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
  //       let forward_dir = if origin.0 <= 2 { 1 } else { -1 };
  //       let rows_moved = to.0 as i32 - from.0 as i32;
  //       let cols_moved = to.1 as i32 - from.1 as i32;
  //       if rows_moved == 0 || cols_moved == 0 {
  //         return Err(PieceMovementError { reason: format!("Stones must move diagonally forward by exactly 1 tile - not {} forward and {} lateral", from.0.abs_diff(to.0), from.1.abs_diff(to.1)) });
  //       }
  //       if (rows_moved < 0) != (forward_dir < 0) {
  //         return Err(PieceMovementError { reason: format!("Stone cannot move backwards {} spaces ({from:?} -> {to:?})", from.0.abs_diff(to.0)) });
  //       }
  //       if rows_moved != forward_dir {
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
    moves.push(MovementEntry { from, movements: vec![(target.into(),None)], promotion: None });
    return true;
  }
  return false;
}

fn add_capture_if_owned_by<'a>(moves: &mut Vec<MovementEntry>, from: BoardCoord, offset: (isize,isize), enemy: Player, inspect_board: &impl Fn(BoardCoord) -> Result<&'a Option<Box<Piece>>, Box<dyn Error>>) -> bool {
  let target = BoardCoordS::from(from) + offset;
  match inspect_board(target.into()).unwrap_or(&None) {
    Some(piece) if piece.get_owner() == enemy => {
      moves.push(MovementEntry { from, movements: vec![(target.into(),Some(target.into()))], promotion: None });
      return true;
    },
    Some(_) | None => false
  }
}

fn recursive_explore_checkers_captures<'a>(from: BoardCoord, valid_directions: &[isize], curr_moves: Vec<(BoardCoord,Option<BoardCoord>)>, inspect_board: &impl Fn(BoardCoord) -> Result<&'a Option<Box<Piece>>, Box<dyn Error>>) -> Vec<MovementEntry> {
  let mut result: Vec<MovementEntry> = Default::default();

  let start = BoardCoordS::from(if curr_moves.len() > 0 { curr_moves.last().unwrap().0 } else { from });

  let mut expanded = false;
  for forward_offset in valid_directions {
    for lateral_offset in [-1,1isize] {
      let offset = (*forward_offset, lateral_offset);
      let target = start + offset;
      match inspect_board(target.into()).unwrap_or(&None) {
        Some(piece) if piece.get_owner() == Player::Chess => {
          // moves.push(MovementEntry { from, movements: vec![(target.into(),Some(target.into()))] });
          // return true;
          let dst = target + offset;
          if inspect_board(dst.into()).is_ok_and(|opt_piece| opt_piece.is_none()) {
            let mut new_moves = curr_moves.clone();
            new_moves.push(((target+offset).into(), Some(target.into())));
            // println!("expand possible capture {{ start: {from:?}, moves: {new_moves:?} }}");
            // sleep(Duration::from_millis(500));
            let mut new_captures = recursive_explore_checkers_captures(from, valid_directions, new_moves, inspect_board);
            if !new_captures.is_empty() {
              result.append(&mut new_captures);
              expanded = true;
            }
          }
        },
        Some(_) | None => ()
      }
    }
  }
  if !expanded && curr_moves.len() > 0 {
    result.push(MovementEntry { from: from, movements: curr_moves, promotion: None })
  }

  result
}
fn get_checkers_captures<'a>(from: BoardCoord, valid_directions: &[isize], inspect_board: &impl Fn(BoardCoord) -> Result<&'a Option<Box<Piece>>, Box<dyn Error>>) -> Vec<MovementEntry> {
  recursive_explore_checkers_captures(from, valid_directions, Default::default(), inspect_board)
}
fn denote_checkers_promotions<'a>(moves: &mut Vec<MovementEntry>, origin: BoardCoord, forward_dir: isize, inspect_board: &impl Fn(BoardCoord) -> Result<&'a Option<Box<Piece>>, Box<dyn Error>>) {
  for movement in moves {
    if inspect_board((BoardCoordS::from(movement.end_point()) + (forward_dir, 0)).into()).is_err() { // moving anymore forward from end point is on edge
      // TODO: refactor above if statement to helper function

      if movement.promotion.is_none() {
        let _ = movement.promotion.insert((
          movement.end_point(),
          Box::new(Piece::CheckersPiece(CheckersPiece::King { origin, forward_dir }))
        ));
      }

    }
  }
}

impl Moveable for ChessPiece {
  fn get_valid_moves<'a>(&self, from: BoardCoord, inspect_board: impl Fn(BoardCoord) -> Result<&'a Option<Box<Piece>>, Box<dyn Error>>) -> Vec<MovementEntry> {
    let mut result: Vec<MovementEntry> = Default::default();
    match *self {
      Self::Pawn { origin, forward_dir } => {
        let can_step = add_direct_movement_if_free(&mut result, from, (forward_dir,0), &inspect_board);
        if can_step && from == origin {
          add_direct_movement_if_free(&mut result, from, (forward_dir*2,0), &inspect_board);
        }
        add_capture_if_owned_by(&mut result, from, (forward_dir, 1), Player::Checkers, &inspect_board);
        add_capture_if_owned_by(&mut result, from, (forward_dir,-1), Player::Checkers, &inspect_board);
      }
    }
    result
  }
}
impl Moveable for CheckersPiece {
  fn get_valid_moves<'a>(&self, from: BoardCoord, inspect_board: impl Fn(BoardCoord) -> Result<&'a Option<Box<Piece>>, Box<dyn Error>>) -> Vec<MovementEntry> {
    let forward_dir = self.get_forward_dir();
    let mut result: Vec<MovementEntry> = Default::default();
    // stone moves (added for king as well)
    add_direct_movement_if_free(&mut result, from, (forward_dir,1), &inspect_board);
    add_direct_movement_if_free(&mut result, from, (forward_dir,-1), &inspect_board);
    result.append(&mut get_checkers_captures(from, &[forward_dir], &inspect_board));
    match *self {
      Self::King { .. } => {
        // vec![]
      },
      Self::Stone { origin, forward_dir } => {
        denote_checkers_promotions(&mut result, origin, forward_dir, &inspect_board);
      }
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