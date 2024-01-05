use bevy::prelude::*;
mod entity;

fn main() {
    let player = entity::player::Player { name: todo!(), position: todo!(), health: todo!()  };

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
