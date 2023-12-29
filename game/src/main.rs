use bevy::prelude::*;
use bevy_engine::config::Config;

fn main() {
    App::new()
        .add_plugins(Config::new("My 2D Game!!!!".to_owned(), false, None))
        .run();
}
