use std::ops::Add;
use std::ops::Sub;
use std::fmt::{Display, Formatter};
use std::fmt;
use std::rc::Rc;

#[derive(Debug, Default, Ord, Eq, PartialOrd, Clone, Copy)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

impl Coord {
    pub fn new(x: i32, y: i32) -> Coord {
        Coord { x, y }
    }
}

impl Coord {
    pub fn is_out_of_bounds(&self, max_x: i32, max_y: i32) -> bool {
        if self.x < 0 || self.x >= max_x { return true; }
        if self.y < 0 || self.y >= max_y { return true; }
        return false;
    }
}

impl PartialEq for Coord {
    fn eq(&self, other: &Coord) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl PartialEq<Rc<Coord>> for Coord {
    fn eq(&self, other: &Rc<Coord>) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Display for Coord {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

impl Into<(i32, i32)> for Coord {
    fn into(self) -> (i32, i32) {
        (self.x, self.y)
    }
}

impl Into<Coord> for (i32, i32) {
    fn into(self) -> Coord {
        Coord { x: self.0, y: self.1 }
    }
}

impl Into<Coord> for (u32, u32) {
    fn into(self) -> Coord {
        Coord { x: self.0 as i32, y: self.1 as i32 }
    }
}

impl Add<(i32, i32)> for Coord {
    type Output = Coord;

    fn add(self, rhs: (i32, i32)) -> Self::Output {
        Coord {
            x: self.x + rhs.0,
            y: self.y + rhs.1,
        }
    }
}

impl Add<Coord> for Coord {
    type Output = Coord;

    fn add(self, rhs: Coord) -> Self::Output {
        Coord {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<&Coord> for Coord {
    type Output = Coord;

    fn add(self, rhs: &Coord) -> Self::Output {
        Coord {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Coord {
    type Output = Coord;

    fn sub(self, rhs: Coord) -> Self::Output {
        Coord {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[derive(Debug, Eq, Default, Clone)]
pub struct Node {
    pub xy: Coord,
    //distance to start node
    pub g: i32,
    //heuristic — estimated distance to end node
    pub h: i32,
    //node cost
    pub f: i32,
    pub parent: Option<Rc<Node>>,
}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.xy == other.xy
    }
}

impl PartialEq<Rc<Node>> for Node {
    fn eq(&self, other: &Rc<Node>) -> bool {
        self.xy == other.xy
    }
}

impl Node {
    pub fn new(xy: Coord, parent: Option<Rc<Node>>) -> Node {
        return Node {
            xy,
            parent,
            ..Node::default()
        };
    }
}

impl From<Coord> for Node {
    fn from(coord: Coord) -> Self {
        Node::new(coord, None)
    }
}