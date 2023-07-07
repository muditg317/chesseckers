
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Player {
  Chess,
  Checkers
}

impl Player {
  pub(super) fn other(&self) -> Player {
    match *self {
      Self::Chess => Self::Checkers,
      Self::Checkers => Self::Chess
    }
  }
}