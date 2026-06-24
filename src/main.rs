mod camera;
mod cursor;
mod player;
mod bullets;
mod game;
mod states;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::bullets::*;
use crate::camera::*;
use crate::cursor::*;
use crate::game::*;
use crate::player::*;
use crate::states::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: (700.0, 700.0).into(),
            title: "Ball Collector".to_string(),
            ..default()
        }),
        ..default()
    }));
    app.add_plugins((
        RapierPhysicsPlugin::<NoUserData>::default(),
        GameStatesPlugin,
        Camera3dPlugin,
        CursorPlugin,
        BulletsPlugin,
        GamePlugin,
        PlayerPlugin
    ));
    app.run();
}
