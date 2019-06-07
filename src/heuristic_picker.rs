use crate::{Scene, SceneParams, Coord, Algorithm, point, DPPoint};
use ggez::{Context, GameError, GameResult};
use ggez::event::KeyCode;
use crate::maps::Map;
use crate::renderer::*;
use crate::Diagonal;
use crate::Algo;
use crate::Astar;
use crate::heuristic::Heuristic;
use crate::dijkstra::Dijkstra;
use crate::map_rendering::draw_map_with_costs_path;
use std::rc::Rc;
use std::cell::RefCell;

pub struct HeuristicPicker {
    params: (Rc<Map>, Algo, Diagonal),
    selected: Option<usize>,
    highlighted: usize,
}

impl HeuristicPicker {
    pub fn new(params: (Rc<Map>, Algo, Diagonal)) -> HeuristicPicker {
        if (params.1.supported_heuristics()) {
            return HeuristicPicker {
                params,
                selected: None,
                highlighted: 0,
            };
        } else {
            return HeuristicPicker {
                params,
                selected: Some(0),
                highlighted: 0
            }
        }
    }
}

impl Scene for HeuristicPicker {
    fn update(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        Ok(())
    }

    fn render(&mut self, ctx: &mut Context, renderer: &mut Renderer) -> Result<(), GameError> {
        let title_offset = point(360., 50.);
        let text_offset = point(360., 150.);
        let text_spacing = 50.;

        let cell_size = 50.;
        let grid_width = 4. * cell_size;
        let grid_spacing = 300.;
        let grid_offset = point(390., 700.);

        let marker_offset = point(grid_offset.x + 55., grid_offset.y + grid_width * 0.9);
        let marker_spacing = (grid_spacing + grid_width) as f32;
        let size = 60.;

        let list_mesh = renderer.make_list_indicator_mesh(ctx, 30.)?;

        renderer.draw_white_text(ctx, "Choose a heuristic", title_offset, 48., false);

        for i in 0..Heuristic::len() {
            renderer.draw_white_text(ctx, Heuristic::from_index(i).name(), point(text_offset.x, text_offset.y + (text_spacing * i as f32)), 48., false);
        }

        renderer.draw_mesh(ctx, list_mesh.as_ref(), point(text_offset.x - 50., text_offset.y + 8. + (self.highlighted as f32 * text_spacing)));

        Ok(())
    }

    fn on_button_press(&mut self, keycode: KeyCode) {
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
            KeyCode::Return => self.selected = Some(self.highlighted),
            _ => {}
        }
    }

    fn is_complete(&self) -> bool {
        return self.selected.is_some();
    }

    fn get_next_stage_params(&self) -> SceneParams {
        let map_clone = self.params.0.clone();
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
        let algo: Box<Algorithm> = match self.params.1 {
            Algo::AStar => Box::new(Astar::new_fixed_target(self.params.0.start, self.params.0.targets.clone(), cost_calc, columns, rows, self.params.2, heuristic)),
            Algo::Dijkstra => Box::new(Dijkstra::new_fixed_target(self.params.0.start, self.params.0.targets.clone(), cost_calc, columns, rows, self.params.2))
        };
        SceneParams::AlgoRunner {
            map: self.params.0.clone(),
            heuristic,
            algo: Rc::new(RefCell::new(algo)),
            algo_name: self.params.1.name(),
            diagonal: self.params.2
        }
    }
}