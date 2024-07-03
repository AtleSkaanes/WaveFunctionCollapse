use core::fmt;

//#[derive(Clone, Debug)]
//pub enum TileType {
//    Tile(Tile),
//    Empty(EmptyTile),
//    NoTile,
//}
//impl TileType {
//    pub fn is_tile(&self) -> bool {
//        matches!(self, Self::Tile(_))
//    }
//    pub fn is_empty(&self) -> bool {
//        matches!(self, Self::Empty(_))
//    }
//    pub fn is_notile(&self) -> bool {
//        matches!(self, Self::NoTile)
//    }
//    pub fn unwrap_empty(&self) -> EmptyTile {
//        match self {
//            Self::Empty(tile) => tile.clone(),
//            _ => panic!("Not of type \"EmptyTile\""),
//        }
//    }
//    pub fn unwrap_tile(&self) -> Tile {
//        match self {
//            Self::Tile(tile) => tile.clone(),
//            _ => panic!("Not of type \"Tile\""),
//        }
//    }
//}

#[derive(Clone, Debug)]
pub struct GridCell {
    pub tile: Option<Tile>,
    pub possible_tiles: Vec<Tile>,
}
impl GridCell {
    pub fn new_tile(tile: Tile) -> Self {
        Self {
            tile: Some(tile),
            possible_tiles: vec![],
        }
    }
    pub fn new_empty(possible_tiles: Vec<Tile>) -> Self {
        Self {
            tile: None,
            possible_tiles,
        }
    }
    pub fn is_collapsed(&self) -> bool {
        self.tile.is_some()
    }
}

#[derive(Default, Clone)]
pub struct Tile {
    /// The tile's name
    pub name: String,
    /// The path to the tile's image
    pub img_path: String,
    /// The tiles edges
    pub edges: [bool; 4],
    /// The amount of times the tile is rotated 90-degrees, clockwise
    pub rotation: usize,
}

impl Tile {
    pub fn new(name: &str, img_path: &str, edges: [bool; 4]) -> Self {
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
