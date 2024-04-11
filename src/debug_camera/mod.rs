use std::{f32::consts::PI, fmt::Debug, ops::RangeInclusive};

use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
    window::{Cursor, CursorGrabMode, PrimaryWindow},
};

#[cfg(feature = "ui")]
use crate::ui::popup::PopupEvent;
use crate::ui::popup::PopupPosition;

#[cfg(feature = "ui")]
mod ui;

const MOUSE_LOOK_X_LIMIT: f32 = PI / 2.0;
const RESET_SPEED_THRESHOLD_IN_SECONDS: f32 = 0.2;

#[derive(Default)]
pub struct DebugCameraPlugin {
    pub switcher: DebugCameraSwitcher,
}

impl Plugin for DebugCameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DebugCameraGlobalData>()
            .add_systems(Update, (initialization, focus_system, controller));

        let active_spawner = match self.switcher {
            DebugCameraSwitcher::Default => {
                #[cfg(not(debug_assertions))]
                bevy::log::warn!("Switcher from bevy_dev's `DebugCamera` is active in release builds. This allows the player to easily activate and manage debug cameras, set the `DebugCameraSpawner` value explicitly in the `DebugCameraPlugin`");
                true
            }
            DebugCameraSwitcher::Active => true,
            DebugCameraSwitcher::Disabled => false,
        };
        if active_spawner {
            app.add_systems(Update, switcher);
        }
    }
}

#[derive(Default)]
pub enum DebugCameraSwitcher {
    #[default]
    Default,
    Active,
    Disabled,
}

#[derive(Debug, Resource)]
pub struct DebugCameraGlobalData {
    pub default_value: DebugCamera,
    pub last_used_debug_cameras: Vec<Entity>,
    pub last_used_origin_camera: Option<DebugCameraLastUsedOriginCameraData>,
    pub(super) selected_camera: Option<usize>,
    next_id: u64,
}

impl Default for DebugCameraGlobalData {
    fn default() -> Self {
        Self {
            default_value: DebugCamera::default(),
            last_used_debug_cameras: Vec::new(),
            last_used_origin_camera: None,
            selected_camera: None,
            next_id: 1,
        }
    }
}

#[derive(Debug)]
pub struct DebugCameraLastUsedOriginCameraData {
    pub camera: Entity,
    pub cursor: Cursor,
}

#[derive(Component, Debug, Clone)]
#[non_exhaustive]
pub struct DebugCamera {
    pub speed_increase: f32,
    pub speed_multiplier: f32,
    pub speed_multiplier_range: RangeInclusive<f32>,
    pub sensitivity: f32,
    pub base_speed: f32,
    pub focus: bool,
}

impl Default for DebugCamera {
    fn default() -> Self {
        Self {
            speed_increase: 0.2,
            speed_multiplier: 1.0,
            speed_multiplier_range: 0.001..=10.0,
            sensitivity: 0.1,
            base_speed: 4.5,
            focus: true,
        }
    }
}

#[derive(Debug, Component)]
pub(super) struct DebugCameraData {
    id: u64,
    last_change_position_time: f32,
    current_speed: f32,
}

fn spawn_new_debug_camera(commands: &mut Commands) {
    commands.spawn(DebugCamera::default());
    bevy::log::info!("Spawned new 3D debug camera");
}

fn switcher(
    mut commands: Commands,
    mut cameras: Query<(Entity, &mut Camera)>,
    mut debug_cameras: Query<(Entity, &mut DebugCamera, &DebugCameraData)>,
    mut global: ResMut<DebugCameraGlobalData>,
    mut window: Query<&mut Window, With<PrimaryWindow>>,
    keys: Res<ButtonInput<KeyCode>>,
    #[cfg(feature = "ui")] mut popup_event: EventWriter<PopupEvent>,
) {
    if !keys.pressed(KeyCode::ShiftLeft) {
        if let Some(selected_camera) = global.selected_camera.take() {
            if selected_camera + 1 != global.last_used_debug_cameras.len() {
                let entity = global.last_used_debug_cameras[selected_camera];
                debug_cameras.get_mut(entity).unwrap().1.focus = true;
            }
        }
        return;
    }

    // Spawn new
    if keys.just_pressed(KeyCode::F1) {
        spawn_new_debug_camera(&mut commands);
        return;
    }

    // Switch to game camera
    if keys.just_pressed(KeyCode::Escape) {
        #[cfg(not(feature = "ui"))]
        switch_to_game_camera(&mut cameras, &mut debug_cameras, &mut global, &mut window);
        #[cfg(feature = "ui")]
        switch_to_game_camera(
            &mut cameras,
            &mut debug_cameras,
            &mut global,
            &mut window,
            &mut popup_event,
        );
    }

    // Switch to selected debug camera
    if keys.just_pressed(KeyCode::Tab) {
        global.selected_camera = Some(match global.selected_camera {
            Some(selected_camera) => match selected_camera == 0 {
                true => global.last_used_debug_cameras.len() - 1,
                false => selected_camera - 1,
            },
            None => {
                if global.last_used_debug_cameras.is_empty() {
                    spawn_new_debug_camera(&mut commands);
                    return;
                }

                let len = global.last_used_debug_cameras.len();
                match len == 1 {
                    true => 0,
                    false => len - 2,
                }
            }
        });
    }

    // Show UI for debug camera selection
    #[cfg(feature = "ui")]
    if global.selected_camera.is_some() {
        ui::debug_camera_selector_ui(&mut debug_cameras, &mut global, &mut popup_event);
    }
}

