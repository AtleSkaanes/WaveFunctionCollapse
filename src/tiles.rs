use core::fmt;

#[derive(Clone, Debug)]
pub enum TileType {
    Tile(Tile),
    Empty(EmptyTile),
    NoTile,
}
impl TileType {
    pub fn is_tile(&self) -> bool {
        matches!(self, Self::Tile(_))
    }
    pub fn is_empty(&self) -> bool {
        matches!(self, Self::Empty(_))
    }
    pub fn is_notile(&self) -> bool {
        matches!(self, Self::NoTile)
    }
    pub fn unwrap_empty(&self) -> EmptyTile {
        match self {
            Self::Empty(tile) => tile.clone(),
            _ => panic!("Not of type \"EmptyTile\""),
        }
    }
    pub fn unwrap_tile(&self) -> Tile {
        match self {
            Self::Tile(tile) => tile.clone(),
            _ => panic!("Not of type \"Tile\""),
        }
    }
}

#[derive(Default, Clone)]
pub struct Tile {
    pub name: String,
    pub img_path: String,
    pub edges: Vec<bool>,
    pub rotation: usize,
}

impl Tile {
    pub fn new(name: &str, img_path: &str, edges: Vec<bool>) -> Self {
        Self {
            name: name.to_owned(),
            img_path: img_path.to_owned(),
            edges,
            rotation: 0,
        }
    }
}

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Tile")
            .field("name", &self.name)
            .field("edges", &self.edges)
            .field("rotation", &self.rotation)
            .finish()
    }
}

#[derive(Default, Clone, Debug)]
pub struct EmptyTile {
    pub possible_tiles: Vec<Tile>,
}
