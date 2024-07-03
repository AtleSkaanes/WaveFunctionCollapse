use crate::tiles::Tile;

pub fn dungeon_set() -> Vec<Tile> {
    vec![
        Tile::new(
            "empty",
            "./assets/Tiles/dungeon/empty.png",
            [false, false, false, false],
        ),
        Tile::new(
            "room-end",
            "./assets/Tiles/dungeon/room-end.png",
            [false, true, false, false],
        ),
        Tile::new(
            "room-L",
            "./assets/Tiles/dungeon/room-l.png",
            [false, true, true, false],
        ),
        Tile::new(
            "hallway",
            "./assets/Tiles/dungeon/hallway.png",
            [false, true, false, true],
        ),
        Tile::new(
            "hallway-L",
            "./assets/Tiles/dungeon/hallway-l.png",
            [false, true, true, false],
        ),
        Tile::new(
            "hallway-T",
            "./assets/Tiles/dungeon/hallway-t.png",
            [false, true, true, true],
        ),
    ]
}

pub fn amongus_set() -> Vec<Tile> {
    vec![
        Tile::new(
            "amog-back",
            "./assets/Tiles/amog/back.png",
            [false, true, false, false],
        ),
        Tile::new(
            "amog-fill",
            "./assets/Tiles/amog/fill.png",
            [true, true, true, true],
        ),
        Tile::new(
            "amog-fill",
            "./assets/Tiles/amog/fill.png",
            [true, true, true, true],
        ),
        Tile::new(
            "amog-head",
            "./assets/Tiles/amog/head.png",
            [false, false, true, false],
        ),
        Tile::new(
            "amog-visor",
            "./assets/Tiles/amog/visor.png",
            [true, false, true, true],
        ),
        Tile::new(
            "amog-leg",
            "./assets/Tiles/amog/leg.png",
            [true, false, false, false],
        ),
        Tile::new(
            "amog-none",
            "./assets/Tiles/amog/none.png",
            [false, false, false, false],
        ),
    ]
}

pub fn rpg_set() -> Vec<Tile> {
    vec![
        Tile::new(
            "empty",
            "./assets/Tiles/rpg/empty.png",
            [false, false, false, false],
        ),
        Tile::new(
            "room-end",
            "./assets/Tiles/rpg/room-end.png",
            [false, true, false, false],
        ),
        Tile::new(
            "room-L",
            "./assets/Tiles/rpg/room-l.png",
            [false, true, true, false],
        ),
        Tile::new(
            "room-long",
            "./assets/Tiles/rpg/room-long.png",
            [false, true, false, true],
        ),
        Tile::new(
            "room-any",
            "./assets/Tiles/rpg/room-any.png",
            [true, true, true, true],
        ),
        Tile::new(
            "hallway",
            "./assets/Tiles/rpg/hallway.png",
            [false, true, false, true],
        ),
        Tile::new(
            "hallway-L",
            "./assets/Tiles/rpg/hallway-l.png",
            [false, true, true, false],
        ),
        Tile::new(
            "hallway-T",
            "./assets/Tiles/rpg/hallway-t.png",
            [false, true, true, true],
        ),
    ]
}