fn switch_to_game_camera(
    cameras: &mut Query<(Entity, &mut Camera)>,
    debug_cameras: &mut Query<(Entity, &mut DebugCamera, &DebugCameraData)>,
    global: &mut ResMut<DebugCameraGlobalData>,
    window: &mut Query<&mut Window, With<PrimaryWindow>>,
    #[cfg(feature = "ui")] popup_event: &mut EventWriter<PopupEvent>,
) {
    let last = match global.last_used_origin_camera.take() {
        Some(x) => x,
        None => return,
    };

    // Deactive debug camera
    for mut active_camera in cameras.iter_mut().filter(|x| x.1.is_active) {
        if let Ok(mut debug_camera) = debug_cameras.get_mut(active_camera.0) {
            active_camera.1.is_active = false;
            debug_camera.1.focus = false;
        }
    }

    // Activate previous game camera
    if let Ok(mut camera) = cameras.get_mut(last.camera) {
        camera.1.is_active = true;
    }

    // Set cursor
    let mut window = window
        .get_single_mut()
        .expect("Expected primary window to exist");
    window.cursor = last.cursor;

    bevy::log::info!("Switched to game camera");
    #[cfg(feature = "ui")]
    popup_event.send(PopupEvent::new(
        PopupPosition::BelowCenter,
        1.0,
        move |ui| {
            ui.strong("Switched to game camera");
        },
    ));
}

#[allow(clippy::type_complexity)]
fn initialization(
    mut commands: Commands,
    mut cameras: Query<(
        Entity,
        &mut Camera,
        &GlobalTransform,
        &Transform,
        Option<&DebugCamera>,
    )>,
    mut entities: Query<
        (
            Entity,
            &DebugCamera,
            Option<&GlobalTransform>,
            Option<&Transform>,
        ),
        Added<DebugCamera>,
    >,
    mut window: Query<&mut Window, With<PrimaryWindow>>,
    mut global: ResMut<DebugCameraGlobalData>,
    #[cfg(feature = "ui")] mut popup_event: EventWriter<PopupEvent>,
) {
    for (entity, debug_camera, global_transform, transform) in entities.iter_mut() {
        let active_camera = cameras.iter_mut().find(|x| x.1.is_active);
        let mut e = commands.get_entity(entity).unwrap();

        let id = global.next_id;
        global.next_id += 1;

        // Set default transforms
        let global_transform = match global_transform {
            Some(global_transform) => *global_transform,
            None => match &active_camera {
                Some(a) => *a.2,
                None => GlobalTransform::default(),
            },
        };
        let transform = match transform {
            Some(transform) => *transform,
            None => match &active_camera {
                Some(a) => *a.3,
                None => Transform::default(),
            },
        };

        // Deactive origin camera, and manage window
        if debug_camera.focus {
            #[cfg(not(feature = "ui"))]
            focus_camera(
                &mut window,
                &mut global,
                &mut cameras,
                entity,
                id,
                &mut popup_event,
            );
            #[cfg(feature = "ui")]
            focus_camera(
                &mut window,
                &mut global,
                &mut cameras,
                entity,
                id,
                &mut popup_event,
            );
        } else {
            let pos = global.last_used_debug_cameras.len() - 1;
            global.last_used_debug_cameras.insert(pos, entity);
        }

        // Insert new components
        e.insert((
            Camera3dBundle {
                camera: Camera {
                    is_active: debug_camera.focus,
                    ..Default::default()
                },
                global_transform,
                transform,
                ..Default::default()
            },
            DebugCameraData {
                id,
                last_change_position_time: 0.0,
                current_speed: debug_camera.base_speed,
            },
        ));
    }
}

fn focus_system(
    mut cameras: Query<(
        Entity,
        &mut Camera,
        &GlobalTransform,
        &Transform,
        Option<&DebugCamera>,
    )>,
    mut entities: Query<(Entity, &DebugCamera, &DebugCameraData), Changed<DebugCamera>>,
    mut window: Query<&mut Window, With<PrimaryWindow>>,
    mut global: ResMut<DebugCameraGlobalData>,
    #[cfg(feature = "ui")] mut popup_event: EventWriter<PopupEvent>,
) {
    for (entity, _, data) in entities.iter_mut().filter(|x| x.1.focus) {
        #[cfg(not(feature = "ui"))]
        focus_camera(
            &mut window,
            &mut global,
            &mut cameras,
            entity,
            data.id,
            &mut popup_event,
        );
        #[cfg(feature = "ui")]
        focus_camera(
            &mut window,
            &mut global,
            &mut cameras,
            entity,
            data.id,
            &mut popup_event,
        );
    }
}

