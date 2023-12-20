use cgmath::{vec2, Vector2};
use rand::{seq::SliceRandom, SeedableRng};
use rand_chacha::ChaCha20Rng;

use crate::tiles::{self, EmptyTile, TileType};

pub struct WaveFunctionCollapse {
    pub board_height: usize,
    pub board_width: usize,
    valid_tiles: Vec<tiles::Tile>,
    seed: Option<u64>,
    rng: ChaCha20Rng,
    tiles: Vec<Vec<TileType>>,
    min_entropy: usize,
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
            min_entropy: 0,
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
    fn set(&mut self, x: usize, y: usize, tile: TileType) {
        self.tiles[x][y] = tile;
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
        let neighbors = self.get_neighbors(to_collapse.x, to_collapse.y);
        // let vecs = vec![
        //     new_tile.unwrap_tile().up,
        //     new_tile.unwrap_tile().right,
        //     new_tile.unwrap_tile().down,
        //     new_tile.unwrap_tile().left,
        // ];
        for i in 0..neighbors.len() {
            if let TileType::Empty(tile) = self.get(neighbors[i].x, neighbors[i].y) {
                let edge_index =
                    wrap_mod(i as i32 + 2, new_tile.unwrap_tile().edges.len() as i32) as usize;

                let mut new = EmptyTile::default();
                //println!(
                //     "== Possible ({}) [{}, {}]==",
                //     i, neighbors[i].x, neighbors[i].y
                // );
                //dbg!(&tile.possible_tiles);
                //println!("\n== Current [{}, {}]==", to_collapse.x, to_collapse.y);
                //dbg!(&new_tile);

                for p_tile in tile.possible_tiles {
                    // new.possible_tiles = tile
                    //     .clone()
                    //     .possible_tiles
                    //     .iter()
                    //     .filter(|x| new_tile.unwrap_tile().contains(&x.name))
                    //     .map(|x| x.clone())
                    //     .collect::<Vec<Tile>>();
                    //println!(
                    //     "\np_edge: {}={}, c_edge: {}={}",
                    //     edge_index,
                    //     p_tile.edges[edge_index],
                    //     i,
                    //     new_tile.clone().unwrap_tile().edges[i]
                    // );
                    if p_tile.edges[edge_index] == new_tile.clone().unwrap_tile().edges[i] {
                        //println!("\n\tres:\ttrue");
                        new.possible_tiles.push(p_tile);
                    } else {
                        //println!("\n\tres:\tfalse")
                    }
                }
                //println!("\n== New ({}) ==", i);
                //dbg!(&new);
                //println!("\n== END ==\n\n");
                self.set(neighbors[i].x, neighbors[i].y, TileType::Empty(new))
            }
        }

        Some(())
    }
    fn generate(&mut self) {
        let mut new_tiles = vec![];
        for tile in self.valid_tiles.iter() {
            // Only make rotations if block isn't same on all edges
            if tile.edges.contains(&true) && tile.edges.contains(&false) {
                for i in 1..=3 {
                    let mut new_tile = tile.clone();
                    new_tile.rotation = i;
                    new_tile.edges.rotate_right(i as usize);
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
