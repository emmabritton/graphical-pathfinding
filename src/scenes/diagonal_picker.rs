use crate::{point, DPPoint};
use crate::data::Coord;
use crate::scenes::{Scene, SceneParams};
use ggez::{Context, GameError, GameResult};
use ggez::event::KeyCode;
use crate::data::maps::{Map, Variant};
use crate::graphics::renderer::*;
use crate::data::diagonal::Diagonal;
use crate::algos::Algo;
use crate::graphics::map_rendering::draw_map_with_costs_path;
use std::rc::Rc;
use std::collections::HashMap;

const CURSOR_ID: &'static str = "diagonal_highlighted";

struct DiagonalParams {
    map: Rc<Map>,
    algo: Algo,
    variant: usize,
}

pub struct DiagonalPicker {
    params: DiagonalParams,
    selected: Option<usize>,
    highlighted: usize,
    diagonal_maps: Vec<(Map, Vec<Coord>)>,
}

impl DiagonalPicker {
    pub fn new(map: Rc<Map>, algo: Algo, variant: usize, cursor_mem: &HashMap<&str, usize>) -> DiagonalPicker {
        DiagonalPicker {
            params: DiagonalParams { map, algo, variant },
            selected: None,
            highlighted: *cursor_mem.get(CURSOR_ID).unwrap_or(&0),
            diagonal_maps: vec![
                (Map {
                    variants: vec![Variant { start: Coord::new(0, 0), ends: vec![Coord::new(3, 3)] }],
                    cost: vec![vec![0, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 0]],
                }, vec![Coord::new(0, 0), Coord::new(1, 1), Coord::new(2, 2), Coord::new(3, 3)]),
                (Map {
                    variants: vec![Variant { start: Coord::new(0, 0), ends: vec![Coord::new(3, 3)] }],
                    cost: vec![vec![0, 0, 0, 9], vec![0, 0, 9, 0], vec![0, 9, 0, 0], vec![0, 0, 0, 0]],
                }, vec![Coord::new(0, 0), Coord::new(1, 0), Coord::new(2, 0), Coord::new(3, 1), Coord::new(3, 2), Coord::new(3, 3)]),
                (Map {
                    variants: vec![Variant { start: Coord::new(0, 0), ends: vec![Coord::new(3, 3)] }],
                    cost: vec![vec![0, 0, 0, 9], vec![0, 0, 9, 0], vec![0, 9, 0, 0], vec![9, 0, 0, 0]],
                }, vec![Coord::new(0, 0), Coord::new(1, 1), Coord::new(2, 2), Coord::new(3, 3)])
            ],
        }
    }
}

fn draw_allowed_markers(ctx: &mut Context, renderer: &mut Renderer, map_allowed: [bool; 3], offset: DPPoint, spacing: f32, size: f32) -> GameResult<()> {
    let cross_mesh = renderer.make_cross_mesh(ctx, size)?;
    let tick_mesh = renderer.make_tick_mesh(ctx, size)?;

    for i in 0..map_allowed.len() {
        if map_allowed[i] {
            renderer.draw_mesh(ctx, tick_mesh.as_ref(), point(offset.x + (spacing * i as f32), offset.y));
        } else {
            renderer.draw_mesh(ctx, cross_mesh.as_ref(), point(offset.x + (spacing * i as f32), offset.y));
        }
    }

    Ok(())
}

impl Scene for DiagonalPicker {
    fn update(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
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

        renderer.draw_white_text(ctx, "Choose how to handle diagonals", title_offset, 48., false);

        for i in 0..Diagonal::len() {
            renderer.draw_white_text(ctx, Diagonal::from_index(i).name(), point(text_offset.x, text_offset.y + (text_spacing * i as f32)), 48., false);
        }

        renderer.draw_mesh(ctx, list_mesh.as_ref(), point(text_offset.x - 50., text_offset.y + 8. + (self.highlighted as f32 * text_spacing)));

        for i in 0..self.diagonal_maps.len() {
            draw_map_with_costs_path(ctx, renderer, (grid_offset.x + (grid_spacing * i as f32 + grid_width * i as f32), grid_offset.y), 40., &self.diagonal_maps[i].0, &self.diagonal_maps[i].1, &vec![],0)?;
        }

        match Diagonal::from_index(self.highlighted) {
            Diagonal::Never => {
                draw_allowed_markers(ctx, renderer, [false, false, false], marker_offset, marker_spacing, size)?;
            }
            Diagonal::NoWalls => {
                draw_allowed_markers(ctx, renderer, [true, false, false], marker_offset, marker_spacing, size)?;
            }
            Diagonal::OneWall => {
                draw_allowed_markers(ctx, renderer, [true, true, false], marker_offset, marker_spacing, size)?;
            }
            Diagonal::Always => {
                draw_allowed_markers(ctx, renderer, [true, true, true], marker_offset, marker_spacing, size)?;
            }
        }

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
                if self.highlighted < Diagonal::len() - 1 {
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
        let diagonal = Diagonal::from_index(self.selected.expect("Nothing selected"));
        SceneParams::HeuristicSelection {
            map: self.params.map.clone(),
            algo: self.params.algo,
            variant: self.params.variant,
            diagonal,
        }
    }
}