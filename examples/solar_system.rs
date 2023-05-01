//! Renders a 2D scene containing a single, moving sprite.

use std::{ f64::consts::PI, str::FromStr };

use bevy::{ prelude::*, window::PresentMode, time::FixedTimestep };
const WIDTH: f32 = 800f32;
const HEIGHT: f32 = 500f32;

const DELTA_TIME: f32 = 1.0;
const GRAVITY_CONSTANT: f32 = 6.67e-11;
const AU: f32 = 1.5e11;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Msaa { samples: 4 })
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(DELTA_TIME as f64))
                .with_system(integrate)
                .with_system(interact)
        )
        .run();
}
#[derive(Component, Debug, Default)]
pub struct Pos(pub Vec3);
#[derive(Component, Debug, Default)]
pub struct Acceleration(pub Vec3);

#[derive(Component, Debug, Default)]
pub struct Mass(pub f32);

#[derive(Component, Debug, Default)]
pub struct Celest_Body {
    name: String,
    mass: Mass,
    pos: Pos,
    acceleration: Acceleration,
    radius: f32,
}
#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
struct FixedUpdateStage;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut sun = Celest_Body {
        mass: Mass(2.0e30),
        pos: Pos(Vec3::new(0.0, 0.0, 0.0)),
        radius: 1000.0,
        acceleration: Acceleration(Vec3::ZERO),
        name: String::from_str("Sun").unwrap(),
    };
    let mut earth = Celest_Body {
        mass: Mass(5.972e24),
        pos: Pos(Vec3::new(1.0167 * AU, 100.0, 0.0)),
        radius: 100.0,
        acceleration: Acceleration(Vec3::ZERO),
        name: String::from_str("Earth").unwrap(),
    };
    let mut moon = Celest_Body {
        mass: Mass(2.0e1),
        pos: Pos(Vec3::new(250.0, 250.0, 0.0)),
        acceleration: Acceleration(Vec3::ZERO),
        radius: 1.0,
        name: String::from_str("Moon").unwrap(),
    };
    let mut cam = Camera2dBundle::default();
    cam.projection.scale *= 0.01;
    //commands.spawn(cam);
    // Sun
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("branding/icon.png"),
            transform: Transform::from_xyz(sun.pos.0.x, sun.pos.0.y, 0f32).with_scale(
                Vec3::new(0.55, 0.55, 0.0)
            ),
            ..default()
        },
        sun,
    ));
    // Moon
    /*
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("branding/icon.png"),
            transform: Transform::from_xyz(moon.pos.0.x, moon.pos.0.y, 0f32).with_scale(
                Vec3::new(0.05, 0.05, 0.0)
            ),
            ..default()
        },
        moon,
    ));
    */
    //Earth
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("branding/icon.png"),
            transform: Transform::from_xyz(earth.pos.0.x, earth.pos.0.y, 0f32).with_scale(
                Vec3::new(0.15, 0.15, 0.0)
            ),
            ..default()
        },
        earth,
    ));
}
#[derive(Resource)]
struct MoveTimer(Timer);

fn interact(
    time: Res<Time>,
    //mut timer: ResMut<MoveTimer>,
    mut query: Query<(&mut Celest_Body, &mut Transform)>
) {
    let mut iter = query.iter_combinations_mut();
    while
        let Some(
            [
                (mut c_o1, mut transform1),
                (mut c_o2, mut transform2),
            ],
        ) = iter.fetch_next()
    {
        //println!("interact {:#?} et {:#?}", c_o1.name, c_o2.name);
        let delta = transform2.translation - transform1.translation;

        let distance_sq: f32 = delta.length_squared();
        println!("squared dist {}", distance_sq);
        let f = GRAVITY_CONSTANT / distance_sq;
        let force_unit_mass = delta * f;

        c_o1.acceleration.0 += force_unit_mass * c_o2.mass.0;
        c_o2.acceleration.0 -= force_unit_mass * c_o1.mass.0;

        //transform1.translation.y += speed.y;

        //println!("{:#?}", transform1);
        /*
            transform1.rotate_around(
                Vec3 { x: 0.0, y: 0.0, z: 0.0 },
                Quat::from_euler(
                    EulerRot::ZXY,
                    (20.0_f32).to_radians(),
                    (0.0_f32).to_radians(),
                    (0.0_f32).to_radians()
                )
            );
            transform2.rotate_around(
                Vec3 { x: 0.0, y: 0.0, z: 0.0 },
                Quat::from_euler(
                    EulerRot::ZXY,
                    (20.0_f32).to_radians(),
                    (0.0_f32).to_radians(),
                    (0.0_f32).to_radians()
                )
            );
            */
    }
}
fn integrate(
    time: Res<Time>,
    //mut timer: ResMut<MoveTimer>,
    mut query: Query<(&mut Celest_Body, &mut Transform)>
) {
    //let mut iter = query.iter_mut();
    for (mut c_b, mut transform) in &mut query {
        // TODO
        println!("integrate {:#?}", c_b.name);
        println!("transform {:#?}", transform.translation);
        // dv = F * dt / m
        // dx = dv * dt
        //let velocity = (c_b.acceleration.0 / c_b.mass.0) * DELTA_TIME;
        //transform.translation += velocity * DELTA_TIME;

        let new_pos = transform.translation * 2.0 - c_b.pos.0 + c_b.acceleration.0 * DELTA_TIME;
        c_b.acceleration.0 = Vec3::ZERO;
        c_b.pos.0 = transform.translation;
        transform.translation = new_pos;
    }
}