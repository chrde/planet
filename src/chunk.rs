pub type IntCoord = u64;
pub type RootIndex = u8;
pub const ROOT_QUADS: u8 = 5;

#[derive(Clone)]
pub struct Root {
    index: RootIndex,
}

impl Root {
    pub fn new(index: RootIndex) -> Self {
        assert!(index < ROOT_QUADS);
        Root { index: index }
    }

    pub fn index(&self) -> RootIndex {
        self.index
    }
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
