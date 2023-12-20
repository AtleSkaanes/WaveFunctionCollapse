use core::fmt;

use cgmath::{vec4, Vector4};

#[derive(Clone, Debug)]
pub enum TileType {
    Tile(Tile),
    Empty(EmptyTile),
    NoTile,
}
impl TileType {
    pub fn is_tile(&self) -> bool {
        if let Self::Tile(_) = self {
            return true;
        } else {
            return false;
        }
    }
    pub fn is_empty(&self) -> bool {
        if let Self::Empty(_) = self {
            return true;
        } else {
            return false;
        }
    }
    pub fn is_notile(&self) -> bool {
        if let Self::NoTile = self {
            return true;
        } else {
            return false;
        }
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
    // pub up: Vec<String>,
    // pub down: Vec<String>,
    // pub left: Vec<String>,
    // pub right: Vec<String>,
}
impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Tile")
            .field("name", &self.name)
            .field("edges", &self.edges)
            .field("rotation", &self.rotation)
            // .field("up", &self.up)
            // .field("down", &self.down)
            // .field("left", &self.left)
            // .field("right", &self.right)
            .finish()
    }
}

#[derive(Default, Clone, Debug)]
pub struct EmptyTile {
    pub possible_tiles: Vec<Tile>,
}
