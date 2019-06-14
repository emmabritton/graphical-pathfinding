use crate::data::Coord;
use std::cmp::max;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Heuristic {
    None,
    Manhattan,
    Euclidean,
    Chebyshev,
    Mine
}

trait Abs {
    fn abs(self) -> Self;
}

impl Abs for i32 {
    fn abs(self) -> Self {
        return if self < 0 { self * -1 } else { self };
    }
}

impl Heuristic {
    pub fn name(&self) -> String {
        return match self {
            Heuristic::None => String::from("Always 0"),
            Heuristic::Manhattan => String::from("Manhattan"),
            Heuristic::Euclidean => String::from("Euclidean"),
            Heuristic::Chebyshev => String::from("Chebyshec"),
            Heuristic::Mine => String::from("Fast but less accurate"),
        };
    }

    pub fn len() -> usize {
        5
    }

    pub fn from_index(idx: usize) -> Heuristic {
        return match idx {
            0 => Heuristic::None,
            1 => Heuristic::Manhattan,
            2 => Heuristic::Euclidean,
            3 => Heuristic::Chebyshev,
            4 => Heuristic::Mine,
            _ => panic!("Invalid index: {}", idx),
        };
    }
}


impl Heuristic {
    pub fn calc_multiple(&self, current: &Coord, ends: &Vec<Coord>) -> i32 {
        return ends.iter().map(|end| self.calc_fixed(current, end)).sum();
    }

    pub fn calc_fixed(&self, current: &Coord, end: &Coord) -> i32 {
        return self.calc((current.x - end.x).abs(), (current.y - end.y).abs());
    }

    pub fn calc(&self, dx: i32, dy: i32) -> i32 {
        match self {
            Heuristic::None => return 0,
            Heuristic::Manhattan => return dx + dy,
            Heuristic::Euclidean => return ((dx * dx + dy * dy) as f32).sqrt() as i32,
            Heuristic::Chebyshev => return max(dx, dy),
            Heuristic::Mine => return dx.pow(2) + dy.pow(2)
        }
    }
}