use std::rc::Rc;
use crate::Coord;
use crate::algo::*;
use crate::maps::NODE_WALL;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Diagonal {
    Never,
    NoWalls,
    OneWall,
    Always,
}

impl Diagonal {
    pub fn name(&self) -> String {
        return match self {
            Diagonal::Never => String::from("Never"),
            Diagonal::NoWalls => String::from("No walls"),
            Diagonal::OneWall => String::from("Only one wall"),
            Diagonal::Always => String::from("Always"),
        };
    }

    pub fn len() -> usize {
        4
    }

    pub fn from_index(idx: usize) -> Diagonal {
        return match idx {
            0 => Diagonal::Never,
            1 => Diagonal::NoWalls,
            2 => Diagonal::OneWall,
            3 => Diagonal::Always,
            _ => panic!("Invalid index: {}", idx),
        };
    }

    pub fn max_walls(&self) -> usize {
        match self {
            Diagonal::Never => 0,
            Diagonal::NoWalls => 0,
            Diagonal::OneWall => 1,
            Diagonal::Always => 2,
        }
    }
}

impl Diagonal {
    pub fn get_neighbours(&self, cost_calc: Rc<Box<CostCalc>>, xy: Coord) -> Vec<Coord> {
        let mut results = vec![];

        self.add_cardinal(Direction::Top, cost_calc.clone(), xy.clone(), &mut results);
        self.add_cardinal(Direction::Bottom, cost_calc.clone(), xy.clone(), &mut results);
        self.add_cardinal(Direction::Left, cost_calc.clone(), xy.clone(), &mut results);
        self.add_cardinal(Direction::Right, cost_calc.clone(), xy.clone(), &mut results);

        if self != &Diagonal::Never {
            self.add_diagonal(Direction::TopRight, cost_calc.clone(), xy.clone(), &mut results);
            self.add_diagonal(Direction::BottomRight, cost_calc.clone(), xy.clone(), &mut results);
            self.add_diagonal(Direction::TopLeft, cost_calc.clone(), xy.clone(), &mut results);
            self.add_diagonal(Direction::BottomLeft, cost_calc.clone(), xy.clone(), &mut results);
        }

        return results;
    }

    fn add_cardinal(&self, direction: Direction, cost_calc: Rc<Box<CostCalc>>, xy: Coord, results: &mut Vec<Coord>) {
        let new_cell = direction.convert_to_coord(xy);
        if cost_calc(new_cell) != NODE_WALL {
            results.push(new_cell);
        }
    }

    fn add_diagonal(&self, direction: Direction, cost_calc: Rc<Box<CostCalc>>, xy: Coord, results: &mut Vec<Coord>) {
        let new_cell = direction.convert_to_coord(xy);
        if cost_calc(new_cell) != NODE_WALL {
            let neighbours = direction.convert_direction_to_neighbours(xy);
            let mut wall_count = 0;
            if cost_calc(neighbours[0]) == NODE_WALL { wall_count += 1; }
            if cost_calc(neighbours[1]) == NODE_WALL { wall_count += 1; }
            if wall_count <= self.max_walls() {
                results.push(new_cell);
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    TopLeft,
    Top,
    TopRight,
    Right,
    BottomRight,
    Bottom,
    BottomLeft,
    Left,
}

impl Direction {
    fn convert_to_coord(&self, xy: Coord) -> Coord {
        return match self {
            Direction::Top => Coord::new(xy.x, xy.y - 1),
            Direction::Right => Coord::new(xy.x + 1, xy.y),
            Direction::Left => Coord::new(xy.x - 1, xy.y),
            Direction::Bottom => Coord::new(xy.x, xy.y + 1),
            Direction::TopRight => Coord::new(xy.x + 1, xy.y - 1),
            Direction::TopLeft => Coord::new(xy.x - 1, xy.y - 1),
            Direction::BottomRight => Coord::new(xy.x + 1, xy.y + 1),
            Direction::BottomLeft => Coord::new(xy.x - 1, xy.y + 1),
        };
    }
    fn convert_direction_to_neighbours(&self, xy: Coord) -> Vec<Coord> {
        return match self {
            Direction::Top => vec![Coord::new(xy.x - 1, xy.y - 1), Coord::new(xy.x + 1, xy.y - 1)],
            Direction::Right => vec![Coord::new(xy.x + 1, xy.y - 1), Coord::new(xy.x + 1, xy.y + 1)],
            Direction::Left => vec![Coord::new(xy.x - 1, xy.y - 1), Coord::new(xy.x - 1, xy.y + 1)],
            Direction::Bottom => vec![Coord::new(xy.x - 1, xy.y + 1), Coord::new(xy.x + 1, xy.y + 1)],
            Direction::TopRight => vec![Coord::new(xy.x, xy.y - 1), Coord::new(xy.x + 1, xy.y)],
            Direction::TopLeft => vec![Coord::new(xy.x - 1, xy.y), Coord::new(xy.x, xy.y - 1)],
            Direction::BottomRight => vec![Coord::new(xy.x, xy.y + 1), Coord::new(xy.x + 1, xy.y)],
            Direction::BottomLeft => vec![Coord::new(xy.x - 1, xy.y), Coord::new(xy.x, xy.y + 1)],
        };
    }
}
