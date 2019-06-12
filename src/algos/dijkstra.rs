use std::rc::Rc;
use crate::data::{Coord, Node};
use crate::data::diagonal::Diagonal;
use crate::std_ext::RcBreaker;
use crate::algos::{Algorithm, AlgoStatus};
use crate::algos::AlgoStatus::*;

pub type CostCalc = Fn(Coord) -> i32;
pub type IsValidEnd = Fn(Coord) -> bool;

pub struct Dijkstra {
    diagonal: Diagonal,
    width: i32,
    height: i32,
    open_nodes: Vec<Rc<Node>>,
    closed_nodes: Vec<Rc<Node>>,
    cost_calc: Rc<Box<CostCalc>>,
    is_valid_end: Box<IsValidEnd>,
    status: AlgoStatus,
}

impl Dijkstra {
    pub fn new_fixed_target(start: Coord, ends: Vec<Coord>, cost_calc: Box<CostCalc>, width: i32, height: i32, diagonal: Diagonal) -> Dijkstra {
        let end_clone = ends.clone();
        let rc_cost_calc = Rc::new(cost_calc);
        Dijkstra {
            width,
            height,
            diagonal,
            open_nodes: vec![Rc::new(start.into())],
            closed_nodes: vec![],
            cost_calc: rc_cost_calc.clone(),
            is_valid_end: Box::new(move |xy| end_clone.contains(&xy)),
            status: AlgoStatus::InProgress((vec![], vec![])),
        }
    }
}

impl Dijkstra {
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
        for offset in self.diagonal.get_neighbours(self.cost_calc.clone(), current_node.clone().xy).iter() {
            let new_pos: Coord =  offset.clone();

            if new_pos.is_out_of_bounds(self.width, self.height) { continue; }

            if (self.cost_calc)(new_pos) < 0 { continue; }

            let node = Node::new(new_pos, Some(current_node.clone()));

            children.push(node);
        }

        for mut child in children {
            if self.open_nodes.contains_item(&child) { continue; }
            if self.closed_nodes.contains_item(&child) { continue; }

            child.g = current_node.clone().g + 1;
            child.h = 0;
            child.f = child.g + child.h + ((self.cost_calc)(child.xy) * 5);

            let is_larger = self.open_nodes.iter()
                .any(|item| &child == item && child.f > item.f);
            if is_larger { continue; }

            self.open_nodes.push(Rc::new(child));
        }

        self.status = InProgress((self.open_nodes.iter().map(|node| node.xy.clone()).collect(),
                                  self.closed_nodes.iter().map(|node| node.xy.clone()).collect()));
    }
}

impl Algorithm for Dijkstra {
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