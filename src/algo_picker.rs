use crate::{Scene, point, SceneParams};
use ggez::{Context, GameError};
use ggez::event::KeyCode;
use crate::maps::{Map};
use crate::renderer::*;
use crate::Algo;
use std::rc::Rc;

pub struct AlgoPicker {
    selected_map: Rc<Map>,
    selected: Option<usize>,
    highlighted: usize
}

impl AlgoPicker {
    pub fn new(map: Rc<Map>) -> AlgoPicker {
        AlgoPicker {
            selected_map: map,
            selected: None,
            highlighted: 0
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

    fn on_button_press(&mut self, keycode: KeyCode) {
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
            KeyCode::Return => self.selected = Some(self.highlighted),
            _ => {}
        }
    }

    fn is_complete(&self) -> bool {
        return self.selected.is_some();
    }

    fn get_next_stage_params(&self) -> SceneParams {
        SceneParams::DiagonalSelection {
            map: self.selected_map.clone(),
            algo: Algo::from_index(self.selected.expect("Nothing selected"))
        }
    }
}