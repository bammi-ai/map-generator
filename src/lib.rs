extern crate rand;

use std::collections::BTreeSet;

use rand::Rng;

pub struct Tile {
    pub x: usize,
    pub y: usize,
    pub id: usize,
    pub neighbors: Vec<usize>,
}

pub struct Map {
    pub width: usize,
    pub height: usize,
    pub is_repeated: bool,
    pub map: Vec<Vec<usize>>,
    pub tiles: Vec<Tile>,
}

fn find_first<T: PartialEq>(haystack: &[Vec<T>], needle: T) -> Option<(usize, usize)> {
    haystack.iter().enumerate().find_map(|(y, row)| {
        row.iter()
            .position(|value| *value == needle)
            .map(|x| (x, y))
    })
}

fn new_repeated(width: usize, height: usize, p: f64) -> Map {
    let vertical_walls = (0..height)
        .map(|_| {
            (0..width)
                .map(|_| rand::thread_rng().gen_bool(p))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let horizontal_walls = (0..height)
        .map(|_| {
            (0..width)
                .map(|_| rand::thread_rng().gen_bool(p))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut map = vec![vec![0; width]; height];
    let mut tile_count = 0;
    loop {
        if let Some((x, y)) = find_first(&map, 0) {
            fn flood_fill(
                x: usize,
                y: usize,
                to: usize,
                map: &mut Vec<Vec<usize>>,
                vertical_walls: &Vec<Vec<bool>>,
                horizontal_walls: &Vec<Vec<bool>>,
                width: usize,
                height: usize,
            ) {
                let x = if x == width {
                    0
                } else if x == usize::MAX {
                    width - 1
                } else {
                    x
                };
                let y = if y == height {
                    0
                } else if y == usize::MAX {
                    height - 1
                } else {
                    y
                };
                if map[y][x] != 0 {
                    return;
                }
                map[y][x] = to;
                if !vertical_walls[y][x] {
                    flood_fill(
                        x - 1,
                        y,
                        to,
                        map,
                        vertical_walls,
                        horizontal_walls,
                        width,
                        height,
                    );
                }
                if !vertical_walls[y][x + 1] {
                    flood_fill(
                        x + 1,
                        y,
                        to,
                        map,
                        vertical_walls,
                        horizontal_walls,
                        width,
                        height,
                    );
                }
                if !horizontal_walls[y][x] {
                    flood_fill(
                        x,
                        y - 1,
                        to,
                        map,
                        vertical_walls,
                        horizontal_walls,
                        width,
                        height,
                    );
                }
                if !horizontal_walls[y + 1][x] {
                    flood_fill(
                        x,
                        y + 1,
                        to,
                        map,
                        vertical_walls,
                        horizontal_walls,
                        width,
                        height,
                    );
                }
            }
            tile_count += 1;
            flood_fill(
                x,
                y,
                tile_count,
                &mut map,
                &vertical_walls,
                &horizontal_walls,
                width,
                height,
            );
        } else {
            break;
        }
    }
    let mut tiles = Vec::new();
    for i in 0..tile_count {
        let mut positions = Vec::new();
        let mut neighbors = BTreeSet::new();
        fn flood_fill(
            x: usize,
            y: usize,
            from: usize,
            to: usize,
            map: &mut Vec<Vec<usize>>,
            width: usize,
            height: usize,
            positions: &mut Vec<(usize, usize)>,
            neighbors: &mut BTreeSet<usize>,
        ) {
            let x = if x == width {
                0
            } else if x == usize::MAX {
                width - 1
            } else {
                x
            };
            let y = if y == height {
                0
            } else if y == usize::MAX {
                height - 1
            } else {
                y
            };
            if map[y][x] != from {
                neighbors.insert(map[y][x]);
                return;
            }
            positions.push((x, y));
            map[y][x] = to;
            flood_fill(x - 1, y, from, to, map, width, height, positions, neighbors);
            flood_fill(x + 1, y, from, to, map, width, height, positions, neighbors);
            flood_fill(x, y - 1, from, to, map, width, height, positions, neighbors);
            flood_fill(x, y + 1, from, to, map, width, height, positions, neighbors);
        }
        let (x, y) = find_first(&map, i + 1).unwrap();
        flood_fill(
            x,
            y,
            i + 1,
            i,
            &mut map,
            width,
            height,
            &mut positions,
            &mut neighbors,
        );
        let (x, y) = positions
            .get(rand::thread_rng().gen_range(0..positions.len()))
            .unwrap();
        tiles.push(Tile {
            x: *x,
            y: *y,
            id: i,
            neighbors: neighbors
                .iter()
                .map(|x| if *x > i { *x - 1 } else { *x })
                .collect::<Vec<_>>(),
        })
    }
    Map {
        width,
        height,
        is_repeated: true,
        map,
        tiles,
    }
}

fn new_not_repeated(width: usize, height: usize, p: f64) -> Map {
    let vertical_walls = (0..height)
        .map(|_| {
            (0..width - 1)
                .map(|_| rand::thread_rng().gen_bool(p))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let horizontal_walls = (0..height - 1)
        .map(|_| {
            (0..width)
                .map(|_| rand::thread_rng().gen_bool(p))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut map = vec![vec![0; width]; height];
    let mut tile_count = 0;
    loop {
        if let Some((x, y)) = find_first(&map, 0) {
            fn flood_fill(
                x: usize,
                y: usize,
                to: usize,
                map: &mut Vec<Vec<usize>>,
                vertical_walls: &Vec<Vec<bool>>,
                horizontal_walls: &Vec<Vec<bool>>,
                width: usize,
                height: usize,
            ) {
                if x == usize::MAX || x == width || y == usize::MAX || y == height || map[y][x] != 0
                {
                    return;
                }
                map[y][x] = to;
                if !vertical_walls[y][x] {
                    flood_fill(
                        x - 1,
                        y,
                        to,
                        map,
                        vertical_walls,
                        horizontal_walls,
                        width,
                        height,
                    );
                }
                if x != width - 1 && !vertical_walls[y][x + 1] {
                    flood_fill(
                        x + 1,
                        y,
                        to,
                        map,
                        vertical_walls,
                        horizontal_walls,
                        width,
                        height,
                    );
                }
                if !horizontal_walls[y][x] {
                    flood_fill(
                        x,
                        y - 1,
                        to,
                        map,
                        vertical_walls,
                        horizontal_walls,
                        width,
                        height,
                    );
                }
                if y != height - 1 && !horizontal_walls[y + 1][x] {
                    flood_fill(
                        x,
                        y + 1,
                        to,
                        map,
                        vertical_walls,
                        horizontal_walls,
                        width,
                        height,
                    );
                }
            }
            tile_count += 1;
            flood_fill(
                x,
                y,
                tile_count,
                &mut map,
                &vertical_walls,
                &horizontal_walls,
                width,
                height,
            );
        } else {
            break;
        }
    }
    let mut tiles = Vec::new();
    for i in 0..tile_count {
        let mut positions = Vec::new();
        let mut neighbors = BTreeSet::new();
        fn flood_fill(
            x: usize,
            y: usize,
            from: usize,
            to: usize,
            map: &mut Vec<Vec<usize>>,
            width: usize,
            height: usize,
            positions: &mut Vec<(usize, usize)>,
            neighbors: &mut BTreeSet<usize>,
        ) {
            if x == usize::MAX || x == width || y == usize::MAX || y == height {
                return;
            }
            if map[y][x] != from {
                neighbors.insert(map[y][x]);
                return;
            }
            positions.push((x, y));
            map[y][x] = to;
            flood_fill(x - 1, y, from, to, map, width, height, positions, neighbors);
            flood_fill(x + 1, y, from, to, map, width, height, positions, neighbors);
            flood_fill(x, y - 1, from, to, map, width, height, positions, neighbors);
            flood_fill(x, y + 1, from, to, map, width, height, positions, neighbors);
        }
        let (x, y) = find_first(&map, i + 1).unwrap();
        flood_fill(
            x,
            y,
            i + 1,
            i,
            &mut map,
            width,
            height,
            &mut positions,
            &mut neighbors,
        );
        let (x, y) = positions
            .get(rand::thread_rng().gen_range(0..positions.len()))
            .unwrap();
        tiles.push(Tile {
            x: *x,
            y: *y,
            id: i,
            neighbors: neighbors
                .iter()
                .map(|x| if *x > i { *x - 1 } else { *x })
                .collect::<Vec<_>>(),
        })
    }
    Map {
        width,
        height,
        is_repeated: true,
        map,
        tiles,
    }
}

impl Map {
    pub fn new(width: usize, height: usize, is_repeated: bool, p: f64) -> Map {
        if is_repeated {
            new_repeated(width, height, p)
        } else {
            new_not_repeated(width, height, p)
        }
    }
}
