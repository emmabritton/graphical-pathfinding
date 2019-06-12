use crate::data::Coord;
use ggez::{Context, filesystem};
use std::io::Read;
use crate::{GRID_VERT_COUNT, GRID_HORZ_COUNT};
use std::str::Chars;

#[allow(dead_code)]
pub const NODE_FREE: i32 = 0;
pub const NODE_WALL: i32 = -1;

pub struct Variant {
    pub start: Coord,
    pub ends: Vec<Coord>,
}

pub struct Map {
    pub variants: Vec<Variant>,
    pub cost: Vec<Vec<i32>>,
}

impl Map {
    pub fn get_column_count(&self) -> usize {
        self.cost.len()
    }

    pub fn get_row_count(&self) -> usize {
        self.cost[0].len()
    }
}

#[inline]
pub fn node_cost_to_percentage(cost: i32) -> f32 {
    cost as f32 / 10.
}

pub fn read_map_file(ctx: &mut Context, which: usize) -> Map {
    if let Ok(mut file) = filesystem::open(ctx, format!("/map{}", which)) {
        let mut buffer = String::new();
        match file.read_to_string(&mut buffer) {
            Ok(_) => {
                let lines: Vec<&str> = buffer.split_whitespace().collect();

                let mut cost = vec![vec![0; GRID_VERT_COUNT]; GRID_HORZ_COUNT];

                let mut x = 0_usize;
                let mut y = 0_usize;

                let mut variants = vec![];

                lines.iter().for_each(|&line| {
                    let mut chars = line.chars();
                    match chars.next().unwrap() {
                        'M' => {
                            for letter in chars {
                                match letter {
                                    '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' => cost[x][y] = letter.to_digit(10).unwrap() as i32,
                                    '9' => cost[x][y] = NODE_WALL,
                                    '0' | '\n' => { /* ignored */ }
                                    _ => panic!("Unexpected character {} found at {},{} in map {}", letter, x, y, which)
                                }
                                if letter != '\n' {
                                    x += 1;
                                }
                                if x >= GRID_HORZ_COUNT {
                                    x = 0;
                                    y += 1;
                                }
                                if y > GRID_VERT_COUNT {
                                    break;
                                }
                            }
                        }
                        'S' => {
                            variants.push(Variant { start: convert_chars_to_coords(chars), ends: vec![] });
                        }
                        'E' => {
                            if let Some(variant) = variants.last_mut() {
                                variant.ends.push(convert_chars_to_coords(chars));
                            } else {
                                panic!("End without start in map {}", which);
                            }
                        }
                        _ => {}
                    }
                });

                if y < GRID_VERT_COUNT {
                    panic!("map {} is too short", which);
                }

                if variants.is_empty() {
                    panic!("map {} has no variants", which);
                }

                variants.iter().for_each(|variant| {
                    if variant.ends.is_empty() {
                        panic!("variant in {} has no end", which);
                    } else {
                        variant.ends.iter().for_each(|end| {
                            if cost[end.x as usize][end.y as usize] == NODE_WALL {
                                panic!("map {} has end in a wall at {},{}", which, end.x, end.y)
                            }
                        });
                    }
                    if variant.start.x as usize > GRID_HORZ_COUNT || variant.start.y as usize > GRID_HORZ_COUNT {
                        panic!("variant in {} has start outside bounds", which);
                    } else {
                        if cost[variant.start.x as usize][variant.start.y as usize] == NODE_WALL {
                            panic!("map {} has start in a wall at {},{}", which, variant.start.x, variant.start.y)
                        }
                    }
                });

                return Map {
                    variants,
                    cost,
                };
            }
            Err(err) => {
                eprintln!("{}", err);
                panic!("Failed to read map {}", which);
            }
        }
    } else {
        panic!("Map {} missing", which);
    }
}

fn convert_chars_to_coords(chars: Chars) -> Coord {
    let char_array = chars.collect::<Vec<char>>();
    let text = char_array.iter().collect::<String>();
    let mut split = text.split(",");
    let x = split.next();
    let y = split.next();
    let x = x.expect("Invalid coord").parse().expect("Coord not a num");
    let y = y.expect("Invalid coord").parse().expect("Coord not a num");
    return Coord::new(x, y);
}