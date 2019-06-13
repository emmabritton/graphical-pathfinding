extern crate ggez;

mod algos;
mod data;
mod graphics;
mod scenes;
mod std_ext;

use ggez::{Context, ContextBuilder, GameResult, timer};
use ggez::graphics as ggez_g;
use ggez::event::{self, EventHandler, KeyMods, KeyCode};
use ggez::conf::{WindowMode, WindowSetup};
use ggez::mint::Point2;
use ggez::graphics::{Text, Color};
use ggez::graphics::TextFragment;
use ggez::graphics::Scale;
use ggez::quit;
use std::rc::Rc;
use crate::std_ext::max;
use std::env;
use std::path;
use crate::graphics::renderer::Renderer;
use crate::scenes::diagonal_picker::DiagonalPicker;
use crate::scenes::map_picker::MapPicker;
use crate::scenes::algo_picker::AlgoPicker;
use crate::scenes::executor::Executor;
use crate::scenes::heuristic_picker::HeuristicPicker;
use crate::scenes::{Scene, SceneParams};
use std::cell::RefCell;
use std::collections::HashMap;

pub const SCREEN_WIDTH: f32 = 1920.;
pub const SCREEN_HEIGHT: f32 = 1080.;
pub const GRID_VERT_COUNT: usize = 17;
pub const GRID_HORZ_COUNT: usize = 32;

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

    let mut my_game = GraphicalPath::new();

    let mut picker = MapPicker::new(&my_game.cursor_mem);
    if picker.setup(ctx, &mut my_game.renderer.borrow_mut()).is_err() {
        panic!("Failed to setup map picked");
    }
    my_game.active_scene = Some(Box::new(RefCell::new(picker)));

    match event::run(ctx, event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly"),
        Err(e) => println!("Error occured: {}", e)
    }
}

struct GraphicalPath {
    active_scene: Option<Box<RefCell<Scene>>>,
    renderer: Rc<RefCell<Renderer>>,
    cursor_mem: HashMap<&'static str, usize>
}

impl GraphicalPath {
    fn new() -> GraphicalPath {
        return GraphicalPath {
            active_scene: None,
            renderer: Rc::new(RefCell::new(Renderer::new())),
            cursor_mem: HashMap::new()
        };
    }
}

impl EventHandler for GraphicalPath {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if let Some(scene) = &mut self.active_scene {
            scene.borrow_mut().update(ctx)?;

            if scene.borrow_mut().is_complete() {
                let params = scene.borrow_mut().get_next_stage_params(&mut self.cursor_mem);
                match params {
                    SceneParams::DiagonalSelection { map, algo, variant } => {
                        self.active_scene = Some(Box::new(RefCell::new(DiagonalPicker::new(map, algo, variant, &self.cursor_mem))));
                    }
                    SceneParams::AlgoSelection { map, variant } => {
                        let picker = AlgoPicker::new(map.clone(), variant, &self.cursor_mem);
                        self.active_scene = Some(Box::new(RefCell::new(picker)));
                    }
                    SceneParams::HeuristicSelection { map, algo, diagonal, variant } => {
                        let picker = HeuristicPicker::new(map, algo, diagonal, variant, &self.cursor_mem);
                        self.active_scene = Some(Box::new(RefCell::new(picker)));
                    }
                    SceneParams::AlgoRunner { map, algo, algo_name, diagonal, heuristic, variant } => {
                        let executor = Executor::new(map.clone(), algo, algo_name, diagonal.name(), heuristic.name(), variant, &self.cursor_mem);
                        self.active_scene = Some(Box::new(RefCell::new(executor)));
                    }
                    SceneParams::EndOfProgram => {
                        quit(ctx);
                    }
                }
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        ggez_g::clear(ctx, [0., 0., 0., 1.].into());

        if let Some(scene) = &mut self.active_scene {
            scene.borrow_mut().render(ctx, &mut self.renderer.borrow_mut())?;
        }

        self.draw_fps(ctx);

        ggez_g::present(ctx)?;
        timer::yield_now();
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods, _repeat: bool) {
        match keycode {
            KeyCode::Escape | KeyCode::Q => quit(ctx),
            KeyCode::R => {
                let mut picker = MapPicker::new(&self.cursor_mem);
                if picker.setup(ctx, &mut self.renderer.borrow_mut()).is_err() {
                    panic!("Failed to setup map picked");
                }
                self.active_scene = Some(Box::new(RefCell::new(picker)));
            }
            _ => {
                if let Some(scene) = &mut self.active_scene {
                    scene.borrow_mut().on_button_down(keycode);
                }
            }
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods) {
        match keycode {
            _ => {
                if let Some(scene) = &mut self.active_scene {
                    scene.borrow_mut().on_button_up(keycode);
                }
            }
        }
    }
}

impl GraphicalPath {
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