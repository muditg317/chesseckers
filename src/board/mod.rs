mod position;
mod piece;
mod player;
mod moves;
mod utils;

use std::{ops::{Index, IndexMut}, error::Error, fmt::Display};

pub use self::{player::Player, moves::{MoveData, MovementEntry}};

use self::{position::Position, piece::{Piece, ChessPiece, PieceTrait, CheckersPiece}, utils::BoardCoord, moves::Moveable};

const BOARD_SIZE: usize = 8;

pub struct Board {
  size: usize,
  data: [[Position; BOARD_SIZE]; BOARD_SIZE], // (0,0) top left of board
  next_moves: MoveData,
  render_callback: Option<Box<dyn Fn(&Self)>>
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
  pub fn set_render_callback(&mut self, cb: impl Fn(&Self) + 'static) {
    // let _ = self.render_callback.insert(Box::new(cb));
    self.render_callback = Some(Box::new(cb));
  }
  fn render(&self) {
    self.render_callback.as_ref().and_then(|cb| Some(cb ( self)));
  }
  /**
   * reset the board state
   * first Turn player pieces on bottom
   */
  pub fn reset(&mut self, first: Player) -> Result<MoveData, position::PieceCreationError> {
    let chess_dir = match first {
      Player::Chess => -1,
      Player::Checkers => 1
    };
    let checkers_dir = chess_dir * -1;
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
      self[(chess_pawn_row,c)].create_piece(Box::new(ChessPiece::new_piece((chess_pawn_row,c), chess_dir, ("pawn",))))?;
      self[(chess_pawn_row-2,c)].create_piece(Box::new(ChessPiece::new_piece((chess_pawn_row,c), chess_dir, ("pawn",))))?;
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
        self[(checkers_row as usize, checkers_col)].create_piece(Box::new(CheckersPiece::new_piece((checkers_row as usize, checkers_col), checkers_dir, ())))?;
      }
    }
    self.render();

    Ok(self.update_next_move_set(first))
  }

  fn on_board(&self, coords: BoardCoord) -> bool {
    // (0..self.size).contains(&coords.0) && (0..self.size).contains(&coords.1)
    // (0 <= coords.0) && (0 <= coords.1)
    (coords.0 < self.size) && (coords.1 < self.size)
  }
  fn inspect(&self, coords: BoardCoord) -> Result<&Option<Box<Piece>>, Box<dyn Error>> {

    if self.on_board(coords) { Ok(&self[coords].piece) } else { Err(MoveError { reason: String::from("out of bounds!") }.into()) }
  }

  fn update_next_move_set(&mut self, player: Player) -> MoveData {
    self.next_moves = MoveData::new(player);
    for r in 0..self.size {
      for c in 0..self.size {
        let pos = (r,c);
        if !self[pos].is_empty() && self[pos].owned_by(player) {
          self.next_moves.add_moves(&mut self[pos].piece_ref().as_ref().get_valid_moves(pos, |coords| {
            self.inspect(coords)
          }));
        }
      }
    }
    if player == Player::Checkers {
      self.next_moves.filter_captures_if_present();
    }
    self.next_moves.clone()
  }
  pub fn exec_move(&mut self, chosen_move: &MovementEntry) -> Result<MoveData, Box<dyn Error>> {
    if !self.next_moves.contains(chosen_move) {
      return Err(MoveError { reason: format!("chosen move not valid! tried: {chosen_move:?} - options: {:?}", self.next_moves) }.into());
    }


    
    // let mut moved_piece = self[chosen_move.from].remove();
    let mut final_pos = chosen_move.from;
    for (dst, opt_capture) in &chosen_move.movements {
      // TODO: call render callback after each frame of movement
      opt_capture.and_then(|capture_loc| {
        self[capture_loc].remove().as_ref().on_taken(capture_loc, self[final_pos].piece_ref());
        Some(())
      });
      let moved_piece = self[final_pos].remove();
      let _ = self[*dst].replace(moved_piece);
      final_pos = *dst;
      self.render();
      // moved_piece = self[final_pos].remove();
    }

    // let moved_piece = self[from].remove();
    // self[to].replace(moved_piece).and_then(|mut taken| {
    //   taken.as_mut().on_taken(to, self[to].piece_ref());
    //   Some(())
    // });
    // self[to].piece_ref().on_moved(from, to);
    self[final_pos].piece_ref().on_moved(chosen_move.from, final_pos);

    chosen_move.promotion.as_ref().and_then(|(loc, piece)| {
      // println!()
      self[*loc].replace(piece.clone());
      self.render();
      Some(())
    });

    Ok(self.update_next_move_set(self.next_moves.player.other()))
  }
}

impl Default for Board {
  fn default() -> Self {
    Board {
      size: BOARD_SIZE,
      data: std::array::from_fn::<_,BOARD_SIZE,_>(|r| std::array::from_fn::<_,BOARD_SIZE,_>(|c| Position::new(r,c))),
      next_moves: Default::default(),
      render_callback: None
    }
  }
}

impl Index<BoardCoord> for Board {
  type Output = Position;

  fn index(&self, (r,c): BoardCoord) -> &Self::Output {
    &self.data[r][c]
  }
}

impl IndexMut<BoardCoord> for Board {
  fn index_mut(&mut self, (r,c): BoardCoord) -> &mut Self::Output {
    &mut self.data[r][c]
  }
}
