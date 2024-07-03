use cgmath::{vec2, Vector2};
use rand::{seq::SliceRandom, SeedableRng};
use rand_chacha::ChaCha20Rng;

use crate::tiles::{self, EmptyTile, Tile, TileType};

pub struct WaveFunctionCollapse {
    pub board_height: usize,
    pub board_width: usize,
    valid_tiles: Vec<tiles::Tile>,
    seed: Option<u64>,
    rng: ChaCha20Rng,
    tiles: Vec<Vec<TileType>>,
}

impl WaveFunctionCollapse {
    pub fn new(
        board_height: usize,
        board_width: usize,
        valid_tiles: Vec<tiles::Tile>,
        seed: Option<u64>,
    ) -> Self {
        let mut wfc = Self {
            board_height,
            board_width,
            valid_tiles,
            seed,
            rng: ChaCha20Rng::from_entropy(),
            tiles: vec![],
        };
        // wfc.min_entropy = wfc.valid_tiles.len();
        wfc.generate();
        wfc
    }
    pub fn get(&self, x: usize, y: usize) -> TileType {
        if x >= self.tiles.len() || y >= self.tiles[x].len() {
            return TileType::NoTile;
        }
        self.tiles[x][y].clone()
    }
    pub fn insert(&mut self, x: usize, y: usize, tile: TileType, rot: usize) -> Result<(), ()> {
        if x >= self.tiles.len() || y >= self.tiles[x].len() {
            return Err(());
        }
        if let TileType::Tile(t) = tile {
            let mut new_tile = t.clone();
            new_tile.rotation += rot;
            new_tile.edges.rotate_right(rot);

            self.set(x, y, TileType::Tile(new_tile.clone()));
            self.update_neighbors(x, y, t);
        } else {
            self.set(x, y, tile);
        }

        //println!("Min ent: {}", self.min_entropy());

        Ok(())
    }
    pub fn get_entropy(&self, x: usize, y: usize) -> Option<Vec<Tile>> {
        if x >= self.tiles.len() || y >= self.tiles[x].len() {
            return None;
        }
        if self.tiles[x][y].is_empty() {
            return Some(self.tiles[x][y].unwrap_empty().possible_tiles.clone());
        }
        None
    }

    pub fn next(&mut self) -> Option<()> {
        let min = self.min_entropy();
        //println!("min: {}", min);
        let mut collapsable: Vec<Vector2<usize>> = vec![];
        // Get all tiles that can collapse
        for x in 0..self.tiles.len() {
            for y in 0..self.tiles[x].len() {
                if let TileType::Empty(tile) = self.get(x, y) {
                    if tile.possible_tiles.len() == min {
                        collapsable.push(vec2(x, y));
                    }
                }
            }
        }
        // Get a specific tile to collapse
        let to_collapse = collapsable.choose(&mut self.rng)?;
        let new_tile = TileType::Tile(
            self.get(to_collapse.x, to_collapse.y)
                .unwrap_empty()
                .possible_tiles
                .choose(&mut self.rng)?
                .clone(),
        );

        // Set new tile
        self.set(to_collapse.x, to_collapse.y, new_tile.clone());

        // Update all neighbours' possible tiles
        self.update_neighbors(to_collapse.x, to_collapse.y, new_tile.unwrap_tile());

        Some(())
    }
    fn set(&mut self, x: usize, y: usize, tile: TileType) {
        self.tiles[x][y] = tile;
    }
    fn update_neighbors(&mut self, x: usize, y: usize, current_tile: Tile) {
        let neighbors = self.get_neighbors(x, y);
        for (i, neighbor) in neighbors.iter().enumerate() {
            if let TileType::Empty(neighbor_tile) = self.get(neighbor.x, neighbor.y) {
                let edge_index = wrap_mod(i as i32 + 2, current_tile.edges.len() as i32) as usize;

                let mut new = EmptyTile::default();

                for p_tile in neighbor_tile.possible_tiles {
                    if p_tile.edges[edge_index] == current_tile.clone().edges[i] {
                        new.possible_tiles.push(p_tile);
                    }
                }
                self.set(neighbor.x, neighbor.y, TileType::Empty(new))
            }
        }
    }
    fn generate(&mut self) {
        let mut new_tiles = vec![];
        for tile in self.valid_tiles.iter() {
            // Only make rotations if block isn't same on all edges
            if tile.edges.contains(&true) && tile.edges.contains(&false) {
                for i in 1..=3 {
                    let mut new_tile = tile.clone();
                    new_tile.rotation = i;
                    new_tile.edges.rotate_right(i);
                    new_tiles.push(new_tile.clone());
                }
            }
        }
        self.valid_tiles.append(&mut new_tiles);
        for x in 0..self.board_width {
            self.tiles.push(vec![]);
            for _y in 0..self.board_height {
                let mut t = tiles::EmptyTile::default();
                t.possible_tiles = self.valid_tiles.clone();
                self.tiles[x].push(TileType::Empty(t));
            }
        }
        if let Some(seed) = self.seed {
            self.rng = ChaCha20Rng::seed_from_u64(seed);
        }
    }
    fn min_entropy(&self) -> usize {
        let mut min = 0;
        for col in self.tiles.iter() {
            for item in col {
                if let TileType::Empty(tile) = item {
                    if tile.possible_tiles.len() < min || min == 0 {
                        min = tile.possible_tiles.len();
                    }
                }
            }
        }
        min
    }
    fn get_neighbors(&self, x: usize, y: usize) -> Vec<Vector2<usize>> {
        vec![
            vec2(
                x,
                if (y as i32 - 1) >= 0 {
                    y - 1
                } else {
                    self.tiles[x].len()
                },
            ),
            vec2(x + 1, y),
            vec2(x, y + 1),
            vec2(
                if (x as i32 - 1) >= 0 {
                    x - 1
                } else {
                    self.tiles.len()
                },
                y,
            ),
        ]
    }
}

fn wrap_mod(a: i32, b: i32) -> i32 {
    ((b + a) as f32 % b as f32) as i32
}
