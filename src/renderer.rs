use std::collections::HashMap;
use ggez::graphics::{Mesh, MeshBuilder, DrawMode, Drawable, Rect, Color, TextFragment, Scale, Text};
use std::rc::Rc;
use ggez::{Context, GameResult, graphics};
use crate::{DPPoint, point};

pub struct Renderer {
    mesh_cache: HashMap<String, Rc<Mesh>>
}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {
            mesh_cache: HashMap::new()
        }
    }
}

impl Renderer {
    pub fn make_grid_mesh(&mut self, ctx: &mut Context, cell_size: f32, horz_count: usize, vert_count: usize, intensity: u8) -> GameResult<Rc<Mesh>> {
        let width = cell_size * horz_count as f32;
        let height = cell_size * vert_count as f32;
        let key = format!("grid_{}_{}_{}_{}", cell_size, horz_count, vert_count, intensity);
        if self.mesh_cache.contains_key(&key) {
            return Ok(self.mesh_cache[&key].clone());
        } else {
            let grid_line_width = 2.;
            let grid_line_color = (intensity, intensity, intensity, 255).into();
            let mut mesh_builder = MeshBuilder::new();
            for x in 0..horz_count {
                mesh_builder.line(&[point(x as f32 * cell_size, 0.), point(x as f32 * cell_size, height)], grid_line_width, grid_line_color)?;
            }
            for y in 0..vert_count {
                mesh_builder.line(&[point(0., y as f32 * cell_size), point(width, y as f32 * cell_size)], grid_line_width, grid_line_color)?;
            }
            mesh_builder.rectangle(DrawMode::stroke( grid_line_width), Rect::new(0., 0., width, height), grid_line_color);
            let mesh = Rc::new(mesh_builder.build(ctx)?);
            self.mesh_cache.insert(key, mesh.clone());
            return Ok(mesh);
        }
    }

    pub fn make_rect_mesh(&mut self, ctx: &mut Context, width: f32, height: f32, filled: bool, thickness: f32) -> GameResult<Rc<Mesh>> {
        let key = format!("rect_{}_{}_{}", width, height, filled);
        if self.mesh_cache.contains_key(&key) {
            return Ok(self.mesh_cache[&key].clone());
        } else {
            let mut mesh_builder = MeshBuilder::new();
            let mode;
            if filled {
                mode = DrawMode::fill();
            } else {
                mode = DrawMode::stroke(thickness);
            }
            mesh_builder.rectangle(mode, Rect::new(0., 0., width, height), (0.8, 0.8, 0.8, 1.).into());
            let mesh = Rc::new(mesh_builder.build(ctx)?);
            self.mesh_cache.insert(key, mesh.clone());
            return Ok(mesh);
        }
    }

    pub fn make_square_mesh(&mut self, ctx: &mut Context, cell_size: f32, filled: bool, thickness: f32) -> GameResult<Rc<Mesh>> {
        return self.make_rect_mesh(ctx, cell_size, cell_size, filled, thickness);
    }

    pub fn draw_mesh<D: Drawable>(&mut self, ctx: &mut Context, mesh: &D, xy: DPPoint) {
        graphics::draw(ctx, mesh, (xy, )).expect("couldn't draw");
    }

    pub fn draw_coloured_mesh<D: Drawable>(&mut self, ctx: &mut Context, mesh: &D, xy: DPPoint, new_colour: Color) {
        graphics::draw(ctx, mesh, (xy, new_colour)).expect("couldn't draw");
    }

    pub fn draw_text(&mut self, ctx: &mut Context, text: String, position: DPPoint) {
        let text = Text::new(TextFragment {
            text,
            scale: Some(Scale::uniform(48.)),
            ..TextFragment::default()
        });
        self.draw_mesh(ctx, &text, position);
    }
}