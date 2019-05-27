use std::rc::Rc;
use crate::models::{Coord, Node};
use crate::std_ext::RcBreaker;
use crate::Algorithm;
use crate::AlgoStatus;
use crate::AlgoStatus::*;

pub type IsPassable = Fn(Coord) -> bool;
pub type IsValidEnd = Fn(Coord) -> bool;
pub type HeuristicCalc = Fn(Coord) -> i32;

pub struct Astar {
    diagonal_movement_allowed: bool,
    width: i32,
    height: i32,
    open_nodes: Vec<Rc<Node>>,
    closed_nodes: Vec<Rc<Node>>,
    is_passable: Box<IsPassable>,
    is_valid_end: Box<IsValidEnd>,
    heuristic_calc: Box<HeuristicCalc>,
    status: AlgoStatus,
    allowed_neighbours: Vec<(i32,i32)>
}

fn get_neighbours(diagonals_allowed: bool) -> Vec<(i32,i32)> {
    if diagonals_allowed {
        vec![(0, -1), (0, 1), (-1, 0), (1, 0), (-1, -1), (-1, 1), (1, -1), (1, 1)]
    } else {
        vec![(0, -1), (0, 1), (-1, 0), (1, 0)]
    }
}

impl Astar {
    pub fn new_fixed_target(start: Coord, ends: Vec<Coord>, is_passable: Box<IsPassable>, width: i32, height: i32, diagonal_movement_allowed: bool) -> Astar {
        let end_clone = ends.clone();
        Astar {
            width,
            height,
            diagonal_movement_allowed,
            open_nodes: vec![Rc::new(start.into())],
            closed_nodes: vec![],
            is_passable,
            is_valid_end: Box::new(move |xy| end_clone.contains(&xy)),
            heuristic_calc: Box::new(move |xy|
                                         ends.iter().map(|end| ((xy.x - end.x).pow(2) + (xy.y - end.y).pow(2))).sum() //?
            ),
            status: AlgoStatus::InProgress((vec![], vec![])),
            allowed_neighbours: get_neighbours(diagonal_movement_allowed)
        }
    }

    pub fn new_open_target(start: Coord, is_valid_end: Box<IsValidEnd>, heuristic_calc: Box<HeuristicCalc>, is_passable: Box<IsPassable>, width: i32, height: i32, diagonal_movement_allowed: bool) -> Astar {
        Astar {
            width,
            height,
            diagonal_movement_allowed,
            open_nodes: vec![Rc::new(start.into())],
            closed_nodes: vec![],
            is_passable,
            is_valid_end,
            heuristic_calc,
            status: AlgoStatus::InProgress((vec![], vec![])),
            allowed_neighbours: get_neighbours(diagonal_movement_allowed)
        }
    }
}

impl Astar {
    fn process_once(&mut self) {
        if self.open_nodes.is_empty() {
            self.status = NoPath;
            return;
        }

        let (idx, _) = self.open_nodes.iter()
            .enumerate()
            .min_by(|&lhs: &(usize, &Rc<Node>), &rhs: &(usize, &Rc<Node>)| lhs.1.f.cmp(&rhs.1.f))
            .unwrap();

        let current_node = self.open_nodes.remove(idx);

        if (self.is_valid_end)(current_node.xy) {
            let mut path = vec![];
            let mut current = Some(current_node);
            while current.is_some() {
                let current1 = current.clone().unwrap();
                let current2 = current.clone().unwrap();
                path.push(current1);
                current = current2.parent.clone();
            }
            let result: Vec<Coord> = path.iter()
                .rev()
                .map(|item| item.xy)
                .collect();

            self.status = Found(result);
            return;
        }

        self.closed_nodes.push(current_node.clone());

        let mut children = vec![];
        for offset in self.allowed_neighbours.iter() {
            let new_pos: Coord = current_node.clone().xy + Coord { x: offset.0, y: offset.1 };

            if new_pos.is_out_of_bounds(self.width, self.height) { continue; }

            if !(self.is_passable)(new_pos) { continue; }

            let node = Node::new(new_pos, Some(current_node.clone()));

            children.push(node);
        }

        for mut child in children {
            if self.open_nodes.contains_item(&child) { continue; }
            if self.closed_nodes.contains_item(&child) { continue; }

            child.g = current_node.clone().g + 1;
            child.h = (self.heuristic_calc)(child.xy);
            child.f = child.g + child.h;

            let is_larger = self.open_nodes.iter()
                .any(|item| &child == item && child.g > item.g);
            if is_larger { continue; }

            self.open_nodes.push(Rc::new(child));
        }

        self.status = InProgress((self.open_nodes.iter().map(|node| node.xy.clone()).collect(),
                                  self.closed_nodes.iter().map(|node| node.xy.clone()).collect()));
    }
}

impl Algorithm for Astar {
    fn tick(&mut self) {

        match self.status {
            AlgoStatus::InProgress(_) => self.process_once(),
            _ => {
                //do nothing
            }
        }
    }

    fn get_data(&self) -> &AlgoStatus {
        return &self.status;
    }
}