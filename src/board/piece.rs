use std::fmt::Debug;


pub(super) trait Piece {

}

impl Debug for dyn Piece {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "piece")
  }
}