mod aldous_broder;
mod binarytree;
mod blobby;
mod eller;
mod growingtree;
mod hex_blobby;
mod hex_parallel;
mod hex_util;
mod houston;
mod huntandkill;
mod kruskal;
mod origin_shift;
mod parallel;
mod penrose;
mod prim;
mod recdiv;
mod sidewinder;
mod wilson;

// extern crate maze_utils;
extern crate derive_more;
extern crate itertools;

mod util;

#[cfg(not(target_arch = "wasm32"))]
mod desktop_util;
#[cfg(target_arch = "wasm32")]
mod web_util;

#[macro_use]
extern crate enumset;
#[macro_use]
extern crate lazy_static;

use macroquad::{
    logging as log,
    miniquad::date::now,
    prelude::mouse_position,
    prelude::{
        get_frame_time, is_key_down, is_key_pressed, is_mouse_button_pressed, Conf, KeyCode,
        MouseButton,
    },
    rand,
    window::{clear_background, next_frame},
};

use util::{Algorithm, Args, RealArgs, WHITE};

use crate::util::State;

fn window_conf() -> Conf {
    Conf {
        window_title: "Mazes".to_owned(),
        window_width: 818,
        window_height: 618,
        high_dpi: true,
        sample_count: 1,
        ..Default::default()
    }
}

struct MyGame {
    algorithm: Box<dyn Algorithm>,
    args: RealArgs,
    update_timer: f32,
    paused: bool,
}

impl MyGame {
    pub fn new(algorithm: Box<dyn Algorithm>, args: RealArgs) -> MyGame {
        MyGame {
            algorithm,
            args,
            update_timer: 0.0,
            paused: false,
        }
    }

    fn handle_events(&mut self) -> bool {
        if is_key_down(KeyCode::Q) && is_key_down(KeyCode::LeftSuper)
            || is_key_down(KeyCode::RightSuper)
        {
            return true;
        }

        if is_key_pressed(KeyCode::R) {
            // R was pressed, so restart.
            self.paused = false;
            log::info!("Refreshing with {}", self.args.get_variant());
            self.algorithm.re_init(self.args.get_variant());
        }

        if is_key_pressed(KeyCode::Space) || is_mouse_button_pressed(MouseButton::Left) {
            // Space was pressed, so pause.
            self.paused = !self.paused;
        }
        false
    }

    fn update(&mut self) -> bool {
        if self.args.needs_reset() {
            // log::info!("Needs reset!");
            self.algorithm.re_init(self.args.get_variant());
        }
        self.update_timer += get_frame_time();
        let rv = self.handle_events();
        if self.update_timer > 0.08 {
            self.update_timer = 0.0;
            if !self.paused {
                match self.algorithm.get_state() {
                    State::Done => {
                        self.algorithm.move_to(mouse_position());
                    }
                    _ => {
                        self.algorithm.update();
                    }
                }
            }
        }
        rv
    }

    fn draw(&mut self) {
        // Clear the screen to a blank, white color
        clear_background(WHITE);
        self.algorithm.draw();
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    rand::srand(now() as u64);

    let args = RealArgs::new();
    let arg = args.get_algorithm();
    let variant = args.get_variant();
    log::info!("Args: {}, {}", args.get_algorithm(), args.get_variant());

    let message = format!("Expected an integer number of seeds. Got {}!", variant);
    let algorithm: Box<dyn Algorithm> = match arg.as_str() {
        "parallel" => Box::new(parallel::Exports::new(variant.parse().expect(&message))),
        "eller" => Box::new(eller::Exports::new()),
        "kruskal" => Box::new(kruskal::Exports::new()),
        "prim" => Box::new(prim::Exports::new()),
        "recdiv" => Box::new(recdiv::Exports::new()),
        "blobby" => Box::new(blobby::Exports::new()),
        "aldousbroder" => Box::new(aldous_broder::Exports::new(variant == "fast")),
        "wilson" => Box::new(wilson::Exports::new(variant == "slow")),
        "houston" => Box::new(houston::Exports::new()),
        "huntandkill" => Box::new(huntandkill::Exports::new()),
        "growingtree" => Box::new(growingtree::Exports::new(variant)),
        "bintree" => Box::new(binarytree::Exports::new(variant)),
        "sidewinder" => Box::new(sidewinder::Exports::new(variant == "hard")),
        "originshift" => Box::new(origin_shift::Exports::new(variant.parse().expect(&message))),
        "hexparallel" => Box::new(hex_parallel::Exports::new(variant.parse().expect(&message))),
        "hexblobby" => Box::new(hex_blobby::Exports::new()),
        "penrose" => Box::new(penrose::Exports::new(variant)),
        _ => {
            log::error!("Unimplemented algorithm: {:?}!", arg);
            panic!("Unimplemented algorithm.")
        }
    };
    log::info!(
        "Algorithm: {:?}, {:?}",
        algorithm.name(),
        algorithm.get_variant()
    );

    //     window.set_title(&format!("Some {} mazes…", algorithm.name()));
    let mut game = MyGame::new(algorithm, args);
    next_frame().await;

    loop {
        if game.update() {
            break;
        }
        game.draw();
        next_frame().await
    }
}
