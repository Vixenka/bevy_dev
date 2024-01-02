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

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    commands.spawn((PrototypeMaterialMeshBundle {
        mesh: meshes.add(shape::Box::new(50.0, 2.0, 50.0).into()),
        material: "floor",
        ..default()
    },));

    commands.spawn((PrototypeMaterialMeshBundle {
        transform: Transform::from_xyz(-1.0, 1.0, -0.5),
        mesh: meshes.add(shape::Box::new(4.0, 2.0, 1.0).into()),
        material: "wall",
        ..default()
    },));

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 32000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_euler(
            EulerRot::ZYX,
            0.0,
            PI / 4.,
            -PI / 4.,
        )),
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 7.0,
            maximum_distance: 25.0,
            ..default()
        }
        .into(),
        ..default()
    });

    commands.spawn(Camera3dBundle {
        projection: PerspectiveProjection::default().into(),
        transform: Transform::from_xyz(1.5, 1.5, 1.5)
            .looking_at(Vec3::new(-2.0, -0.8, -2.0), Vec3::Y),
        ..default()
    });
}
