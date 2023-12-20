mod tiles;
mod wfc;

use std::time::Instant;

use cgmath::{vec2, Vector2};
use image::{
    imageops::{resize, rotate90},
    GenericImage, ImageBuffer, RgbaImage,
};

use crate::tiles::Tile;

fn main() {
    let tiles = vec![
        Tile {
            name: "room-end".to_string(),
            img_path: "assets/Tiles/dungeon/room-end.png".to_string(),
            edges: vec![false, true, false, false],
            rotation: 0,
        },
        Tile {
            name: "room-L".to_string(),
            img_path: "assets/Tiles/dungeon/room-l.png".to_string(),
            edges: vec![false, true, true, false],
            rotation: 0,
        },
        Tile {
            name: "hallway".to_string(),
            img_path: "assets/Tiles/dungeon/hallway.png".to_string(),
            edges: vec![false, true, false, true],
            rotation: 0,
        },
        Tile {
            name: "hallway-L".to_string(),
            img_path: "assets/Tiles/dungeon/hallway-l.png".to_string(),
            edges: vec![false, true, true, false],
            rotation: 0,
        },
        Tile {
            name: "hallway-T".to_string(),
            img_path: "assets/Tiles/dungeon/hallway-t.png".to_string(),
            edges: vec![false, true, true, true],
            rotation: 0,
        },
    ];

    const N: usize = 20;
    let before = Instant::now();
    create_map(tiles, vec2(N, N));
    let after = before.elapsed();
    println!("Elapsed time: {:.2?}", after.as_nanos());

    // let canvas_size = vec2(512, 512);
    // let mut canvas: RgbaImage = ImageBuffer::new(canvas_size.x, canvas_size.y);
    //
    // let wfc_size = vec2(10, 10);
    // let mut wave = wfc::WaveFunctionCollapse::new(wfc_size.x, wfc_size.y, tiles, None);
    //
    // let mut it = 0;
    // loop {
    //     if wave.next().is_none() {
    //         break;
    //     }
    //     it += 1;
    // }
    // for x in 0..wfc_size.x {
    //     for y in 0..wfc_size.y {
    //         if let tiles::TileType::Tile(tile) = wave.get(x, y) {
    //             //println!("== x: {}, y: {} ==", x, y);
    //             //dbg!(&tile);
    //             //println!("\n");
    //             let mut tile_img = image::open(&tile.img_path).unwrap().to_rgba8();
    //             // resize
    //             tile_img = resize(
    //                 &tile_img,
    //                 canvas_size.x / wfc_size.x as u32,
    //                 canvas_size.y / wfc_size.y as u32,
    //                 image::imageops::FilterType::Nearest,
    //             );
    //             // rotate
    //             for _ in 0..tile.rotation {
    //                 tile_img = rotate90(&tile_img);
    //             }
    //
    //             canvas
    //                 .copy_from(
    //                     &tile_img,
    //                     x as u32 * (canvas_size.x / wfc_size.x as u32),
    //                     y as u32 * (canvas_size.y / wfc_size.y as u32),
    //                 )
    //                 .unwrap();
    //         }
    //     }
    // }
    // println!("Rounds: {}", it);
    // canvas.save(&format!("wfc-test13-{}.png", it)).unwrap();
}

fn create_map(tiles: Vec<Tile>, wfc_size: Vector2<usize>) {
    let canvas_size = vec2(512, 512);
    let mut canvas: RgbaImage = ImageBuffer::new(canvas_size.x, canvas_size.y);

    let mut wave = wfc::WaveFunctionCollapse::new(wfc_size.x, wfc_size.y, tiles, None);

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
    canvas
        .save(&format!("wfc-test13-{}.png", wfc_size.x))
        .unwrap();
}
