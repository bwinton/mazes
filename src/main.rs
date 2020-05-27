mod util;

mod backtrack;
mod parallel;
mod eller;

#[macro_use]
extern crate clap;
#[macro_use]
extern crate enumset;

use crate::util::{Algorithm, CELL_WIDTH, COLUMNS, LINE_WIDTH, ROWS};

use clap::Arg;

use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::{self, quit, EventHandler};
use ggez::input::keyboard::{KeyCode, KeyMods};
use ggez::timer::check_update_time;
use ggez::{graphics, Context, ContextBuilder, GameResult};

struct MyGame {
    // Your state here...
    algorithm: Box<dyn Algorithm>,
}

impl MyGame {
    pub fn new(_ctx: &mut Context, algorithm: Box<dyn Algorithm>) -> MyGame {
        // Load/create resources such as images here.
        MyGame { algorithm }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        while check_update_time(ctx, 6) {
            self.algorithm.update();
        }
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        keymods: KeyMods,
        _repeat: bool,
    ) {
        if keycode == KeyCode::Q && keymods.contains(KeyMods::LOGO) {
            quit(ctx);
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);
        self.algorithm.draw(ctx)?;
        graphics::present(ctx)
    }
}

fn main() {
    let matches = app_from_crate!("\n")
        .arg(
            Arg::with_name("algorithm")
                .short('a')
                .about("Which algorithm to run")
                .long_about("Specify an algorithm to run.")
                .takes_value(true)
                .possible_values(&[
                    "backtrack",
                    "parallel",
                    "eller",
                    "kruskal",
                    "prim",
                    "recdiv",
                    // "blobby",
                    "aldousbroder",
                    "wilson",
                    // "houston",
                    "huntandkill",
                    // "tree",
                    "growingbintree",
                    "bintree",
                    "sidewinder",
                ])
                .default_value("backtrack"),
        )
        .get_matches();

    let algorithm: Box<dyn Algorithm> = match matches.value_of("algorithm").unwrap() {
        "backtrack" => Box::new(backtrack::Exports::new()),
        "parallel" => Box::new(parallel::Exports::new()),
        "eller" => Box::new(eller::Exports::new()),
        _ => panic!("Unimplemented algorithm."),
    };
    println!("Algorithm: {:?}", algorithm.name());

    // Make a Context.
    let (mut ctx, mut event_loop) = ContextBuilder::new("mazes", "Blake Winton")
        .window_setup(WindowSetup::default().title("Some mazes…"))
        .window_mode(WindowMode::default().dimensions(
            COLUMNS * CELL_WIDTH + LINE_WIDTH,
            ROWS * CELL_WIDTH + LINE_WIDTH,
        ))
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let mut my_game = MyGame::new(&mut ctx, algorithm);

    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
}
