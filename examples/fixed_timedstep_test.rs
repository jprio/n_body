//! Renders a 2D scene containing a single, moving sprite.

use std::{ f64::consts::PI, str::FromStr };
use bevy::{ prelude::*, window::PresentMode, time::{ FixedTimestep } };
// The timestep says how many times to run the SystemSet every second
// For TIMESTEP_1, it's once every second
// For TIMESTEP_2, it's twice every second

const TIMESTEP_1_PER_SECOND: f64 = 60.0 / 60.0;
const TIMESTEP_2_PER_SECOND: f64 = 30.0 / 60.0;

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_system_set(
            SystemSet::new()
                // This prints out "hello world" once every second
                .with_run_criteria(FixedTimestep::step(TIMESTEP_1_PER_SECOND))
                .with_system(slow_timestep)
        )
        .add_system_set(
            SystemSet::new()
                // This prints out "goodbye world" twice every second
                .with_run_criteria(FixedTimestep::step(TIMESTEP_2_PER_SECOND))
                .with_system(fast_timestep)
        )
        .run();
}

fn slow_timestep() {
    println!("hello world");
}

fn fast_timestep() {
    println!("goodbye world");
}