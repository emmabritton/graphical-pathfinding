extern crate ggez;

mod astar;
mod models;
mod std_ext;
mod maps;
mod algo_picker;
mod map_picker;
mod renderer;
mod executor;
mod ggez_ext;

use ggez::{Context, ContextBuilder, GameResult, graphics, timer};
use ggez::event::{self, EventHandler, KeyMods, KeyCode};
use ggez::conf::{WindowMode, WindowSetup};
use ggez::mint::Point2;
use ggez::graphics::{Text, Color};
use ggez::graphics::TextFragment;
use ggez::graphics::Scale;
use ggez::quit;
use std::rc::Rc;
use crate::Mode::*;
use crate::std_ext::max;
use std::env;
use std::path;
use crate::map_picker::MapPicker;
use std::cell::RefCell;
use crate::renderer::Renderer;
use crate::models::Coord;
use crate::executor::Executor;
use crate::astar::Astar;
use crate::maps::Map;
use crate::algo_picker::AlgoPicker;

pub const SCREEN_WIDTH: f32 = 1920.;
pub const SCREEN_HEIGHT: f32 = 1080.;
pub const GRID_WIDTH: f32 = 1920.;
pub const GRID_HEIGHT: f32 = 1020.;
pub const CELL_SIZE: f32 = 60.;
pub const GRID_VERT_COUNT: usize = 17;
pub const GRID_HORZ_COUNT: usize = 32;
pub const GRID_START: (f32, f32) = (0., CELL_SIZE);

pub enum AlgoStatus {
    InProgress((Vec<Coord>, Vec<Coord>)),
    Found(Vec<Coord>),
    NoPath,
}

pub trait Algorithm {
    fn tick(&mut self);
    fn get_data(&self) -> &AlgoStatus;
}

pub type DPPoint = Point2<f32>;

pub fn point(x: f32, y: f32) -> DPPoint {
    return DPPoint { x, y };
}

fn main() {
    let mut cb = ContextBuilder::new("graphical_pathing", "Ray Britton")
        .window_mode(WindowMode {
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            resizable: false,
            ..WindowMode::default()
        })
        .window_setup(WindowSetup {
            title: String::from("Graphic Pathfinding"),
            ..WindowSetup::default()
        });

    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
//        println!("Adding path {:?} from manifest", path);
        cb = cb.add_resource_path(path);
    } else {
        //path::PathBuf::from("./resources") //might be needed if released
        panic!("Failed to get resources");
    }

    let (ctx, event_loop) = &mut cb
        .build()
        .expect("Could not create ggez context!");

    let mut my_game = GPath::new();

    let mut picker = MapPicker::new();
    if picker.setup(ctx, &mut my_game.renderer.borrow_mut()).is_err() {
        panic!("Failed to setup map picked");
    }
    my_game.active_scene = Some(Box::new(RefCell::new(picker)));

    match event::run(ctx, event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly"),
        Err(e) => println!("Error occured: {}", e)
    }
}

#[derive(Debug)]
enum Mode {
    MapSelection,
    AlgoSelection,
    AlgoRunner,
}

pub enum SceneParams {
    AlgoSelection { map: Rc<Map> },
    AlgoRunner { map: Rc<Map>, algo: Rc<RefCell<Algorithm>>, algo_name: String },
    Empty,
}

struct GPath {
    mode: Mode,
    active_scene: Option<Box<RefCell<Scene>>>,
    //this seems questionable
    renderer: Rc<RefCell<Renderer>>,
}

trait Scene {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()>;
    fn render(&mut self, ctx: &mut Context, renderer: &mut Renderer) -> GameResult<()>;
    fn on_button_press(&mut self, keycode: KeyCode);
    fn is_complete(&self) -> bool;
    fn get_next_stage_params(&self) -> SceneParams;
}

impl GPath {
    fn new() -> GPath {
        return GPath {
            mode: MapSelection,
            active_scene: None,
            renderer: Rc::new(RefCell::new(Renderer::new())),
        };
    }
}

impl EventHandler for GPath {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if let Some(scene) = &mut self.active_scene {
            scene.borrow_mut().update(ctx)?;

            if scene.borrow_mut().is_complete() {
                println!("{:?} scene complete", self.mode);
                let params = scene.borrow_mut().get_next_stage_params();
                match params {
                    SceneParams::AlgoSelection { map } => {
                        let picker = AlgoPicker::new(map.clone());
                        self.active_scene = Some(Box::new(RefCell::new(picker)));
                        self.mode = AlgoSelection;
                        println!("Starting algo picker");
                    }
                    SceneParams::AlgoRunner { map, algo, algo_name } => {
                        let executor = Executor::new(map.clone(), algo.clone(), algo_name);
                        self.active_scene = Some(Box::new(RefCell::new(executor)));
                        self.mode = AlgoRunner;
                        println!("Starting algo runner");
                    }
                    _ => panic!("Invalid output from map picker")
                }
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, [0., 0., 0., 1.].into());

        if let Some(scene) = &mut self.active_scene {
            scene.borrow_mut().render(ctx, &mut self.renderer.borrow_mut())?;
        }

        self.draw_fps(ctx);

        graphics::present(ctx)?;
        timer::yield_now();
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, _keycode: KeyCode, _keymod: KeyMods, _repeat: bool) {}

    fn key_up_event(&mut self, ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods) {
        match keycode {
            KeyCode::Escape | KeyCode::Q => quit(ctx),
            KeyCode::R => {
                let mut picker = MapPicker::new();
                if picker.setup(ctx, &mut self.renderer.borrow_mut()).is_err() {
                    panic!("Failed to setup map picked");
                }
                self.active_scene = Some(Box::new(RefCell::new(picker)));
                self.mode = MapSelection;
            },
            _ => {
                if let Some(scene) = &mut self.active_scene {
                    scene.borrow_mut().on_button_press(keycode);
                }
            }
        }
    }
}

impl GPath {
    fn draw_fps(&mut self, ctx: &mut Context) {
        let text = Text::new(TextFragment {
            text: format!("{:.0}", timer::fps(ctx)),
            color: Some(Color::new(1., 0., 0., 0.5)),
            scale: Some(Scale::uniform(32.)),
            ..TextFragment::default()
        });
        self.renderer.borrow_mut().draw_mesh(ctx, &text, point(SCREEN_WIDTH - 100., 0.));
    }
}