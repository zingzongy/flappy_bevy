use crate::prelude::*;

//spawn camera
pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

pub fn handle_resize(
    mut resize_events: EventReader<WindowResized>,
    mut windows: Query<&mut Window>,
    mut camera_query: Query<&mut OrthographicProjection, With<Camera>>,
) {
    const ASPECT_RATIO: f32 = GAME_WIDTH / GAME_HEIGHT;

    for resize_event in resize_events.read() {
        let mut window = windows.single_mut();

        //calculate the new dimension based on the aspect ratio
        let new_width = resize_event.width;
        let new_height = new_width / ASPECT_RATIO;

        //update the window resolution
        window.resolution.set(new_width, new_height);

        //adjust the camera projection to scale the game content
        if let Ok(mut projection) = camera_query.get_single_mut() {
            projection.scale = GAME_HEIGHT / new_height;
        }
    }
}