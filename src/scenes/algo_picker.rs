use crate::{point};
use crate::scenes::{Scene, SceneParams};
use ggez::{Context, GameError};
use ggez::event::KeyCode;
use crate::data::maps::Map;
use crate::graphics::renderer::*;
use crate::algos::Algo;
use std::rc::Rc;

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
    pub fn new(map: Rc<Map>, variant: usize) -> AlgoPicker {
        AlgoPicker {
            params: AlgoParams { map, variant },
            selected: None,
            highlighted: 0,
        }
    }
}

impl Scene for AlgoPicker {
    fn update(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        Ok(())
    }

    fn render(&mut self, ctx: &mut Context, renderer: &mut Renderer) -> Result<(), GameError> {
        let offset = (360., 150.);
        let line_height = 50.;

        let list_mesh = renderer.make_list_indicator_mesh(ctx, 30.)?;

        for i in 0..Algo::len() {
            renderer.draw_white_text(ctx, Algo::from_index(i).name(), point(offset.0, offset.1 + (line_height * i as f32)), 48., false);
        }

        renderer.draw_mesh(ctx, list_mesh.as_ref(), point(310., offset.1 + 8. + (self.highlighted as f32 * line_height)));

        renderer.draw_white_text(ctx, String::from("Choose an algorithm"), point(360., 48.), 48., false);

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

    fn get_next_stage_params(&self) -> SceneParams {
        SceneParams::DiagonalSelection {
            map: self.params.map.clone(),
            variant: self.params.variant,
            algo: Algo::from_index(self.selected.expect("Nothing selected")),
        }
    }
}