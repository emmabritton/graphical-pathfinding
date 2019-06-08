use crate::{Scene, point, GRID_HORZ_COUNT, GRID_VERT_COUNT, SCREEN_WIDTH, SCREEN_HEIGHT, SceneParams};
use ggez::{Context, GameError};
use ggez::event::KeyCode;
use crate::maps::{Map, read_map_file};
use crate::renderer::*;
use std::rc::Rc;
use crate::map_rendering::draw_map_with_costs;
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

    fn get_cell_size_for_screen_width(width: f32) -> f32 {
        match width {
           res if res >= 3840. => return 16.,
           res if res >= 2560. => return 14.,
           res if res >= 1920. => return 10.,
           res if res >= 1366. => return 8.,
           res if res >= 1280. => return 7.,
           res if res >= 1024. => return 6.,
            _ => return 4.
        }
    }

    fn get_spacing_for_screen_width(width: f32) -> f32 {
        match width {
            res if res >= 4096. => return 234.,
            res if res >= 3840. => return 200.,
            res if res >= 1600. => return 50.,
            res if res >= 1400. => return 20.,
            res if res >= 1366. => return 12.,
            res if res >= 1280. => return 25.,
            res if res >= 1024. => return 10.,
            _ => return 25.
        }
    }

    fn draw_maps(&mut self, ctx: &mut Context, renderer: &mut Renderer) -> Result<(), GameError> {
        //Layout for diff screen widths
        //4096:  Grid width: 512  Spacing: 234  Offset: 300  Cell size: 16
        //3840:  Grid width: 512  Spacing: 200  Offset: 240  Cell size: 16
        //2560:  Grid width: 448  Spacing: 50   Offset: 60   Cell size: 14
        //1920:  Grid width: 320  Spacing: 50   Offset: 60   Cell size: 10
        //1600:  Grid width: 256  Spacing: 50   Offset: 60   Cell size:  8
        //1400:  Grid width: 256  Spacing: 20   Offset: 20   Cell size:  8
        //1366:  Grid width: 256  Spacing: 12   Offset: 18   Cell size:  8
        //1280:  Grid width: 224  Spacing: 25   Offset: 30   Cell size:  7
        //1024:  Grid width: 192  Spacing: 10   Offset: 12   Cell size:  6
        // 800:  Grid width: 128  Spacing: 25   Offset: 30   Cell size:  4
        let screen_size = renderer.get_screen_size(ctx);
        let cell_size = MapPicker::get_cell_size_for_screen_width(screen_size.0);
        let grid_spacing = (MapPicker::get_spacing_for_screen_width(screen_size.0), 100.);
        let remaining = screen_size.0 - ((cell_size * GRID_HORZ_COUNT as f32 * 5.) + (grid_spacing.0 * 4.));
        let grid_offset = (remaining / 2., 400.);
        let grid_size = (cell_size * GRID_HORZ_COUNT as f32, cell_size * GRID_VERT_COUNT as f32);
        for x in 0..5 {
            for y in 0..2 {
                let grid_x = x as f32 * (grid_size.0 + grid_spacing.0) + grid_offset.0;
                let grid_y = y as f32 * (grid_size.1 + grid_spacing.1) + grid_offset.1;
                let map_idx = x + y * 5;

                draw_map_with_costs(ctx, renderer, (grid_x, grid_y), cell_size, &self.maps[map_idx])?;
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
        let screen_size = renderer.get_screen_size(ctx);
        let cell_size = MapPicker::get_cell_size_for_screen_width(screen_size.0);
        let grid_spacing = (MapPicker::get_spacing_for_screen_width(screen_size.0), 100.);
        let remaining = screen_size.0 - ((cell_size * GRID_HORZ_COUNT as f32 * 5.) + (grid_spacing.0 * 4.));
        let grid_offset = (remaining / 2., 400.);
        let grid_size = (cell_size * GRID_HORZ_COUNT as f32, cell_size * GRID_VERT_COUNT as f32);
        let highlight_mesh = renderer.make_rect_mesh(ctx, grid_size.0 + 8., grid_size.1 + 8., false, 6.)?;

        let params = DrawParam::new()
            .dest(point(0.,screen_size.1))
            .scale(Vector2 {x: 1., y: -1.}); //Images are drawn upside down due to ggez bug #304
        graphics::draw(ctx, self.map_image.as_ref().unwrap(), params)?;

        let x = self.highlighted % 5;
        let y = self.highlighted / 5;
        let grid_x = x as f32 * (grid_size.0 + grid_spacing.0) + grid_offset.0;
        let grid_y = y as f32 * (grid_size.1 + grid_spacing.1) + grid_offset.1;
        renderer.draw_coloured_mesh(ctx, highlight_mesh.as_ref(), point(grid_x - 2., grid_y - 2.), (0., 1., 1., 1.).into());

        renderer.draw_white_text(ctx, String::from("Choose a map"), point(screen_size.0 / 2., 200.), 48., true);

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