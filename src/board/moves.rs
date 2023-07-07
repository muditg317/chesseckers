use std::{error::Error, ops::Index};

use super::{Player, utils::BoardCoord, piece::Piece};



/**
 * move generation ideas
 * result type: 
 *   struct MoveData {
 *     player: Player,
 *     movements: [
 *       struct MovementEntry {
 *         from: BoardCoord
 *       }
 *     ]
 *   }
 */


#[derive(Debug)]
pub struct MoveData {
  player: Player,
  movements: Vec<MovementEntry>
}

impl MoveData {
  pub(super) fn new(player: Player) -> Self {
    Self { player, movements: Default::default() }
  }

  pub(super) fn add_moves(&mut self, moves: &mut Vec<MovementEntry>) {
    self.movements.append(moves);
  }
}

impl Index<usize> for MoveData {
  type Output = MovementEntry;

  fn index(&self, index: usize) -> &Self::Output {
    &self.movements[index]
  }
}

#[derive(Debug)]
pub struct MovementEntry {
  pub(super) from: BoardCoord,
  pub(super) movements: Vec<(BoardCoord,Option<BoardCoord>)> // Vec<(intermediate_dest, Option<intermediate_capture>)>
}

pub(super) trait Moveable {
  fn get_valid_moves<'a>(&self, from: BoardCoord, inspect_board: impl Fn(BoardCoord) -> Result<&'a Option<Box<Piece>>, Box<dyn Error>>) -> Vec<MovementEntry>;
}