/*
https://teletype.in/@rastr_0/solar_system
*/
use std::{ f64::consts::PI, str::FromStr };
use bevy::{ prelude::*, window::PresentMode, time::{ FixedTimestep }, log::{ debug, LogPlugin } };

const TIMESTEP_1_PER_SECOND: f64 = 60.0 / 60.0;
const TIMESTEP_2_PER_SECOND: f64 = 3.0 / 60.0;
const WIDTH: f32 = 800f32;
const HEIGHT: f32 = 500f32;

const DELTA_TIME: f32 = 100_000.0;
const GRAVITY_CONSTANT: f32 = 6.67e-11;
const AU: f32 = 1.5e11;
const SCALE: f32 = 20.0;
//const AU: f32 = 1.5e1;
#[derive(Component, Debug, Default)]
pub struct Celest_Body {
    name: String,
    mass: Mass,
    pos: Pos,
    velocity: Velocity,
    radius: f32,
}
#[derive(Component, Debug, Default)]
pub struct Pos(pub Vec3);
#[derive(Component, Debug, Default)]
pub struct Velocity(pub Vec3);

#[derive(Component, Debug, Default)]
pub struct Mass(pub f32);

fn main() {
    App::new()
        //.add_plugins(MinimalPlugins)
        //.add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_plugins(
            DefaultPlugins.set(LogPlugin {
                level: bevy::log::Level::DEBUG,
                filter: "debug,wgpu_core=error,wgpu_hal=error,sun_earth=debug".into(),
            })
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIMESTEP_2_PER_SECOND))
                .with_system(interact)
        )
        .run();
}
fn interact(mut query: Query<(&mut Celest_Body, &mut Transform)>) {
    let mut iter = query.iter_combinations_mut();
    while
        let Some(
            [
                (mut c_o1, mut transfo1),
                (mut c_o2, mut transfo2),
            ],
        ) = iter.fetch_next()
    {
        /*
        F = GMm/r2
        F = ma
        => a = GM/r2
        dv = a * dt
        dd = (v + dv) dt
         */
        debug!("interacting {} and {}", c_o1.name, c_o2.name);
        let distance = c_o2.pos.0 - c_o1.pos.0;
        //debug!("distance : {}", distance);
        let distance_cube = c_o2.pos.0.distance_squared(c_o1.pos.0).powf(1.5);
        //println!("distance3 : {}", distance3);

        let force = (GRAVITY_CONSTANT * distance) / distance_cube;
        //println!("force : {}", force);
        //println!("{:#?}", c_o2);
        // update quantities how is this calculated?  F = ma -> a = F/m and position = v * dt
        let acceleration1 = force * c_o2.mass.0;
        debug!("acceleration1 : {}", acceleration1);
        //println!("acceleration1: {}", acceleration1);
        let mut velo1 = acceleration1 * DELTA_TIME + c_o1.velocity.0;
        c_o1.velocity.0 = velo1;
        c_o1.pos.0 += velo1 * DELTA_TIME;

        let acceleration2 = -force * c_o1.mass.0;
        debug!("acceleration2 : {}", acceleration2);
        //println!("acceleration2: {}", acceleration2);
        let mut velo2 = acceleration2 * DELTA_TIME + c_o2.velocity.0;
        //println!("velocity 2 sq: {}", velo2.length_squared());
        c_o2.velocity.0 = velo2;
        c_o2.pos.0 += velo2 * DELTA_TIME;

        debug!("{} pos : {} ", c_o2.name, c_o2.pos.0 / AU);

        transfo2.translation = (c_o2.pos.0 / AU) * SCALE;
        transfo1.translation = (c_o1.pos.0 / AU) * SCALE;
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut sun = Celest_Body {
        //mass: Mass(2.0e6),
        mass: Mass(2.0e30),
        pos: Pos(Vec3::new(0.0, 0.0, 0.0)),
        radius: 1000.0,
        velocity: Velocity(Vec3::ZERO),
        name: String::from_str("Sun").unwrap(),
    };
    let mut earth = Celest_Body {
        //mass: Mass(5.972),
        mass: Mass(5.972e24),
        pos: Pos(Vec3::new(1.0167 * AU, 0.0, 0.0)),
        radius: 10.0,
        velocity: Velocity(Vec3::new(0.0, 29290.0, 0.0)),
        name: String::from_str("Earth").unwrap(),
    };
    let mut march = Celest_Body {
        //mass: Mass(5.972),
        mass: Mass(0.64169e24),
        pos: Pos(Vec3::new(1.666 * AU, 0.0, 0.0)),
        radius: 10.0,
        velocity: Velocity(Vec3::new(0.0, 21970.0, 0.0)),
        name: String::from_str("March").unwrap(),
    };
    commands.spawn((
        sun,
        SpriteBundle {
            texture: asset_server.load("branding/icon.png"),
            transform: Transform::from_xyz(0f32, 0f32, 0f32).with_scale(
                Vec3::new(0.15, 0.15, 0.25)
            ),
            ..default()
        },
    ));
    commands.spawn((
        earth,
        SpriteBundle {
            texture: asset_server.load("branding/icon.png"),
            transform: Transform::from_xyz(100f32, 0f32, 0f32).with_scale(Vec3::new(0.1, 0.1, 0.0)),
            ..default()
        },
    ));

    commands.spawn((
        march,
        SpriteBundle {
            texture: asset_server.load("branding/icon.png"),
            transform: Transform::from_xyz(100f32, 0f32, 0f32).with_scale(Vec3::new(0.1, 0.1, 0.0)),
            ..default()
        },
    ));

    commands.spawn(Camera2dBundle::default());
}