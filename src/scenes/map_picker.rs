use crate::{Scene, point, GRID_HORZ_COUNT, GRID_VERT_COUNT, SceneParams};
use ggez::{Context, GameError};
use ggez::event::KeyCode;
use crate::data::maps::{Map, read_map_file};
use crate::graphics::renderer::*;
use std::rc::Rc;
use crate::graphics::map_rendering::{draw_map_with_costs, draw_map_with_costs_start_end};
use std::collections::HashMap;

const MAP_CURSOR_ID: &'static str = "map_highlighted";
const VARIANT_CURSOR_ID: &'static str = "variant_highlighted";

pub struct MapPicker {
    maps: Vec<Rc<Map>>,
    selected: Option<usize>,
    highlighted: usize,
    variant_highlighted: usize,
}

impl MapPicker {
    pub fn new(cursor_mem: &HashMap<&str, usize>) -> MapPicker {
        MapPicker {
            maps: vec![],
            selected: None,
            highlighted: *cursor_mem.get(MAP_CURSOR_ID).unwrap_or(&0),
            variant_highlighted: *cursor_mem.get(VARIANT_CURSOR_ID).unwrap_or(&0),
        }
    }
}

impl MapPicker {
    pub fn setup(&mut self, ctx: &mut Context, _renderer: &mut Renderer) -> Result<(), GameError> {
        for i in 0..10 {
            self.maps.push(Rc::new(read_map_file(ctx, i)));
        }

        Ok(())
    }

    fn get_cell_size_for_screen(size: (f32, f32)) -> f32 {
        let cell_w = match size.0 {
            res if res >= 3840. => 24.,
            res if res >= 2560. => 16.,
            res if res >= 1920. => 12.,
            res if res >= 1600. => 10.,
            res if res >= 1280. => 8.,
            res if res >= 1024. => 6.,
            _ => return 5.
        };
        let cell_h = match size.1 {
            res if res >= 2160. => 25.,
            res if res >= 1440. => 16.,
            res if res >= 1080. => 12.,
            res if res >= 768. => 9.,
            _ => return 7.
        };
        return if cell_w < cell_h { cell_w } else { cell_h };
    }
}

impl Scene for MapPicker {
    fn update(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        Ok(())
    }

