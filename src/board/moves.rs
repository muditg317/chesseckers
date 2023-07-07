use std::{error::Error, ops::Index};

use super::{Player, utils::BoardCoord, piece::Piece};

#[derive(Debug, Default, Clone)]
pub struct MoveData {
  pub(super) player: Player,
  movements: Vec<MovementEntry>,
  promotion: Option<(BoardCoord, fn () -> Box<Piece>)>
  // TODO: add promotions (from,Box<Piece>)
}

impl MoveData {
  pub(super) fn new(player: Player) -> Self {
    Self { player, movements: Default::default(), promotion: None }
  }

  pub(super) fn add_moves(&mut self, moves: &mut Vec<MovementEntry>) {
    self.movements.append(moves);
  }

  pub(super) fn contains(&self, movement: &MovementEntry) -> bool {
    self.movements.contains(movement)
  }
}

impl Index<usize> for MoveData {
  type Output = MovementEntry;

  fn index(&self, index: usize) -> &Self::Output {
    &self.movements[index]
  }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MovementEntry {
  pub(super) from: BoardCoord,
  pub(super) movements: Vec<(BoardCoord,Option<BoardCoord>)> // Vec<(intermediate_dest, Option<intermediate_capture>)>
}

pub(super) trait Moveable {
  fn get_valid_moves<'a>(&self, from: BoardCoord, inspect_board: impl Fn(BoardCoord) -> Result<&'a Option<Box<Piece>>, Box<dyn Error>>) -> Vec<MovementEntry>;
}