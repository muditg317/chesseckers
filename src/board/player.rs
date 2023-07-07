
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub enum Player {
  #[default] Chess,
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