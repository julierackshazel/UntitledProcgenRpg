use bevy::{prelude::*, transform::commands};
mod entities;
//mod entity::movement;
fn main() {
    let player = entities::player::Player {
        name: test,
        position: todo!(),
        health: todo!(),
    };

    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, keyboard_events)
        .run();
}

fn keyboard_events(key: Res<Input<KeyCode>>) {
    if key.pressed(KeyCode::Right) {
        info!("right arrow");
    } else if key.pressed(KeyCode::Left) {
        info!("left arrow");
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        texture: asset_server.load("programmer_art.png"),
        ..default()
    });
}
// add  stuff here that does more than spawn a sprite  :pray:
fn player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("testing_guy.png"),
        ..default()
    });
}
