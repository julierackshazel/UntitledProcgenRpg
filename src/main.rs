use bevy::{math::vec3, prelude::*, transform::commands};
mod entity;
//mod entity::movement;
fn main() {
    let player = entity::player::Player {
        name: String::from("test"),
        //ideally should be a vector containing x, y, and z coords of the player
        position: Vec3::new(0.0, 0.0, 0.0),
        //should probably be a u32? i can't see any cases where player health would reach 32 bit int limit nor do i think it should ever be negative
        health: 0,
    };

    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
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
