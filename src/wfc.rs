use cgmath::{vec2, Vector2};
use rand::{seq::SliceRandom, SeedableRng};
use rand_chacha::ChaCha20Rng;

use crate::tiles::{self, GridCell, Tile};

pub struct WaveFunctionCollapse {
    pub board_height: usize,
    pub board_width: usize,
    valid_tiles: Vec<Tile>,
    seed: Option<u64>,
    rng: ChaCha20Rng,
    tiles: Vec<Vec<GridCell>>,
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

    pub fn get(&self, x: usize, y: usize) -> Option<GridCell> {
        if x >= self.tiles.len() || y >= self.tiles[x].len() {
            None
        } else {
            Some(self.tiles[x][y].clone())
        }
    }

    pub fn insert(&mut self, x: usize, y: usize, tile: Tile, rot: usize) -> Result<(), ()> {
        if x >= self.tiles.len() || y >= self.tiles[x].len() {
            return Err(());
        }
        let mut new_tile = tile.clone();
        new_tile.rotation += rot;
        new_tile.edges.rotate_right(rot);

        self.set(x, y, GridCell::new_tile(new_tile.clone()));
        self.update_neighbors(x, y, tile);

        //println!("Min ent: {}", self.min_entropy());

        Ok(())
    }

    pub fn get_entropy(&self, x: usize, y: usize) -> Option<Vec<Tile>> {
        if x >= self.tiles.len() || y >= self.tiles[x].len() {
            return None;
        }
        if !self.tiles[x][y].is_collapsed() {
            return Some(self.tiles[x][y].possible_tiles.clone());
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
                let Some(tile) = self.get(x, y) else {
                    continue;
                };
                if tile.is_collapsed() {
                    continue;
                }

                if tile.possible_tiles.len() == min {
                    collapsable.push(vec2(x, y));
                }
            }
        }
        // Get a specific tile to collapse
        let to_collapse = collapsable.choose(&mut self.rng)?;
        let new_tile = self
            .get(to_collapse.x, to_collapse.y)
            .unwrap()
            .possible_tiles
            .choose(&mut self.rng)?
            .clone();

        // Set new tile
        self.set(
            to_collapse.x,
            to_collapse.y,
            GridCell::new_tile(new_tile.clone()),
        );

        // Update all neighbours' possible tiles
        self.update_neighbors(to_collapse.x, to_collapse.y, new_tile);

        Some(())
    }

    fn set(&mut self, x: usize, y: usize, tile: GridCell) {
        self.tiles[x][y] = tile;
    }

    fn update_neighbors(&mut self, x: usize, y: usize, current_tile: Tile) {
        let neighbors = self.get_neighbors(x, y);
        for (i, neighbor) in neighbors.iter().enumerate() {
            let Some(neighbor_tile) = self.get(neighbor.x, neighbor.y) else {
                continue;
            };

            if neighbor_tile.is_collapsed() {
                continue;
            }

            let edge_index = wrap_mod(i as i32 + 2, current_tile.edges.len() as i32) as usize;

            let mut new = GridCell::new_empty(vec![]);

            for p_tile in neighbor_tile.possible_tiles {
                if p_tile.edges[edge_index] == current_tile.clone().edges[i] {
                    new.possible_tiles.push(p_tile);
                }
            }
            self.set(neighbor.x, neighbor.y, new);
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

        let empty_cell = GridCell::new_empty(self.valid_tiles.clone());

        // Fill tile vec with empty cells
        self.tiles = vec![vec![empty_cell.clone(); self.board_height]; self.board_width];

        //for x in 0..self.board_width {
        //    self.tiles.push(vec![]);
        //    for _y in 0..self.board_height {
        //        let tile = GridCell::new_empty(self.valid_tiles.clone());
        //        self.tiles[x].push(tile);
        //    }
        //}

        if let Some(seed) = self.seed {
            self.rng = ChaCha20Rng::seed_from_u64(seed);
        }
    }

    fn min_entropy(&self) -> usize {
        let mut min = usize::MAX;
        for (i, col) in self.tiles.iter().enumerate() {
            for (j, tile) in col.iter().enumerate() {
                if tile.is_collapsed() {
                    //println!("IS COLLAPSED: ({} ; {})", i, j);
                    continue;
                }
                if tile.possible_tiles.len() <= min {
                    //println!("0 at ({} ; {})", i, j);
                    min = tile.possible_tiles.len();
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
