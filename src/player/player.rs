use crate::prelude::*;

const PARENT_MARGIN: f32 = 0.05; //use val 0 - 1

#[derive(Component)]
pub struct ParentEntity; 

#[derive(Component)]
pub struct Bird {
    pub x: f32,
    pub velocity: f32,
}

#[derive(Component)]
pub struct AnimationConfig {
    first_sprite_index: usize,
    last_sprite_index: usize,
    fps: u8,
    frame_timer: Timer,
}

impl AnimationConfig {
    fn new(first: usize, last: usize, fps: u8) -> Self {
        Self {
            first_sprite_index: first,
            last_sprite_index: last,
            fps,
            frame_timer: Self::timer_from_fps(fps),
        }
    }
    fn timer_from_fps(fps: u8) -> Timer {
        Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Once)
    }
}

//this system loops through all the sprites in the 'TextureAtlas', from 'first_sprite_index' to
//'last_sprite_index' (both defined in 'AnimationConfig').
pub fn execute_animations(time: Res<Time>, mut query: Query<(&mut AnimationConfig, &mut Sprite)>) {
    for (mut config, mut sprite) in &mut query {
        //we track how long the current sprite has been displayed for
        config.frame_timer.tick(time.delta());

        //if it has been displayed for the user-defined amount of time (fps)...
        if config.frame_timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                if atlas.index == config.last_sprite_index {
                    //..and it is the last frame, then we move back to the first frame and stop.
                    atlas.index = config.first_sprite_index;

                    config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
                } else {
                    //..and it is not the last frame, then we move to the next frame..
                    atlas.index += 1;
                    //..and reset the frame timer to start counting all over again
                    config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
                }
            }
        }
    }
}

pub fn player_init(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    windows: Query<&Window>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let bird_texture: Handle<Image> = asset_server.load("sprites/bird_spritesheet.png");
    let window = windows.single();
    let bird_x = (window.width() * PARENT_MARGIN) - (window.width() / 2.);
    let bird_y = (window.height() / 2.) - (window.height() * PARENT_MARGIN);

    //The spritesheet has 4 sprites arranged in a row, and they are all 32x32
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 4, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let bird_animation_config = AnimationConfig::new(1, 3, 5);

    commands.spawn((
        Sprite {
            image: bird_texture.clone(), 
            anchor: Anchor::Center,
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: bird_animation_config.first_sprite_index,
            }),
            ..Default::default()
        },
        Transform::from_xyz(bird_x, bird_y, 1.),
        Bird {
            x: 0.,
            velocity: 0.
        }, 
        bird_animation_config,
    ));
}

pub fn apply_gravity(mut player: Query<(&mut Transform, &mut Bird)>, time: Res<Time>) {
    for (mut transform, mut bird) in player.iter_mut() {
        if bird.velocity > -5.5 {
            bird.velocity -= 5.0 * time.delta_secs();
        } 
        transform.translation.y += bird.velocity;
        if transform.translation.y < - (GAME_HEIGHT / 2.) {
            transform.translation.y = - (GAME_HEIGHT / 2.);

        }
        if transform.translation.y > (GAME_HEIGHT / 2.) {
            transform.translation.y = GAME_HEIGHT / 2.;
            bird.velocity = 0.;
        }
    }
}
