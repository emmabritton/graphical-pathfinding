pub mod algo_picker;
pub mod diagonal_picker;
pub mod executor;
pub mod heuristic_picker;
pub mod map_picker;

use crate::graphics::renderer::Renderer;
use ggez::{Context, GameResult};
use ggez::event::KeyCode;
use std::rc::Rc;
use std::cell::RefCell;
use crate::data::{maps::Map, diagonal::Diagonal, heuristic::Heuristic};
use crate::algos::{Algo, Algorithm};

pub trait Scene {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()>;
    fn render(&mut self, ctx: &mut Context, renderer: &mut Renderer) -> GameResult<()>;
    fn on_button_down(&mut self, keycode: KeyCode);
    fn on_button_up(&mut self, keycode: KeyCode);
    fn is_complete(&self) -> bool;
    fn get_next_stage_params(&self) -> SceneParams;
}


pub enum SceneParams {
    AlgoSelection { map: Rc<Map>, variant: usize },
    DiagonalSelection { map: Rc<Map>, algo: Algo, variant: usize },
    HeuristicSelection { map: Rc<Map>, algo: Algo, diagonal: Diagonal, variant: usize },
    AlgoRunner { map: Rc<Map>, algo: Rc<RefCell<Box<Algorithm>>>, algo_name: String, diagonal: Diagonal, heuristic: Heuristic, variant: usize },
    Empty,
}
