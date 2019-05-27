use crate::{Scene, point, GRID_HORZ_COUNT, GRID_VERT_COUNT, SceneParams, Astar, Coord, Algorithm};
use ggez::{Context, GameError};
use ggez::event::KeyCode;
use crate::maps::{Map, read_map_file};
use crate::renderer::*;
use crate::ggez_ext::{is_keycode_a_number, keycode_to_num};
use std::rc::Rc;
use std::cell::RefCell;

pub struct AlgoPicker {
    selected_map: Rc<Map>,
    selected: Option<usize>,
}

impl AlgoPicker {
    pub fn new(map: Rc<Map>) -> AlgoPicker {
        AlgoPicker {
            selected_map: map,
            selected: None,
        }
    }
}

impl Scene for AlgoPicker {
    fn update(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        Ok(())
    }

    fn render(&mut self, ctx: &mut Context, renderer: &mut Renderer) -> Result<(), GameError> {
        let offset = (48., 104.);
        let line_height = 48.;

        let algos = [
            String::from("A* - no diagonals"),
            String::from("A* - diagonals allowed")
        ];

        algos.iter()
            .enumerate()
            .for_each(|(idx, algo)| {
                renderer.draw_text(ctx, format!("{}) {}", idx, algo), point(offset.0, offset.1 + (line_height * idx as f32)));
            });

        renderer.draw_text(ctx, String::from("Choose an algorithm"), point(48., 48.));

        Ok(())
    }

    fn on_button_press(&mut self, keycode: KeyCode) {
        if is_keycode_a_number(keycode) {
            let number = keycode_to_num(keycode);
            println!("Map {} selected", number);
            self.selected = Some(number)
        }
    }

    fn is_complete(&self) -> bool {
        return self.selected.is_some();
    }

    fn get_next_stage_params(&self) -> SceneParams {
        let map = self.selected_map.clone();
        let map_clone = self.selected_map.clone();
        let cost_calc = Box::new(move |xy: Coord| {
            if xy.is_out_of_bounds(GRID_HORZ_COUNT as i32, GRID_VERT_COUNT as i32) {
                -1
            } else {
                map_clone.cost[xy.x as usize][xy.y as usize]
            }
        });
        let algo = match self.selected.unwrap() {
            0 => Astar::new_fixed_target(map.start, map.targets.clone(), cost_calc, GRID_HORZ_COUNT as i32, GRID_VERT_COUNT as i32, false),
            1 => Astar::new_fixed_target(map.start, map.targets.clone(), cost_calc, GRID_HORZ_COUNT as i32, GRID_VERT_COUNT as i32, true),
            _ => panic!("Invalid algo: {}", self.selected.unwrap())
        };
        let algo_name = match self.selected.unwrap() {
            0 => String::from("A*"),
            1 => String::from("A* w/ diagonals"),
            _ => panic!("Invalid algo: {}", self.selected.unwrap())
        };
        return SceneParams::AlgoRunner {
            map: self.selected_map.clone(),
            algo: Rc::new(RefCell::new(algo)),
            algo_name
        };
    }
}