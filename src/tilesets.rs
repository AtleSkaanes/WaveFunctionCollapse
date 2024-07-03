use crate::tiles::Tile;

pub fn dungeon_set() -> Vec<Tile> {
    vec![
        Tile {
            name: "empty".to_owned(),
            img_path: "assets/Tiles/dungeon/empty.png".to_owned(),
            edges: vec![false, false, false, false],
            rotation: 0,
        },
        Tile {
            name: "room-end".to_owned(),
            img_path: "assets/Tiles/dungeon/room-end.png".to_owned(),
            edges: vec![false, true, false, false],
            rotation: 0,
        },
        Tile {
            name: "room-L".to_owned(),
            img_path: "assets/Tiles/dungeon/room-l.png".to_owned(),
            edges: vec![false, true, true, false],
            rotation: 0,
        },
        Tile {
            name: "hallway".to_owned(),
            img_path: "assets/Tiles/dungeon/hallway.png".to_owned(),
            edges: vec![false, true, false, true],
            rotation: 0,
        },
        Tile {
            name: "hallway-L".to_owned(),
            img_path: "assets/Tiles/dungeon/hallway-l.png".to_owned(),
            edges: vec![false, true, true, false],
            rotation: 0,
        },
        Tile {
            name: "hallway-T".to_owned(),
            img_path: "assets/Tiles/dungeon/hallway-t.png".to_owned(),
            edges: vec![false, true, true, true],
            rotation: 0,
        },
    ]
}

pub fn amongus_set() -> Vec<Tile> {
    vec![
        Tile {
            name: "amog-back".to_owned(),
            img_path: "assets/Tiles/amog/back.png".to_owned(),
            edges: vec![false, true, false, false],
            rotation: 0,
        },
        Tile {
            name: "amog-fill".to_owned(),
            img_path: "assets/Tiles/amog/fill.png".to_owned(),
            edges: vec![true, true, true, true],
            rotation: 0,
        },
        Tile {
            name: "amog-fill".to_owned(),
            img_path: "assets/Tiles/amog/fill.png".to_owned(),
            edges: vec![true, true, true, true],
            rotation: 0,
        },
        Tile {
            name: "amog-head".to_owned(),
            img_path: "assets/Tiles/amog/head.png".to_owned(),
            edges: vec![false, false, true, false],
            rotation: 0,
        },
        Tile {
            name: "amog-visor".to_owned(),
            img_path: "assets/Tiles/amog/visor.png".to_owned(),
            edges: vec![true, false, true, true],
            rotation: 0,
        },
        Tile {
            name: "amog-leg".to_owned(),
            img_path: "assets/Tiles/amog/leg.png".to_owned(),
            edges: vec![true, false, false, false],
            rotation: 0,
        },
        Tile {
            name: "amog-none".to_owned(),
            img_path: "assets/Tiles/amog/none.png".to_owned(),
            edges: vec![false, false, false, false],
            rotation: 0,
        },
    ]
}

pub fn rpg_set() -> Vec<Tile> {
    vec![
        Tile {
            name: "empty".to_owned(),
            img_path: "./assets/Tiles/rpg/empty.png".to_owned(),
            edges: vec![false, false, false, false],
            rotation: 0,
        },
        Tile {
            name: "room-end".to_owned(),
            img_path: "./assets/Tiles/rpg/room-end.png".to_owned(),
            edges: vec![false, true, false, false],
            rotation: 0,
        },
        Tile {
            name: "room-L".to_owned(),
            img_path: "./assets/Tiles/rpg/room-l.png".to_owned(),
            edges: vec![false, true, true, false],
            rotation: 0,
        },
        Tile {
            name: "room-long".to_owned(),
            img_path: "./assets/Tiles/rpg/room-long.png".to_owned(),
            edges: vec![false, true, false, true],
            rotation: 0,
        },
        Tile {
            name: "room-any".to_owned(),
            img_path: "./assets/Tiles/rpg/room-any.png".to_owned(),
            edges: vec![true, true, true, true],
            rotation: 0,
        },
        Tile {
            name: "hallway".to_owned(),
            img_path: "./assets/Tiles/rpg/hallway.png".to_owned(),
            edges: vec![false, true, false, true],
            rotation: 0,
        },
        Tile {
            name: "hallway-L".to_owned(),
            img_path: "./assets/Tiles/rpg/hallway-l.png".to_owned(),
            edges: vec![false, true, true, false],
            rotation: 0,
        },
        Tile {
            name: "hallway-T".to_owned(),
            img_path: "./assets/Tiles/rpg/hallway-t.png".to_owned(),
            edges: vec![false, true, true, true],
            rotation: 0,
        },
    ]
}
