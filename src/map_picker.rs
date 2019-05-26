use crate::{Scene, point, GRID_HORZ_COUNT, GRID_VERT_COUNT, SceneParams};
use ggez::{Context, GameError};
use ggez::event::KeyCode;
use crate::maps::{Map, read_map_file};
use crate::renderer::*;
use crate::ggez_ext::{is_keycode_a_number, keycode_to_num};
use std::rc::Rc;

pub struct MapPicker {
    maps: Vec<Rc<Map>>,
    selected: Option<usize>,
//    map_graphic: //use canvas
}

impl MapPicker {
    pub fn new() -> MapPicker {
        MapPicker {
            maps: vec![],
            selected: None,
        }
    }
}

impl MapPicker {
    pub fn setup(&mut self, ctx: &mut Context) {
        for i in 0..10 {
            self.maps.push(Rc::new(read_map_file(ctx, i)));
        }
    }
}

impl Scene for MapPicker {
    fn update(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        Ok(())
    }

    fn render(&mut self, ctx: &mut Context, renderer: &mut Renderer) -> Result<(), GameError> {
        let cell_size = 10.;
        let grid_offset = (60., 400.);
        let grid_spacing = (50., 100.);
        let grid_size = (cell_size * GRID_HORZ_COUNT as f32, cell_size * GRID_VERT_COUNT as f32);
        let grid_mesh = renderer.make_grid_mesh(ctx, cell_size, GRID_HORZ_COUNT, GRID_VERT_COUNT, 150)?;
        let square_mesh = renderer.make_square_mesh(ctx, cell_size, true)?;
        for x in 0..5 {
            for y in 0..2 {
                let grid_x = x as f32 * (grid_size.0 + grid_spacing.0) + grid_offset.0;
                let grid_y = y as f32 * (grid_size.1 + grid_spacing.1) + grid_offset.1;
                renderer.draw_mesh(ctx, grid_mesh.as_ref(), point(grid_x, grid_y));
                let map_idx = x + y * 5;
                for map_x in 0..GRID_HORZ_COUNT {
                    for map_y in 0..GRID_VERT_COUNT {
                        if !self.maps[map_idx].passable[map_x][map_y] {
                            renderer.draw_mesh(ctx, square_mesh.as_ref(), point(grid_x + (map_x as f32 * cell_size), grid_y + (map_y as f32 * cell_size)));
                        }
                    }
                }
                renderer.draw_coloured_mesh(ctx, square_mesh.as_ref(), point(grid_x + (self.maps[map_idx].start.x as f32 * cell_size), grid_y + (self.maps[map_idx].start.y as f32 * cell_size)), (0.5, 1., 0.5, 1.).into());
                for target in &self.maps[map_idx].targets {
                    renderer.draw_coloured_mesh(ctx, square_mesh.as_ref(), point(grid_x + (target.x as f32 * cell_size), grid_y + (target.y as f32 * cell_size)), (0.5, 0.5, 1., 1.).into());
                }
            }
        }

        renderer.draw_text(ctx, String::from("Choose a map (0-9)"), point(750., 200.));

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
        return SceneParams::AlgoSelection {
            map: self.maps[self.selected.unwrap()].clone()
        };
    }
}