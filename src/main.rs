use bevy::{ prelude::{ Component, Query, Res, Transform, Vec3 }, time::Time };

#[derive(Component)]
struct Velocity(Vec3);

#[derive(Component)]
struct Radius(f32);

#[derive(Component)]
struct Mass(f32);
use bevy::prelude::*;

#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

#[derive(Resource)]
struct GreetTimer(Timer);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}
fn main() {
    App::new().add_plugins(DefaultPlugins).add_plugin(HelloPlugin).run();
}
pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_startup_system(add_people)
            .add_system(greet_people);
    }
}
fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    // update our timer with the time elapsed since the last update
    // if that caused the timer to finish, we say hello to everyone
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello {}!", name.0);
        }
    }
}
fn hello_world() {
    println!("hello world!");
}
fn move_system(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, &Velocity(v)) in query.iter_mut() {
        transform.translation += v * time.delta_seconds();
    }
}