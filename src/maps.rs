use crate::models::Coord;
use ggez::{Context, filesystem};
use std::io::Read;
use crate::{GRID_VERT_COUNT, GRID_HORZ_COUNT};

pub struct Map {
    pub idx: usize,
    pub start: Coord,
    pub targets: Vec<Coord>,
    pub cost: Vec<Vec<i32>>,
    pub info: String,
}

impl Map {
    pub fn get_column_count(&self) -> usize {
        self.cost[0].len()
    }

    pub fn get_row_count(&self) -> usize {
        self.cost.len()
    }
}

pub fn read_map_file(ctx: &mut Context, which: usize) -> Map {
    if let Ok(mut file) = filesystem::open(ctx, format!("/map{}", which)) {
        let mut buffer = String::new();
        match file.read_to_string(&mut buffer) {
            Ok(result) => {
                if (result + 1) < GRID_VERT_COUNT * GRID_HORZ_COUNT {
                    println!("length: {}, required: {} (newlines are ignored)", result, GRID_VERT_COUNT * GRID_HORZ_COUNT);
                    panic!("Map {} is too small", which);
                }

                let mut start = Coord { x: -1, y: -1 };
                let mut targets = vec![];
                let mut cost = vec![vec![0; GRID_VERT_COUNT]; GRID_HORZ_COUNT];

                let mut x = 0_usize;
                let mut y = 0_usize;


                let (map_chars, info_chars) = buffer.split_at(GRID_VERT_COUNT * (GRID_HORZ_COUNT + 1));
                let chars = map_chars.chars();
                for letter in chars {
                    match letter {
                        's' => {
                            if start.x == -1 {
                                start = Coord { x: x as i32, y: y as i32 }
                            } else {
                                panic!("More than one start found in map {}", which);
                            }
                        }
                        't' => targets.push(Coord { x: x as i32, y: y as i32 }),
                        '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' => cost[x][y] = letter.to_digit(10).unwrap() as i32,
                        '9' => cost[x][y] = -1,
                        _ => {}
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

                let info = info_chars.to_string();

                if start.x < 0 || start.y < 0 {
                    panic!("No start found in map {}", which);
                }

                if targets.is_empty() {
                    panic!("No targets found in map {}", which);
                }

                return Map {
                    idx: which,
                    start,
                    targets,
                    cost,
                    info,
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