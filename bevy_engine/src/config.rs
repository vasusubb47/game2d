use bevy::prelude::*;
use bevy::window::{PresentMode, WindowMode, WindowResolution};

pub struct Config {
    app_name: String,
    resizable: bool,
    dimensions: Option<(u32, u32)>,
}

impl Config {
    /// Create a new
    pub fn new(app_name: String, resizable: bool, dimensions: Option<(u32, u32)>) -> Self {
        Config {
            app_name,
            resizable,
            dimensions,
        }
    }
}

impl Plugin for Config {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: self.app_name.to_owned(),
                present_mode: PresentMode::AutoVsync,
                resizable: self.resizable,
                mode: if self.dimensions.is_none() {
                    WindowMode::Fullscreen
                } else {
                    WindowMode::Windowed
                },
                resolution: if self.dimensions.is_some() {
                    let dim = self.dimensions.unwrap();
                    WindowResolution::new(dim.0 as f32, dim.1 as f32)
                } else {
                    WindowResolution::default()
                },
                ..default()
            }),
            ..default()
        }));
    }
}
