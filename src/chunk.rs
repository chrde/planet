pub type IntCoord = u64;
pub type RootIndex = u8;

#[derive(Clone)]
pub struct Root {
  pub index: RootIndex,
}

pub struct CellPos {
  pub root: Root,
  pub x: IntCoord,
  pub y: IntCoord,
}

pub struct Cell {
  pub height: f64,
}

pub struct Chunk {
  pub origin: CellPos,
  pub cells: Vec<Cell>,
}