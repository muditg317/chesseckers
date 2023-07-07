mod position;
mod piece;
mod player;

use std::{ops::{Index, IndexMut}, error::Error, fmt::Display};

use crate::board::position::Position;

pub use self::player::Player;

use self::{piece::{Piece, ChessPiece, PieceTrait, CheckersPiece}};

const BOARD_SIZE: usize = 8;

pub struct Board {
  size: usize,
  data: [[Position; BOARD_SIZE]; BOARD_SIZE], // (0,0) top left of board
  next_turn_player: Player
}

#[derive(Debug)]
pub struct MoveError {
  pub reason: String
  // pub cause: option_env!()
}
impl Display for MoveError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "MoveError: {}", self.reason)
  }
}
impl Error for MoveError {}

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
    self.next_turn_player = first;
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
      self[(chess_pawn_row,c)].create_piece(Box::new(ChessPiece::new_piece((chess_pawn_row,c), ("pawn",))))?;
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
        self[(checkers_row as usize, checkers_col)].create_piece(Box::new(CheckersPiece::new_piece((checkers_row as usize, checkers_col), ())))?;
      }
    }
    Ok(())
  }

  fn move_helper(&self, player: Player, from: (usize,usize), to: (usize,usize)) -> Result<&Box<Piece>, MoveError> {
    if player != self.next_turn_player {
      return Err(MoveError { reason: format!("not {player:?} player's turn") });
    }

    if !(0..self.size).contains(&from.0) || !(0..self.size).contains(&from.1) {
      return Err(MoveError { reason: format!("cannot move from {from:?}  -- not on board!") });
    }
    if !(0..self.size).contains(&to.0) || !(0..self.size).contains(&to.1) {
      return Err(MoveError { reason: format!("cannot move to {to:?}  -- not on board!") });
    }

    if from == to {
      return Err(MoveError { reason: format!("cannot move to same place! ({from:?} -> {to:?})") });
    }
    
    if self[from].is_empty() {
      return Err(MoveError { reason: format!("cannot move from {from:?} -- no piece present") });
    }
    if !self[from].owned_by(player) {
      return Err(MoveError { reason: format!("piece at {from:?} doesn't belong to {player:?} player") });
    }
    
    if !self[to].is_empty() && self[to].owned_by(player) {
      return Err(MoveError { reason: format!("cannot capture {} piece at {to:?} -- {player:?} player already owns piece", self[to]) });
    }
    
    Ok(self[from].piece_ref())
  }

  pub fn make_move(&mut self, player: Player, from: (usize,usize), to: (usize,usize)) -> Result<(), Box<dyn Error>> {
    self.move_helper(player, from, to)?.move_test(from, to, &|coords: (usize,usize)| {
      &self[coords].piece
    })?; // self::Index::index

    let moved_piece = self[from].remove();
    self[to].replace(moved_piece).and_then(|mut taken| {
      taken.as_mut().on_taken(to, self[to].piece_ref());
      Some(())
    });
    self[to].piece_ref().on_moved(from, to);
    self.next_turn_player = match self.next_turn_player {
      Player::Chess => Player::Checkers,
      Player::Checkers => Player::Chess
    };
    Ok(())
  }

}

impl Default for Board {
  fn default() -> Self {
    Board {
      size: BOARD_SIZE,
      data: std::array::from_fn::<_,BOARD_SIZE,_>(|r| std::array::from_fn::<_,BOARD_SIZE,_>(|c| Position::new(r,c))),
      next_turn_player: Player::Chess
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
