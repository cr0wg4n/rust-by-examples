#![allow(unused)]

use bevy::{prelude::*};

const PLAYER_SPRITE: &str = "player_1.png";

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor{
            title: "Rust invaders".to_string(),
            width: 598.0,
            height: 676.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>, mut windows: ResMut<Windows>) {
    // Camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    
    // Position window
    let mut window = windows.get_primary_mut().unwrap();
    window.set_position(IVec2::new(3870,4830));
    
    // spawn a sprite
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(50.0,50.0)),
            ..Default::default()
        },
        ..Default::default()
    });
}