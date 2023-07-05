mod position;
mod piece;

use std::{ops::{Index, IndexMut}, error::Error, fmt::Display};

use crate::board::position::Position;

use self::piece::Piece;

#[derive(Debug)]
pub enum Player {
  Chess,
  Checkers
}

const BOARD_SIZE: usize = 8;

pub struct Board {
  size: usize,
  data: [[Position; BOARD_SIZE]; BOARD_SIZE] // (0,0) top left of board
  // piece_at: fn(r:usize,c:usize) -> Option<Box<dyn Piece>>
}

#[derive(Debug)]
pub struct MoveError {
  pub reason: String
}
impl Display for MoveError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "Move error: {}", self.reason)
  }
}
impl Error for MoveError {
}

impl Board {
  pub fn print(&self) -> () {
    for r in 0..self.size {
      for c in 0..self.size {
        print!("{}", self[(r,c)]);
      }
      println!();
    }
  }
  pub fn debug(&self) -> () {
    for r in 0..self.size {
      for c in 0..self.size {
        println!("{:?}", self[(r,c)]);
      }
    }
  }
  /**
   * reset the board state
   * first Turn player pieces on bottom
   */
  pub fn reset(&mut self, first: Player) -> Result<(), position::PieceCreationError> {
    for r in 0..self.size {
      for c in 0..self.size {
        self[(r,c)].clear();
      }
    }
    let (_chess_piece_row, chess_pawn_row) = match first {
      Player::Chess => (self.size-1, self.size-2),
      Player::Checkers => (0, 1)
    };
    for c in 0..self.size {
      self[(chess_pawn_row,c)].create_piece(Box::new(Piece::ChessPiece(piece::ChessPiece::Pawn)))?;
    }
    let (checkers_row_start, checkers_row_incr) = match first {
      Player::Chess => (0isize,1isize),
      Player::Checkers => ((self.size as isize)-1, -1)
    };
    for r in 0..3usize {
      let checkers_row = checkers_row_start + (r as isize)*checkers_row_incr;
      // let half_size = self.size/2;
      for c in 0..self.size/2 {
        let checkers_col = c*2 + 1-r%2;
        self[(checkers_row as usize, checkers_col)].create_piece(Box::new(Piece::CheckersPiece(piece::CheckersPiece::Stone)))?;
      }
    }
    Ok(())
  }

  pub fn make_move(&mut self, player: Player, from: (usize,usize), to: (usize,usize)) -> Result<(), MoveError> {
    {
      let from_pos = &self[from];
      if from_pos.piece.is_none() {
        return Err(MoveError { reason: format!("cannot move from {:?} -- no piece present", from) });
      }
      let piece = from_pos.piece.as_ref().unwrap();
      match (&player, piece.as_ref()) {
        (Player::Chess, Piece::CheckersPiece(_)) | (Player::Checkers, Piece::ChessPiece(_)) => return Err(MoveError { reason: format!("piece at {:?} doesn't belong to {:?} player", from, player) }),
        _ => ()
      }
    }
    {
      let to_pos = &self.data[to.0][to.1];
      if to_pos.is_empty() {
        let piece = self[from].piece.take().unwrap();
        self[from].clear();
        let _ = self[to].piece.insert(piece);
        return Ok(());
      }
    }
    return Err(MoveError { reason: format!("cannot move to {:?} -- piece", to) });
    // Ok(())
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

impl IndexMut<(usize,usize)> for Board {
  fn index_mut(&mut self, (r,c): (usize,usize)) -> &mut Self::Output {
    &mut self.data[r][c]
  }
}
