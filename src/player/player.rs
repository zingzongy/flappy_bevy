use crate::prelude::*;

const PARENT_MARGIN: f32 = 0.05; //use val 0 - 1

#[derive(Component)]
pub struct ParentEntity; 

#[derive(Component)]
pub struct Bird {
    pub x: f32,
    pub velocity: f32,
}

pub fn player_init(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    windows: Query<&Window>,
) {
    let bird_texture: Handle<Image> = asset_server.load("sprites/bird.png");
    let window = windows.single();
    let bird_x = (window.width() * PARENT_MARGIN) - (window.width() / 2.);
    let bird_y = (window.height() / 2.) - (window.height() * PARENT_MARGIN);

    commands.spawn((
        Sprite {
            image: bird_texture, 
            anchor: Anchor::Center,
            ..Default::default()
        },
        Transform::from_xyz(bird_x, bird_y, 1.),
        Bird {
            x: 0.,
            velocity: 0.
        }, 
    ));

    /*commands.spawn((
        Sprite {
            color: Color::srgb(0.3, 0.3, 0.5),
            custom_size: Some(Vec2::new(window.width(), window.height(),)),
            ..Default::default()
        },
        Transform::from_xyz(0., 0., 0.),
        BackgroundColor(Color::srgb(0.1, 0.1, 1.)),
        ParentEntity,
    ))
    .with_children(|p| {
        //spawning the bird
        p.spawn((
            Sprite {
                image: bird_texture, 
                anchor: Anchor::Center,
                ..Default::default()
            },
            Transform::from_xyz(bird_x, bird_y, 1.),
            Bird {
                x: 0.,
                velocity: 0.
            },
        ));
    });
    */
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
