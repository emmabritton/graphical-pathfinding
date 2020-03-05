use crate::{point};
use crate::scenes::{Scene, SceneParams};
use ggez::{Context, GameError};
use ggez::event::KeyCode;
use crate::data::maps::Map;
use crate::graphics::renderer::*;
use crate::algos::Algo;
use std::rc::Rc;
use std::collections::HashMap;

const CURSOR_ID: &'static str = "algo_highlighted";

struct AlgoParams {
    map: Rc<Map>,
    variant: usize,
}

pub struct AlgoPicker {
    params: AlgoParams,
    selected: Option<usize>,
    highlighted: usize,
}

impl AlgoPicker {
    pub fn new(map: Rc<Map>, variant: usize, cursor_mem: &HashMap<&str, usize>) -> AlgoPicker {
        AlgoPicker {
            params: AlgoParams { map, variant },
            selected: None,
            highlighted: *cursor_mem.get(CURSOR_ID).unwrap_or(&0)
        }
    }
}

impl Scene for AlgoPicker {
    fn update(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        Ok(())
    }

    fn render(&mut self, ctx: &mut Context, renderer: &mut Renderer) -> Result<(), GameError> {
        let offset = renderer.calc_percent_to_px(0.19, 0.14);
        let line_height = renderer.calc_height(0.05);

        let list_mesh = renderer.make_list_indicator_mesh(ctx, renderer.calc_height(0.03))?;

        for i in 0..Algo::len() {
            renderer.draw_white_text(ctx, Algo::from_index(i).name(), point(offset.0, offset.1 + (line_height * i as f32)), renderer.calc_height(0.04), false);
        }

        renderer.draw_mesh(ctx, list_mesh.as_ref(), point(renderer.calc_width(0.16), offset.1 + renderer.calc_height(0.008) + (self.highlighted as f32 * line_height)));

        renderer.draw_white_text(ctx, String::from("Choose an algorithm"), renderer.calc_percent_to_point(0.19, 0.04), renderer.calc_height(0.04), false);

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
                if self.highlighted < Algo::len() - 1 {
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
        SceneParams::DiagonalSelection {
            map: self.params.map.clone(),
            variant: self.params.variant,
            algo: Algo::from_index(self.selected.expect("Nothing selected")),
        }
    }
}