fn focus_camera(
    window: &mut Query<&mut Window, With<PrimaryWindow>>,
    global: &mut ResMut<DebugCameraGlobalData>,
    cameras: &mut Query<(
        Entity,
        &mut Camera,
        &GlobalTransform,
        &Transform,
        Option<&DebugCamera>,
    )>,
    entity: Entity,
    id: u64,
    #[cfg(feature = "ui")] popup_event: &mut EventWriter<PopupEvent>,
) {
    if let Ok(mut debug_camera) = cameras.get_mut(entity) {
        // Skip if camera is already active
        if debug_camera.1.is_active {
            return;
        }

        debug_camera.1.is_active = true;
    }

    if let Some(mut active_camera) = cameras.iter_mut().find(|x| x.1.is_active) {
        if active_camera.4.is_none() {
            let mut window = window
                .get_single_mut()
                .expect("Expected primary window to exist");

            global.last_used_origin_camera = Some(DebugCameraLastUsedOriginCameraData {
                camera: active_camera.0,
                cursor: window.cursor,
            });

            window.cursor.grab_mode = CursorGrabMode::Locked;
            window.cursor.visible = false;
        }
        active_camera.1.is_active = false;
    }

    for (i, e) in global.last_used_debug_cameras.iter().enumerate() {
        if *e == entity {
            global.last_used_debug_cameras.remove(i);
            break;
        }
    }
    global.last_used_debug_cameras.push(entity);

    #[cfg(feature = "ui")]
    popup_event.send(PopupEvent::new(
        PopupPosition::BelowCenter,
        1.0,
        move |ui| {
            ui.strong(format!("Switched to debug camera #{}", id));
        },
    ));
}

fn controller(
    mut cameras: Query<(
        &mut Transform,
        &mut DebugCameraData,
        &mut DebugCamera,
        &Camera,
    )>,
    mut mouse_motion: EventReader<MouseMotion>,
    mut mouse_wheel: EventReader<MouseWheel>,
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    #[cfg(feature = "ui")] mut popup_event: EventWriter<PopupEvent>,
) {
    let (mut transform, mut data, mut debug_camera, _) =
        match cameras.iter_mut().find(|x| x.3.is_active) {
            Some(v) => v,
            None => return,
        };

    // Speed
    for input in mouse_wheel.read() {
        debug_camera.speed_multiplier = (debug_camera.speed_multiplier
            * match input.y > 0.0 {
                true => 1.1,
                false => 0.9,
            })
        .clamp(
            *debug_camera.speed_multiplier_range.start(),
            *debug_camera.speed_multiplier_range.end(),
        );

        #[cfg(feature = "ui")]
        {
            let value = debug_camera.speed_multiplier;
            popup_event.send(PopupEvent::new(
                PopupPosition::BelowCenter,
                0.5,
                move |ui| {
                    ui.label(format!("Speed multiplier: {:.2}", value));
                },
            ));
        }
    }

    // Position
    let mut translation = Vec3::ZERO;
    if keys.pressed(KeyCode::KeyW) {
        translation -= Vec3::from(transform.local_z());
    }
    if keys.pressed(KeyCode::KeyS) {
        translation += Vec3::from(transform.local_z());
    }
    if keys.pressed(KeyCode::KeyA) {
        translation -= Vec3::from(transform.local_x());
    }
    if keys.pressed(KeyCode::KeyD) {
        translation += Vec3::from(transform.local_x());
    }
    if keys.pressed(KeyCode::KeyQ) {
        translation -= Vec3::Y;
    }
    if keys.pressed(KeyCode::KeyE) {
        translation += Vec3::Y;
    }

    transform.translation += translation.normalize_or_zero()
        * (data.current_speed * debug_camera.speed_multiplier * time.delta_seconds());

    // Rotation
    for input in mouse_motion.read() {
        let (mut y, mut x, _) = transform.rotation.to_euler(EulerRot::YXZ);

        x -= (input.delta.y * debug_camera.sensitivity).to_radians();
        x = x.clamp(-MOUSE_LOOK_X_LIMIT, MOUSE_LOOK_X_LIMIT);

        y -= (input.delta.x * debug_camera.sensitivity).to_radians();

        transform.rotation = Quat::from_rotation_y(y) * Quat::from_rotation_x(x);
    }

    // Increase speed
    if translation != Vec3::ZERO {
        data.current_speed +=
            data.current_speed * time.delta_seconds() * debug_camera.speed_increase;
        data.last_change_position_time = time.elapsed_seconds();
    } else if data.last_change_position_time > RESET_SPEED_THRESHOLD_IN_SECONDS {
        data.current_speed = debug_camera.base_speed;
    }
}
