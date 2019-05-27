use crate::{Scene, point, GRID_HORZ_COUNT, GRID_VERT_COUNT, SCREEN_WIDTH, SCREEN_HEIGHT, SceneParams};
use ggez::{Context, GameError};
use ggez::event::KeyCode;
use crate::maps::{Map, read_map_file};
use crate::renderer::*;
use std::rc::Rc;
use ggez::mint::Vector2;
use ggez::graphics;
use ggez::graphics::{Canvas, Image, DrawParam};

pub struct MapPicker {
    maps: Vec<Rc<Map>>,
    selected: Option<usize>,
    highlighted: usize,
    map_image: Option<Image>,
}

impl MapPicker {
    pub fn new() -> MapPicker {
        MapPicker {
            maps: vec![],
            selected: None,
            highlighted: 0,
            map_image: None
        }
    }
}

impl MapPicker {
    pub fn setup(&mut self, ctx: &mut Context, renderer: &mut Renderer) -> Result<(), GameError> {
        for i in 0..10 {
            self.maps.push(Rc::new(read_map_file(ctx, i)));
        }
        let canvas = Canvas::with_window_size(ctx)?;
        graphics::set_canvas(ctx, Some(&canvas));

        let test_mesh = renderer.make_square_mesh(ctx, 10., true, 0.)?;
        renderer.draw_mesh(ctx, test_mesh.as_ref(), point(0.,0.));
        renderer.draw_mesh(ctx, test_mesh.as_ref(), point(SCREEN_WIDTH - 10.,SCREEN_HEIGHT - 10.));

        self.draw_maps(ctx, renderer)?;

        self.map_image = Some(canvas.into_inner());
        graphics::set_canvas(ctx, None);

        Ok(())
    }

    fn draw_maps(&mut self, ctx: &mut Context, renderer: &mut Renderer) -> Result<(), GameError> {
        let cell_size = 10.;
        let grid_offset = (60., 400.);
        let grid_spacing = (50., 100.);
        let grid_size = (cell_size * GRID_HORZ_COUNT as f32, cell_size * GRID_VERT_COUNT as f32);
        let grid_mesh = renderer.make_grid_mesh(ctx, cell_size, GRID_HORZ_COUNT, GRID_VERT_COUNT, 150)?;
        let square_mesh = renderer.make_square_mesh(ctx, cell_size, true, 2.)?;
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
        Ok(())
    }
}

impl Scene for MapPicker {
    fn update(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        Ok(())
    }

    fn render(&mut self, ctx: &mut Context, renderer: &mut Renderer) -> Result<(), GameError> {
        let cell_size = 10.;
        let grid_offset = (60., 400.);
        let grid_spacing = (50., 100.);
        let grid_size = (cell_size * GRID_HORZ_COUNT as f32, cell_size * GRID_VERT_COUNT as f32);
        let highlight_mesh = renderer.make_rect_mesh(ctx, grid_size.0 + 8., grid_size.1 + 8., false, 6.)?;

        let params = DrawParam::new()
            .dest(point(0.,1080.))
            .scale(Vector2 {x: 1., y: -1.}); //Images are drawn upside down due to ggez bug #304
        graphics::draw(ctx, self.map_image.as_ref().unwrap(), params)?;

        let x = self.highlighted % 5;
        let y = self.highlighted / 5;
        let grid_x = x as f32 * (grid_size.0 + grid_spacing.0) + grid_offset.0;
        let grid_y = y as f32 * (grid_size.1 + grid_spacing.1) + grid_offset.1;
        renderer.draw_coloured_mesh(ctx, highlight_mesh.as_ref(), point(grid_x - 2., grid_y - 2.), (0., 1., 1., 1.).into());

        renderer.draw_text(ctx, String::from("Choose a map"), point(770., 200.));

        Ok(())
    }

    fn on_button_press(&mut self, keycode: KeyCode) {
        match keycode {
            KeyCode::Up => {
                if self.highlighted > 4 {
                    self.highlighted -= 5;
                }
            },
            KeyCode::Down => {
                if self.highlighted < 5 {
                    self.highlighted += 5;
                }
            },
            KeyCode::Left => {
                if self.highlighted > 0 {
                    self.highlighted -= 1;
                }
            },
            KeyCode::Right => {
                if self.highlighted < 9 {
                    self.highlighted += 1;
                }
            },
            KeyCode::Return => {
                self.selected = Some(self.highlighted);
            },
            _ => {}
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