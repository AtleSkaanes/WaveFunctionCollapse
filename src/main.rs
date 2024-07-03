mod tiles;
pub mod tilesets;
mod wfc;

use cgmath::vec2;
use image::{
    imageops::{resize, rotate90},
    GenericImage, ImageBuffer, RgbaImage,
};

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
        wave.insert(x, 0, tiles[0].clone(), 0).unwrap();
        wave.insert(x, wfc_size.y - 1, tiles[0].clone(), 0).unwrap();
    }

    for y in 0..wfc_size.y {
        wave.insert(0, y, tiles[0].clone(), 0).unwrap();
        wave.insert(wfc_size.x - 1, y, tiles[0].clone(), 0).unwrap();
    }

    //dbg!(wave.get_entropy(1, 1));
    //dbg!(wave.get(0, 0));

    let mut it = 0;
    loop {
        if wave.next().is_none() {
            break;
        }
        it += 1;
    }
    for x in 0..wfc_size.x {
        for y in 0..wfc_size.y {
            let Some(cell) = wave.get(x, y) else {
                continue;
            };

            let Some(tile) = cell.tile else {
                continue;
            };

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
    println!("Rounds: {}", it);
    canvas.save(&format!("wfc-test14-{}.png", it)).unwrap();
}
