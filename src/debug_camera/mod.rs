use std::{fmt::Debug, ops::RangeInclusive};

use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
    window::Cursor,
};

#[cfg(feature = "ui")]
use crate::ui::popup::PopupEvent;

mod controller;
mod focus;
mod initialization;
#[cfg(feature = "ui")]
mod ui;

#[cfg(feature = "ui")]
const SELECTOR_NEXT_ELEMENT_THRESHOLD_IN_SECONDS: f32 = 0.25;
#[cfg(feature = "ui")]
const SELECTOR_NEXT_ELEMENT_IN_SECONDS: f32 = 0.1;

pub struct DebugCameraPlugin {
    pub switcher: DebugCameraSwitcher,
    pub spawn_debug_camera_if_any_camera_exist: bool,
}

impl Default for DebugCameraPlugin {
    fn default() -> Self {
        Self {
            switcher: Default::default(),
            spawn_debug_camera_if_any_camera_exist: true,
        }
    }
}

impl Plugin for DebugCameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DebugCameraGlobalData>().add_systems(
            Update,
            (
                initialization::system,
                focus::system
                    .after(initialization::system)
                    .run_if(focus::run_if_changed),
                controller::system,
            ),
        );

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

        if self.spawn_debug_camera_if_any_camera_exist {
            app.add_systems(PostUpdate, spawn_debug_camera_if_any_camera_exist);
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
    last_switch_time: f32,
    next_id: u64,
}

impl Default for DebugCameraGlobalData {
    fn default() -> Self {
        Self {
            default_value: DebugCamera::default(),
            last_used_debug_cameras: Vec::new(),
            last_used_origin_camera: None,
            selected_camera: None,
            last_switch_time: 0.0,
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

#[allow(clippy::too_many_arguments)]
fn switcher(
    mut commands: Commands,
    mut debug_cameras: Query<(Entity, &mut DebugCamera, &DebugCameraData)>,
    mut global: ResMut<DebugCameraGlobalData>,
    keys: Res<ButtonInput<KeyCode>>,
    #[cfg(feature = "ui")] mut popup_event: EventWriter<PopupEvent>,
    #[cfg(feature = "ui")] time: Res<Time>,
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
        commands.spawn(DebugCamera::default());
        return;
    }

    // Switch to game camera
    if keys.just_pressed(KeyCode::Escape) {
        for mut debug_camera in debug_cameras.iter_mut() {
            debug_camera.1.focus = false;
        }
    }

    // Switch to selected debug camera
    #[cfg(not(feature = "ui"))]
    let event = keys.just_pressed(KeyCode::Tab);
    #[cfg(feature = "ui")]
    let event = select_next_camera_key_event(&mut global, &keys, &time);

    if event {
        global.selected_camera = Some(match global.selected_camera {
            Some(selected_camera) => match selected_camera == 0 {
                true => global.last_used_debug_cameras.len() - 1,
                false => selected_camera - 1,
            },
            None => {
                if global.last_used_debug_cameras.is_empty() {
                    commands.spawn(DebugCamera::default());
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

fn select_next_camera_key_event(
    global: &mut ResMut<DebugCameraGlobalData>,
    keys: &Res<ButtonInput<KeyCode>>,
    time: &Res<Time>,
) -> bool {
    if keys.just_pressed(KeyCode::Tab) {
        global.last_switch_time =
            time.elapsed_seconds() + SELECTOR_NEXT_ELEMENT_THRESHOLD_IN_SECONDS;
        return true;
    }

    if keys.pressed(KeyCode::Tab)
        && global.last_switch_time + SELECTOR_NEXT_ELEMENT_IN_SECONDS < time.elapsed_seconds()
    {
        global.last_switch_time = time.elapsed_seconds();
        true
    } else {
        false
    }
}

fn spawn_debug_camera_if_any_camera_exist(
    mut commands: Commands,
    mut mouse_motion: EventReader<MouseMotion>,
    mut mouse_wheel: EventReader<MouseWheel>,
    query: Query<(), With<Camera>>,
) {
    if query.is_empty() {
        commands.spawn(DebugCamera::default()).insert(Transform {
            translation: Vec3::new(0.0, 0.0, -5.0),
            rotation: Quat::from_rotation_y(180.0f32.to_radians()),
            ..Default::default()
        });
    }

    // Clear first gained events
    mouse_motion.clear();
    mouse_wheel.clear();
}
