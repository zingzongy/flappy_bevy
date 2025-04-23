use crate::prelude::*;

pub fn keyboard_input_system(
    keys: Res<ButtonInput<KeyCode>>, 
    mut player: Query<&mut Bird>, 
    mut game_exit: EventWriter<AppExit>,
) {
    if let Ok(mut bird) = player.get_single_mut() {
        if keys.just_pressed(KeyCode::Space) {
            println!("The Space key was just pressed");
            bird.velocity += 1.3;
        } 
    }

    //exit when pressed
    if keys.just_pressed(KeyCode::Escape) {
        println!("Exiting the app...");
        game_exit.send(AppExit::Success);
    }
}

pub fn menu_options(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keys.just_pressed(KeyCode::KeyP) {
        next_state.set(GameState::Playing);
    }
}