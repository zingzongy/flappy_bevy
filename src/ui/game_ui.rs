use crate::prelude::*;

#[derive(Component)]
pub struct ScoreTextContainer;

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct InstructionText;

#[derive(Component)]
pub struct ScoreValueText;

pub fn time_system(
    time: Res<Time>, 
    mut time_elapsed: Local<f32>, 
    mut total_time_elapsed: Local<f32>, 
    player: Query<&mut Bird>
) {
    //setting the time elapsed to keep track of time
    // Get the time elapsed since the last frame as a Duration
    let delta_time = time.delta();

    // Alternatively, get the time elapsed in seconds as a floating-point number
    let delta_seconds = time.delta_secs();
    *time_elapsed += delta_seconds;
    *total_time_elapsed += delta_seconds;

    if *time_elapsed > 3. {
        println!("Time elapsed since last frame: {:?}", delta_time);
        println!("Time elapsed in seconds: {}", delta_seconds);
        println!("Total time elapsed: {}s ", *total_time_elapsed);
        if let Ok(bird) = player.get_single() {
            println!("Current X pos: {}", bird.x);
        }

        *time_elapsed = 0.;
    }
}

pub fn spawn_score_text(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    score: Res<Score>,
) {
    let text_handle = asset_server.load("fonts/Minecraft.ttf");
    let init_score_value = score.value;
    commands.spawn((
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Row, //arrange children vertically
            position_type: PositionType::Absolute,
            width: Val::Percent(10.),
            height: Val::Percent(10.),
            left: Val::Percent(5.),
            top: Val::Percent(5.),
            align_items: AlignItems::Center, //center children horizontally
            justify_content: JustifyContent::Center, //center children vertically
            ..Default::default()
        },
        //BackgroundColor(Color::srgb(0.15, 0.2, 0.2)),
        ScoreTextContainer,
    ))
    .with_children(|p| {
        //score text
        p.spawn((
            Node{
                //margin: UiRect::bottom(Val::Percent(5.)),
                ..Default::default()
            },
            Text::new("Score: "),
            TextFont {
                font: text_handle.clone(),
                font_size: 18.,
                ..Default::default()
            },
            TextLayout::new_with_justify(JustifyText::Center),
            ScoreText,
        ));
        //score value text
        p.spawn((
            Text::new(init_score_value.to_string()),       
            TextFont {
                font: text_handle.clone(),
                font_size: 18.,
                ..Default::default()
            },
            ScoreValueText,
        ));
    });
}

pub fn spawn_instruction_text(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let text_handle = asset_server.load("fonts/Minecraft.ttf");
    commands.spawn((
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column, //arrange children vertically
            position_type: PositionType::Absolute,
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            left: Val::Percent(0.),
            top: Val::Percent(50.),
            align_items: AlignItems::Center, //center children horizontally
            justify_content: JustifyContent::Center, //center children vertically
            ..Default::default()
        },
        //BackgroundColor(Color::srgb(0.15, 0.2, 0.2)),
        InstructionText,
    ))
    .with_children(|p| {
        //first item
        p.spawn((
            Node{
                margin: UiRect::bottom(Val::Percent(5.)),
                ..Default::default()
            },
            Text::new("Press <SPACE> to Flap"),
            TextFont {
                font: text_handle.clone(),
                font_size: 18.,
                ..Default::default()
            },
            TextLayout::new_with_justify(JustifyText::Center),
        ));
    });
}

pub fn update_score_text(score: Res<Score>, mut query: Query<&mut Text, With<ScoreValueText>>) {
    if score.is_changed() {
        for mut text in query.iter_mut() {
            *text = Text::new(format!("{}", score.value));
        }
    }
}

pub fn despawn_instruction(
    mut commands: Commands,
    instruction_query: Query<Entity, With<InstructionText>>
) {
    let instruction = instruction_query.get_single().unwrap();
    commands.entity(instruction).despawn_recursive();
}