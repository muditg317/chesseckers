use std::ops::Add;


pub type BoardCoord = (usize,usize);

#[derive(Debug, Clone, Copy)]
pub struct BoardCoordS(pub usize, pub usize);

impl BoardCoordS {
  fn as_tuple(&self) -> (usize,usize) {
    (self.0,self.1)
  }
}

impl From<BoardCoord> for BoardCoordS {
  fn from((r,c): BoardCoord) -> Self {
    BoardCoordS(r, c)
  }
}

impl From<BoardCoordS> for BoardCoord {
  fn from(value: BoardCoordS) -> Self {
    value.as_tuple()
  }
}

impl Add<(isize,isize)> for BoardCoordS {
  type Output = Self;

  fn add(self, rhs: (isize,isize)) -> Self::Output {
    BoardCoordS(
      (self.0 as isize + rhs.0) as usize,
      (self.1 as isize + rhs.1) as usize
    )
  }
}