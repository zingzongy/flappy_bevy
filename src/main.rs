mod ui;
mod settings;
mod player;
mod controls;
mod gamestate;
mod obstacles;

pub mod prelude {
    pub use bevy::{
        prelude::*, 
        window::{WindowResized, WindowResolution}, 
        sprite::*,
        input::keyboard::Key,
        };
    pub use crate::{
        ui::game_ui::*,
        settings::window_settings::{spawn_camera, handle_resize},
        player::player::*, 
        controls::inputs::*,
        gamestate::game_state::*,
        obstacles::obstacles::*,
    };
    pub const GAME_WIDTH: f32 = 800.;
    pub const GAME_HEIGHT: f32 = 500.;
    #[derive(Resource, Default)]
    pub struct Score {
        pub value: i32,
    }
}

use prelude::*;

fn main() {
    App::new()
            // Configure the window size using WindowDescriptor
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(800., 500.).with_scale_factor_override(1.),
                resizable: true,
                title: "Flappy Bevy".to_string(),
                ..Default::default()
           }),
           ..Default::default()
        }))
        .init_resource::<Score>()
        .init_state::<GameState>()
        .add_systems(Startup, (spawn_camera, enter_main_menu))
        //main menu state
        .add_systems(OnEnter(GameState::MainMenu), enter_main_menu)
        .add_systems(Update, menu_options.run_if(in_state(GameState::MainMenu)))
        .add_systems(OnExit(GameState::MainMenu), exit_main_menu)
        //playing state
        .add_systems(OnEnter(GameState::Playing), (player_init, spawn_obstacles, spawn_score_text, spawn_instruction_text, reset_score))
        .add_systems(Update, (apply_gravity, move_obstacles, update_score_text, game_over,).run_if(in_state(GameState::Playing)))
        .add_systems(OnExit(GameState::Playing), (exit_playing, despawn_obstacles, despawn_instruction))
        //main game over
        .add_systems(OnEnter(GameState::GameOver), enter_gameover_menu)
        .add_systems(Update, menu_options.run_if(in_state(GameState::GameOver)))
        .add_systems(OnExit(GameState::GameOver), exit_main_menu)
        //rest of the update systems that does not depend on state
        .add_systems(Update, (
            handle_resize, 
            keyboard_input_system,
            time_system,
            check_state,
        ))
        .run();
}