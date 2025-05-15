use std::f32::consts::PI;

use bevy::{pbr::CascadeShadowConfigBuilder, prelude::*};
use bevy_dev::prelude::*;

#[bevy_main]
fn main() {
    App::new()
        .add_plugins((DefaultPlugins, DevPlugins))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Circle::new(4.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));

    // cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(50.0, 2.0, 50.0))),
        PrototypeMaterial::new("floor"),
        Transform::default(),
    ));

    commands.spawn((
        Transform::from_xyz(-1.0, 1.0, -0.5),
        Mesh3d(meshes.add(Cuboid::new(4.0, 2.0, 1.0))),
        PrototypeMaterial::new("wall"),
    ));

    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::ZYX, 0.0, PI / 4., -PI / 4.)),
        CascadeShadowConfigBuilder {
            first_cascade_far_bound: 7.0,
            maximum_distance: 25.0,
            ..default()
        }
        .build(),
    ));

    // commands.spawn(Camera3dBundle {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(1.5, 1.5, 1.5).looking_at(Vec3::new(-2.0, -0.8, -2.0), Vec3::Y),
    ));
}
