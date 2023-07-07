use std::{error::Error, ops::Index};

use super::{Player, utils::BoardCoord, piece::Piece};

#[derive(Debug, Default, Clone)]
pub struct MoveData {
  pub(crate) player: Player,
  movements: Vec<MovementEntry>
}

impl MoveData {
  pub(super) fn new(player: Player) -> Self {
    Self { player, movements: Default::default() }
  }

  pub(super) fn add_moves(&mut self, moves: &mut Vec<MovementEntry>) {
    self.movements.append(moves);
  }

  pub(super) fn contains(&self, movement: &MovementEntry) -> bool {
    self.movements.contains(movement)
  }

  fn contains_capture(&self) -> bool {
    self.movements.iter().any(|m| m.contains_capture())
  }
  pub(super) fn filter_captures_if_present(&mut self) -> () {
    if self.contains_capture() {
      self.movements.retain(|m| m.contains_capture());
    }
  }

  pub(crate) fn num_moves(&self) -> usize {
    self.movements.len()
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
  pub(super) movements: Vec<(BoardCoord,Option<BoardCoord>)>, // Vec<(intermediate_dest, Option<intermediate_capture>)>
  pub(super) promotion: Option<(BoardCoord, Box<Piece>)>
}

impl MovementEntry {
  fn contains_capture(&self) -> bool {
    self.movements.iter().any(|movement| {
      movement.1.is_some()
    })
  }

  pub(super) fn end_point(&self) -> BoardCoord {
    match self.movements.last() {
      Some(point) => point.0,
      None => self.from
    }
  }
}

pub(super) trait Moveable {
  fn get_valid_moves<'a>(&self, from: BoardCoord, inspect_board: impl Fn(BoardCoord) -> Result<&'a Option<Box<Piece>>, Box<dyn Error>>) -> Vec<MovementEntry>;
}