use ggez::{Context, GameError};
use ggez::event::KeyCode;
use crate::point;
use crate::data::maps::Map;
use crate::data::Coord;
use crate::graphics::renderer::Renderer;
use crate::data::diagonal::Diagonal;
use crate::algos::{Algo, Algorithm};
use crate::algos::astar::Astar;
use crate::data::heuristic::Heuristic;
use crate::algos::dijkstra::Dijkstra;
use crate::scenes::{Scene, SceneParams};
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

const CURSOR_ID: &'static str = "heuristic_highlighted";

pub struct HeuristicParams {
    map: Rc<Map>,
    algo: Algo,
    diagonal: Diagonal,
    variant: usize,
}

pub struct HeuristicPicker {
    params: HeuristicParams,
    selected: Option<usize>,
    highlighted: usize,
}

impl HeuristicPicker {
    pub fn new(map: Rc<Map>, algo: Algo, diagonal: Diagonal, variant: usize, cursor_mem: &HashMap<&str, usize>) -> HeuristicPicker {
        let params = HeuristicParams {
            map,
            algo,
            diagonal,
            variant,
        };
        let highlighted = *cursor_mem.get(CURSOR_ID).unwrap_or(&0);
        if params.algo.supported_heuristics() {
            return HeuristicPicker {
                params,
                selected: None,
                highlighted,
            };
        } else {
            return HeuristicPicker {
                params,
                selected: Some(0),
                highlighted,
            };
        }
    }
}

impl Scene for HeuristicPicker {
    fn update(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        Ok(())
    }

    fn render(&mut self, ctx: &mut Context, renderer: &mut Renderer) -> Result<(), GameError> {
        let title_offset = point(360., 50.);
        let text_offset = point(360., 150.);
        let text_spacing = 50.;

        let list_mesh = renderer.make_list_indicator_mesh(ctx, 30.)?;

        renderer.draw_white_text(ctx, "Choose a heuristic", title_offset, 48., false);

        for i in 0..Heuristic::len() {
            renderer.draw_white_text(ctx, Heuristic::from_index(i).name(), point(text_offset.x, text_offset.y + (text_spacing * i as f32)), 48., false);
        }

        renderer.draw_mesh(ctx, list_mesh.as_ref(), point(text_offset.x - 50., text_offset.y + 8. + (self.highlighted as f32 * text_spacing)));

        Ok(())
    }

    fn on_button_down(&mut self, keycode: KeyCode) {
        match keycode {
            KeyCode::Up => {
                if self.highlighted > 0 {
                    self.highlighted -= 1;
                }
            }
            KeyCode::Down => {
                if self.highlighted < Heuristic::len() - 1 {
                    self.highlighted += 1;
                }
            }
            _ => {}
        }
    }

    fn on_button_up(&mut self, keycode: KeyCode) {
        match keycode {
            KeyCode::Return => self.selected = Some(self.highlighted),
            _ => {}
        }
    }

    fn is_complete(&self) -> bool {
        return self.selected.is_some();
    }

    fn get_next_stage_params(&self, cursor_mem: &mut HashMap<&str, usize>) -> SceneParams {
        cursor_mem.insert(CURSOR_ID, self.highlighted);
        let map_clone = self.params.map.clone();
        let columns = map_clone.get_column_count() as i32;
        let rows = map_clone.get_row_count() as i32;
        let heuristic = Heuristic::from_index(self.selected.expect("Nothing selected"));
        let cost_calc = Box::new(move |xy: Coord| {
            if xy.is_out_of_bounds(columns, rows) {
                -1
            } else {
                map_clone.cost[xy.x as usize][xy.y as usize]
            }
        });
        let algo: Box<Algorithm> = match self.params.algo {
            Algo::AStar => Box::new(Astar::new_fixed_target(self.params.map.variants[self.params.variant].start, self.params.map.variants[self.params.variant].ends.clone(), cost_calc, columns, rows, self.params.diagonal, heuristic)),
            Algo::Dijkstra => Box::new(Dijkstra::new_fixed_target(self.params.map.variants[self.params.variant].start, self.params.map.variants[self.params.variant].ends.clone(), cost_calc, columns, rows, self.params.diagonal))
        };
        SceneParams::AlgoRunner {
            map: self.params.map.clone(),
            heuristic,
            algo: Rc::new(RefCell::new(algo)),
            algo_name: self.params.algo.name(),
            diagonal: self.params.diagonal,
            variant: self.params.variant,
        }
    }
}