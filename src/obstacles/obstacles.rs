use crate::{prelude::*, Score};
use bevy::color::palettes::css::WHITE;
use rand::Rng;

#[derive(Component)]
pub struct Obstacle {
    pub gap_y: f32,
    pub size: f32,
}

pub fn spawn_obstacles(
    mut commands: Commands,
    score: Res<Score>,
) {
    let mut rng = rand::rng();
    
    let x = GAME_WIDTH / 2.; //spawn obstacles at the right edge of the screen
    let gap_y = rng.random_range(-240.0..240.0); //randomly generate the gap's y position
    let size = (250.0 - score.value as f32).max(2.0);//ensure the gap size doesnt shrink below 2.0

    //boundaries of the gap
    let gap_top = gap_y + size / 2.;
    let gap_bot = gap_y - size / 2.;
    let top_obs_size = GAME_HEIGHT / 2. - gap_top;
    let bot_obs_size = GAME_HEIGHT / 2. + gap_bot;
    //spawn the top part of the obstacle
    commands.spawn((
        Obstacle {gap_y, size},
        Sprite {
            color: Color::Srgba(WHITE),
            custom_size: Some(Vec2::new(10.0, top_obs_size)), //width and height of the obstacle
            ..Default::default()
        },
        Transform::from_xyz(x, gap_top + top_obs_size / 2. , 1.0)
    ));
    //spawn the bottom part of the obstacle
    commands.spawn((
        Obstacle {gap_y, size},
        Sprite {
            color: Color::Srgba(WHITE),
            custom_size: Some(Vec2::new(10.0, bot_obs_size)), //width and height of the obstacle
            ..Default::default()
        },
        Transform::from_xyz(x, gap_bot - bot_obs_size / 2. , 1.0)
    ));
}

pub fn move_obstacles(
    mut commands : Commands,
    mut obstacles: Query<(Entity, &mut Transform), With<Obstacle>>,
    mut score: ResMut<Score>,
) {
    //check to see if obstacle exists, if it does, we check if it went out of bound, despawn if yes
    if !obstacles.is_empty() {
        for (obstacle, mut pos) in obstacles.iter_mut() {
            pos.translation.x -= 5.;
            if pos.translation.x < - (GAME_WIDTH / 2.) {
                commands.entity(obstacle).despawn();
            }
        }
    // if its empty, we spawn new obstacles, add 1 to score
    } else {
        score.value += 1;
        let mut rng = rand::rng();
    
        let x = GAME_WIDTH / 2.; //spawn obstacles at the right edge of the screen
        let gap_y = rng.random_range(-240.0..240.0); //randomly generate the gap's y position
        let size = (250.0 - (score.value as f32) * 10.).max(2.0);//ensure the gap size doesnt shrink below 2.0

        //boundaries of the gap
        let gap_top = gap_y + size / 2.;
        let gap_bot = gap_y - size / 2.;
        let top_obs_size = GAME_HEIGHT / 2. - gap_top;
        let bot_obs_size = GAME_HEIGHT / 2. + gap_bot;
        //spawn the top part of the obstacle
        commands.spawn((
            Obstacle {gap_y, size},
            Sprite {
                color: Color::Srgba(WHITE),
                custom_size: Some(Vec2::new(10.0, top_obs_size)), //width and height of the obstacle
                ..Default::default()
            },
            Transform::from_xyz(x, gap_top + top_obs_size / 2. , 1.0)
        ));
        //spawn the bottom part of the obstacle
        commands.spawn((
            Obstacle {gap_y, size},
            Sprite {
                color: Color::Srgba(WHITE),
                custom_size: Some(Vec2::new(10.0, bot_obs_size)), //width and height of the obstacle
                ..Default::default()
            },
            Transform::from_xyz(x, gap_bot - bot_obs_size / 2. , 1.0)
        ));
    }
}

pub fn despawn_obstacles(
    mut commands: Commands,
    obstacles: Query<Entity, With<Obstacle>>,
) {
    for obstacle in obstacles.iter() {
        commands.entity(obstacle).despawn();
    }
}
