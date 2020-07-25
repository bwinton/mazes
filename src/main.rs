mod backtrack;
mod eller;
mod kruskal;
mod parallel;
mod util;

#[macro_use]
extern crate enumset;
#[macro_use]
extern crate lazy_static;

#[cfg(cargo_web)]
mod web_util;

#[cfg(not(cargo_web))]
mod desktop_util;

#[cfg(not(cargo_web))]
#[macro_use]
extern crate clap;

#[cfg(cargo_web)]
extern crate stdweb;

#[cfg(not(cargo_web))]
use desktop_util::get_args;

#[cfg(cargo_web)]
use web_util::get_args;

use crate::util::{Algorithm, CELL_WIDTH, COLUMNS, LINE_WIDTH, ROWS};

// use std::thread::sleep;
// use std::time::Duration;

use quicksilver::{
    geom::Vector,
    graphics::{Color, Graphics},
    input::Key,
    log, run, Input, Result, Settings, Timer, Window,
};

struct MyGame {
    // Your state here...
    algorithm: Box<dyn Algorithm>,
    update_timer: Timer,
    draw_timer: Timer,
}

impl MyGame {
    pub fn new(algorithm: Box<dyn Algorithm>) -> MyGame {
        // Load/create resources such as images here.
        MyGame {
            algorithm,
            update_timer: Timer::time_per_second(20.0),
            draw_timer: Timer::time_per_second(60.0),
        }
    }

    fn update(&mut self, input: &Input, window: &Window, gfx: &mut Graphics) -> bool {
        if input.key_down(Key::Q) && (input.key_down(Key::LWin) || input.key_down(Key::RWin)) {
            return true;
        }

        while self.update_timer.tick() {
            self.algorithm.update();
        }

        if self.draw_timer.exhaust().is_some() && self.draw(window, gfx).is_err() {
            // Got an error, let's quit the app, and hope they logged.
            return true;
        }
        false
    }

    fn draw(&mut self, window: &Window, gfx: &mut Graphics) -> Result<()> {
        // Clear the screen to a blank, white color
        gfx.clear(Color::WHITE);

        self.algorithm.draw(gfx)?;

        // Send the data to be drawn
        gfx.present(&window)?;
        Ok(())
    }
}

fn main() {
    run(
        Settings {
            size: Vector::new(
                COLUMNS * CELL_WIDTH + LINE_WIDTH,
                ROWS * CELL_WIDTH + LINE_WIDTH,
            ),
            log_level: log::Level::Info,
            // icon_path: Some("static/maze.png"),
            ..Settings::default()
        },
        app,
    );
}

async fn app(window: Window, mut gfx: Graphics, mut input: Input) -> Result<()> {
    let arg = get_args()?;
    let algorithm: Box<dyn Algorithm> = match arg.as_str() {
        "backtrack" => Box::new(backtrack::Exports::new()),
        "parallel" => Box::new(parallel::Exports::new()),
        "eller" => Box::new(eller::Exports::new()),
        "kruskal" => Box::new(kruskal::Exports::new()),
        _ => {
            log::error!("Unimplemented algorithm: {:?}!", arg);
            panic!("Unimplemented algorithm.")
        }
    };
    log::info!("Algorithm: {:?}", algorithm.name());
    window.set_title(&format!("Some {} mazes…", algorithm.name()));

    let mut game = MyGame::new(algorithm);
    game.draw(&window, &mut gfx)?;

    'outer: loop {
        while input.next_event().await.is_some() {}
        if game.update(&input, &window, &mut gfx) {
            break 'outer;
        }
    }
    Ok(())
}
