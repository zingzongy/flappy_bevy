use crate::prelude::*;


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