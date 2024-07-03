mod tiles;
pub mod tilesets;
mod wfc;

use cgmath::vec2;
use image::{
    imageops::{resize, rotate90},
    GenericImage, ImageBuffer, RgbaImage,
};

use crate::tiles::TileType;

fn main() {
    let mut tiles = tilesets::rpg_set();

    for _ in 0..100 {
        tiles.push(tiles[0].clone());
    }

    let canvas_size = vec2(512, 512);
    let mut canvas: RgbaImage = ImageBuffer::new(canvas_size.x, canvas_size.y);

    let wfc_size = vec2(16, 16);
    let mut wave = wfc::WaveFunctionCollapse::new(wfc_size.x, wfc_size.y, tiles.clone(), None);

    for x in 0..wfc_size.x {
        let _ = wave.insert(x, 0, TileType::Tile(tiles[0].clone()), 0);
        let _ = wave.insert(x, wfc_size.y - 1, TileType::Tile(tiles[0].clone()), 0);
    }

    for y in 0..wfc_size.y {
        let _ = wave.insert(0, y, TileType::Tile(tiles[0].clone()), 0);
        let _ = wave.insert(wfc_size.x - 1, y, TileType::Tile(tiles[0].clone()), 0);
    }

    let mut it = 0;
    loop {
        if wave.next().is_none() {
            break;
        }
        it += 1;
    }
    for x in 0..wfc_size.x {
        for y in 0..wfc_size.y {
            if let tiles::TileType::Tile(tile) = wave.get(x, y) {
                let mut tile_img = image::open(&tile.img_path).unwrap().to_rgba8();
                // resize
                tile_img = resize(
                    &tile_img,
                    canvas_size.x / wfc_size.x as u32,
                    canvas_size.y / wfc_size.y as u32,
                    image::imageops::FilterType::Nearest,
                );
                // rotate
                for _ in 0..tile.rotation {
                    tile_img = rotate90(&tile_img);
                }

                canvas
                    .copy_from(
                        &tile_img,
                        x as u32 * (canvas_size.x / wfc_size.x as u32),
                        y as u32 * (canvas_size.y / wfc_size.y as u32),
                    )
                    .unwrap();
            }
        }
    }
    println!("Rounds: {}", it);
    canvas.save(&format!("wfc-test14-{}.png", it)).unwrap();
}