    fn render(&mut self, ctx: &mut Context, renderer: &mut Renderer) -> Result<(), GameError> {
        let screen_size = renderer.get_screen_size(ctx);
        let cell_size = MapPicker::get_cell_size_for_screen(screen_size);
        let grid_size = (cell_size * GRID_HORZ_COUNT as f32, cell_size * GRID_VERT_COUNT as f32);
        let grid_spacing = screen_size.1 * 0.05;
        let indicator_size = 30.;
        let indicator_spacing = 30.;
        let variant_spacing = (screen_size.0 - (grid_size.0 * 4.) - (indicator_spacing * 2.) - indicator_size) * 0.3;

        let grid_offset = (grid_spacing, (screen_size.1 * 0.5) - (grid_size.1 * 0.5));
        let indicator_pos = (grid_spacing + grid_size.0 + indicator_spacing, (screen_size.1 * 0.5) - (indicator_size * 0.5));
        let variant_offset = (indicator_pos.0 + indicator_size + indicator_spacing + grid_size.0 * 0.5 + variant_spacing, (screen_size.1 * 0.5) - (grid_size.1 * 0.5));

        if self.variant_highlighted > 0 {
            draw_map_with_costs_start_end(ctx, renderer, (variant_offset.0 - grid_size.0 - variant_spacing, variant_offset.1), cell_size, self.maps[self.highlighted].as_ref(), self.variant_highlighted - 1)?;
        }

        draw_map_with_costs_start_end(ctx, renderer, variant_offset, cell_size, self.maps[self.highlighted].as_ref(), self.variant_highlighted)?;

        if self.variant_highlighted < (self.maps[self.highlighted].variants.len() - 1) {
            draw_map_with_costs_start_end(ctx, renderer, (variant_offset.0 + grid_size.0 + variant_spacing, variant_offset.1), cell_size, self.maps[self.highlighted].as_ref(), self.variant_highlighted + 1)?;
            if self.variant_highlighted < (self.maps[self.highlighted].variants.len() - 2) {
                draw_map_with_costs_start_end(ctx, renderer, (variant_offset.0 + (grid_size.0 + variant_spacing) * 2., variant_offset.1), cell_size, self.maps[self.highlighted].as_ref(), self.variant_highlighted + 2)?;
            }
        }

        let grid_background = renderer.make_rect_mesh(ctx, indicator_pos.0 + indicator_spacing * 2., screen_size.1, true, 0.)?;
        renderer.draw_coloured_mesh(ctx, grid_background.as_ref(), point(0., 0.), (0, 0, 0, 255).into());

        if self.highlighted > 0 {
            draw_map_with_costs(ctx, renderer, (grid_offset.0, grid_offset.1 - grid_spacing - grid_size.1), cell_size, self.maps[self.highlighted - 1].as_ref())?;
            if self.highlighted > 1 {
                draw_map_with_costs(ctx, renderer, (grid_offset.0, grid_offset.1 - ((grid_spacing + grid_size.1) * 2.)), cell_size, self.maps[self.highlighted - 2].as_ref())?;
            }
        }

        draw_map_with_costs(ctx, renderer, grid_offset, cell_size, self.maps[self.highlighted].as_ref())?;

        if self.highlighted < (self.maps.len() - 1) {
            draw_map_with_costs(ctx, renderer, (grid_offset.0, grid_offset.1 + grid_spacing + grid_size.1), cell_size, self.maps[self.highlighted + 1].as_ref())?;
            if self.highlighted < (self.maps.len() - 2) {
                draw_map_with_costs(ctx, renderer, (grid_offset.0, grid_offset.1 + (grid_spacing + grid_size.1) * 2.), cell_size, self.maps[self.highlighted + 2].as_ref())?;
            }
        }

        let highlight_mesh = renderer.make_rect_mesh(ctx, grid_size.0 + 8., grid_size.1 + 8., false, 6.)?;
        renderer.draw_coloured_mesh(ctx, highlight_mesh.as_ref(), point(variant_offset.0 - 4., variant_offset.1 - 4.), (0., 1., 1., 1.).into());

        let indicator = renderer.make_list_indicator_mesh(ctx, indicator_size)?;
        renderer.draw_mesh(ctx, indicator.as_ref(), point(indicator_pos.0, indicator_pos.1));

        let grid_shader = renderer.make_rect_mesh(ctx, grid_size.0 * 1.1, grid_size.1, true, 0.)?;
        renderer.draw_coloured_mesh(ctx, grid_shader.as_ref(), point(grid_offset.0 - 10., -grid_spacing), (0., 0., 0., 0.75).into());
        renderer.draw_coloured_mesh(ctx, grid_shader.as_ref(), point(grid_offset.0 - 10., grid_offset.1 - grid_spacing - grid_size.1), (0., 0., 0., 0.4).into());
        renderer.draw_coloured_mesh(ctx, grid_shader.as_ref(), point(grid_offset.0 - 10., grid_offset.1 + grid_spacing + grid_size.1), (0., 0., 0., 0.4).into());
        renderer.draw_coloured_mesh(ctx, grid_shader.as_ref(), point(grid_offset.0 - 10., screen_size.1 - grid_size.1 * 0.8), (0., 0., 0., 0.75).into());

        renderer.draw_white_text(ctx, String::from("Choose map and variant"), point(screen_size.0 / 2., 50.), 48., true);

        Ok(())
    }

    fn on_button_down(&mut self, keycode: KeyCode) {
        match keycode {
            KeyCode::Up => {
                if self.highlighted > 0 {
                    self.highlighted -= 1;
                    self.variant_highlighted = 0;
                }
            }
            KeyCode::Down => {
                if self.highlighted < 9 {
                    self.highlighted += 1;
                    self.variant_highlighted = 0;
                }
            }
            KeyCode::Left => {
                if self.variant_highlighted > 0 {
                    self.variant_highlighted -= 1;
                }
            }
            KeyCode::Right => {
                if self.variant_highlighted < self.maps[self.highlighted].variants.len() - 1 {
                    self.variant_highlighted += 1;
                }
            }
            _ => {}
        }
    }

    fn on_button_up(&mut self, keycode: KeyCode) {
        match keycode {
            KeyCode::Return => {
                self.selected = Some(self.highlighted);
            }
            _ => {}
        }
    }

    fn is_complete(&self) -> bool {
        return self.selected.is_some();
    }

    fn get_next_stage_params(&self, cursor_mem: &mut HashMap<&str, usize>) -> SceneParams {
        cursor_mem.insert(MAP_CURSOR_ID, self.highlighted);
        cursor_mem.insert(VARIANT_CURSOR_ID, self.variant_highlighted);
        return SceneParams::AlgoSelection {
            map: self.maps[self.selected.unwrap()].clone(),
            variant: self.variant_highlighted,
        };
    }
}