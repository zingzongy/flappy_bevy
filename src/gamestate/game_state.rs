use crate::prelude::*;
#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    //#[default]
    Playing,
    GameOver,    
}

#[derive(Component)]
pub struct MainMenuParent;

pub fn enter_main_menu(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
) {
    let text_handle: Handle<Font> = asset_server.load("fonts/Minecraft.ttf");
    let font_size = 24.;
    commands.spawn((
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column, //arrange children vertically
            position_type: PositionType::Absolute,
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            left: Val::Percent(0.),
            top: Val::Percent(0.),
            align_items: AlignItems::Center, //center children horizontally
            justify_content: JustifyContent::Center, //center children vertically
            ..Default::default()
        },
        BackgroundColor(Color::srgb(0.15, 0.2, 0.2)),
        MainMenuParent, //marker, so that we can despawn this recursively and get rid of all the main menu items
    ))
    .with_children(|p| {
        //first item
        p.spawn((
            Node{
                margin: UiRect::bottom(Val::Percent(5.)),
                ..Default::default()
            },
            Text::new("Welcome to Flappy Bevy!"),
            TextFont {
                font: text_handle.clone(),
                font_size: 36.,
                ..Default::default()
            },
            TextLayout::new_with_justify(JustifyText::Center),
        ));
        //second item
        p.spawn((
            Text::new("Press <P> to Play"),
            TextFont {
                font: text_handle.clone(),
                font_size: font_size,
                ..Default::default()
            },
            TextLayout::new_with_justify(JustifyText::Center),
        ));
        //third item
        p.spawn((
            Text::new("Press <ESC> to Exit"),
            TextFont {
                font: text_handle.clone(),
                font_size: font_size,
                ..Default::default()
            },
            //TextLayout::new_with_justify(JustifyText::Center),
        ));
    }); 
}

pub fn enter_gameover_menu(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
) {
    let text_handle: Handle<Font> = asset_server.load("fonts/Minecraft.ttf");
    let font_size = 24.;
    commands.spawn((
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column, //arrange children vertically
            position_type: PositionType::Absolute,
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            left: Val::Percent(0.),
            top: Val::Percent(0.),
            align_items: AlignItems::Center, //center children horizontally
            justify_content: JustifyContent::Center, //center children vertically
            ..Default::default()
        },
        BackgroundColor(Color::srgb(0.15, 0.2, 0.2)),
        MainMenuParent, //marker, so that we can despawn this recursively and get rid of all the main menu items
    ))
    .with_children(|p| {
        //first item
        p.spawn((
            Node{
                margin: UiRect::bottom(Val::Percent(5.)),
                ..Default::default()
            },
            Text::new("Game Over!"),
            TextFont {
                font: text_handle.clone(),
                font_size: 36.,
                ..Default::default()
            },
            TextLayout::new_with_justify(JustifyText::Center),
        ));
        //second item
        p.spawn((
            Text::new("Press <P> to Play"),
            TextFont {
                font: text_handle.clone(),
                font_size: font_size,
                ..Default::default()
            },
            TextLayout::new_with_justify(JustifyText::Center),
        ));
        //third item
        p.spawn((
            Text::new("Press <ESC> to Exit"),
            TextFont {
                font: text_handle.clone(),
                font_size: font_size,
                ..Default::default()
            },
            //TextLayout::new_with_justify(JustifyText::Center),
        ));
    }); 
}

pub fn exit_main_menu(mut commands: Commands, main_menu: Query<Entity, With<MainMenuParent>>) {
    for item in main_menu.iter() {
        commands.entity(item).despawn_recursive();
    }
}

pub fn exit_playing(
    mut commands: Commands, 
    playing_items: Query<Entity, With<ParentEntity>>,
) {
    for parent in playing_items.iter() {
        commands.entity(parent).despawn_recursive();
    }
}

pub fn game_over(
    player: Query<&Transform, With<Bird>>,
    mut next_state: ResMut<NextState<GameState>>,
    obstacles: Query<(&Transform, &Obstacle)>
) {
    for bird in player.iter() {
        //falling to the ground
        if (bird.translation.y - ( - (GAME_HEIGHT / 2.))).abs() < 0.1 {
            println!("bird died");
            next_state.set(GameState::GameOver);
        }
        //collision detection with obstacles
        for (position, obstacle) in obstacles.iter() {
            let gap_top = obstacle.gap_y + obstacle.size / 2.;
            let gap_bot = obstacle.gap_y - obstacle.size / 2.;
            if (bird.translation.x >= (position.translation.x - 12.)) && (bird.translation.x <= (position.translation.x + 12.)) && (bird.translation.y > gap_top || bird.translation.y < gap_bot) {
                println!("Hit obstacle!");
                next_state.set(GameState::GameOver);
            }
        }
    }
}

pub fn check_state(state: Res<State<GameState>>, time: Res<Time>, mut timer: Local<f32>) {
    let delta_sec = time.delta_secs();
    *timer += delta_sec;
    match state.get() {
        GameState::MainMenu => {
            if *timer >= 2. {
                println!("Current State: MainMenu");
                *timer = 0.;
            }
        } ,
        GameState::GameOver => {
            if *timer >= 2. {
                println!("Current State: Game Over");
                *timer = 0.;
            }
        } ,
        GameState::Playing => {
            if *timer >= 2. {
                println!("Current State: Playing");
                *timer = 0.;
            }
        } ,
    }
}