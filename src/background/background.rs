use std::collections::btree_map::Range;

use crate::prelude::*;
use bevy::render::view::window;
use rand::Rng;

#[derive(Component)]
pub struct Cloud; 

pub fn init_clouds(
    mut commands: Commands,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window>,
) {
    let mut rng = rand::rng();
    let cloud_texture: Handle<Image> = asset_server.load("sprites/cloud_1.png");
    let window = window_query.single();

    for _ in 0..rng.random_range(2.0..5.0) as i32 {
        let cloud_margin = rng.random_range(0.05..1.);
        let cloud_x = window.width() * cloud_margin - (window.width() / 2.);
        let cloud_y = ((window.height() / 2.) - window.height() * cloud_margin).clamp(0., window.height() / 2.);
        let cloud_scale = rng.random_range(2.0..4.0);

        commands.spawn((
            Sprite {
                image: cloud_texture.clone(), 
                anchor: Anchor::Center,
                ..Default::default()
            },
            Transform {
                translation: Vec3 { x: cloud_x, y: cloud_y, z: -1. },
                scale: Vec3 { x: cloud_scale, y: cloud_scale, z: 1. },
                ..Default::default()
            },
            Cloud,
        ));
    }
}

pub fn spawn_cloud(
    mut commands: Commands,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window>,
    cloud_query: Query<(Entity, &Transform), With<Cloud>>,
    mut timer: Local<f32>,
) {
    let mut rng = rand::rng();
    let cloud_texture: Handle<Image> = asset_server.load("sprites/cloud_1.png");
    let window = window_query.single();
    for (cloud, pos) in cloud_query.iter() {
        if pos.translation.x < - window.width() / 2. - (window.width() / 2. * 0.25) {
            commands.entity(cloud).despawn();
        }
    }
    let cloud_margin = rng.random_range(0.05..1.);
    //let cloud_x = window.width() * cloud_margin - (window.width() / 2.);
    let cloud_y = ((window.height() / 2.) - window.height() * cloud_margin).clamp(0., window.height() / 2.);
    let cloud_scale = rng.random_range(2.0..4.0);

    //defining range for random delay
    let min_delay: f32 = 1.0;
    let max_delay: f32 = 3.0;
    //accumulate elapsed time
    *timer += time.delta_secs();
    //check if timer has exceeded the current delay
    if *timer > 0.0 {
        //generate a random delay between the ranges
        let delay = rng.random_range(min_delay..max_delay);
        //spawning a cloud 
        commands.spawn((
            Sprite {
                image: cloud_texture.clone(), 
                anchor: Anchor::Center,
                ..Default::default()
            },
            Transform {
                translation: Vec3 { x: window.width() / 2. + (window.width() / 2. * 0.25) , y: cloud_y, z: -1. },
                scale: Vec3 { x: cloud_scale, y: cloud_scale, z: 1. },
                ..Default::default()
            },
            Cloud,
        ));
        //reset timer with new delay
        *timer = -delay; //starting countup from new delay
    }
}


pub fn despawn_clouds(mut commands: Commands, cloud_query: Query<Entity, With<Cloud>>) {
    for cloud in cloud_query.iter() {
        commands.entity(cloud).despawn();
    }
}

pub fn move_clouds(mut cloud_query: Query<&mut Transform, With<Cloud>>, time: Res<Time>) {
    for mut cloud in cloud_query.iter_mut() {
        let mut rng = rand::rng();
        let move_speed_factor = rng.random_range(1.0..2.0);
        cloud.translation.x -= 40. * move_speed_factor * time.delta_secs();
    }
}

#[derive(Component)]
pub struct BackGroundSprite;

pub fn setup_background(
    mut commands: Commands,
    window_query: Query<&Window>
) {
    let window = window_query.get_single().unwrap();
    commands.spawn((
        Sprite {
            color: Color::srgb(0.53, 0.8, 0.92),
            custom_size: Some(Vec2::new(window.width(), window.height())),
            ..Default::default()
        },
        Transform::from_xyz(0., 0., -2.),
        BackGroundSprite,
    ));
}

pub fn despawn_background(mut commands: Commands, bg_query: Query<Entity, With<BackGroundSprite>>) {
    let bg = bg_query.get_single().unwrap();
    commands.entity(bg).despawn();
}