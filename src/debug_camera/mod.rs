#![doc = include_str!("../../docs/features/debug_camera.md")]

use std::{fmt::Debug, ops::RangeInclusive};

use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
    window::CursorOptions,
};

#[cfg(feature = "ui")]
use crate::ui::popup::{PopupEvent, PopupPosition};

mod controller;
mod focus;
mod initialization;
#[cfg(feature = "ui")]
mod ui;

#[cfg(feature = "ui")]
const SELECTOR_NEXT_ELEMENT_THRESHOLD_IN_SECONDS: f32 = 0.25;
#[cfg(feature = "ui")]
const SELECTOR_NEXT_ELEMENT_IN_SECONDS: f32 = 0.1;

/// Plugin for [`crate::debug_camera`] feature.
///
/// # Remarks
/// This plugin is necessary to use [`crate::debug_camera`] feature. It is added to App by [`crate::DevPlugins`].
///
/// If `ui` feature is enabled, it will require to add [`crate::ui::DebugUiPlugin`] to App, before adding this.
pub struct DebugCameraPlugin {
    /// Allows to switch between cameras, and spawn new debug cameras.
    pub switcher: DebugCameraSwitcher,
    /// Show debug camera renderer preview in selector UI.
    ///
    /// If enabled
    ///
    /// ![Preview enabled](https://raw.githubusercontent.com/Vixenka/bevy_dev/master/images/debug_camera/switching.webp)
    ///
    /// If disabled
    ///
    /// ![Preview disabled](https://raw.githubusercontent.com/Vixenka/bevy_dev/master/images/debug_camera/switching_without_preview.webp)
    ///
    /// # Remarks
    /// This feature requires `ui` feature to be enabled.
    ///
    /// Preview is rendered only when `UI` is showed, and rendered in low resolution. Only one debug camera refresh their preview in one frame, what do not affect performance so much.
    #[cfg(feature = "ui")]
    pub show_preview: bool,
    /// Spawn debug camera if any camera exist.
    ///
    /// # Remarks
    /// Camera is spawned with default values in any [`PostUpdate`] stage if any camera exist.
    pub spawn_debug_camera_if_any_camera_exist: bool,
}

impl Default for DebugCameraPlugin {
    fn default() -> Self {
        Self {
            switcher: Default::default(),
            #[cfg(feature = "ui")]
            show_preview: true,
            spawn_debug_camera_if_any_camera_exist: true,
        }
    }
}

impl Plugin for DebugCameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DebugCameraGlobalData>()
            .init_resource::<DebugCameraControls>()
            .add_systems(
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
            app.add_systems(Update, switcher.before(initialization::system));

            #[cfg(feature = "ui")]
            if self.show_preview {
                app.add_plugins(ui::DebugCameraPreviewPlugin);
            }
        }

        if self.spawn_debug_camera_if_any_camera_exist {
            app.add_systems(PostUpdate, spawn_debug_camera_if_any_camera_exist);
        }
    }
}

/// Setting for debug camera switcher.
#[derive(Default)]
pub enum DebugCameraSwitcher {
    /// The same as [`DebugCameraSwitcher::Active`], but in release builds it will make a warning.
    #[default]
    Default,
    /// Active debug camera switcher.
    Active,
    /// Disable debug camera switcher.
    Disabled,
}

/// Global data for debug camera.
#[derive(Debug, Resource)]
pub struct DebugCameraGlobalData {
    /// Default values for new created debug cameras.
    pub default_value: DebugCamera,
    /// Last used debug cameras, in order.
    pub last_used_debug_cameras: Vec<Entity>,
    /// Last used origin camera.
    pub last_used_origin_camera: Option<DebugCameraLastUsedOriginCameraData>,
    pub(super) selected_camera: Option<usize>,
    #[cfg(feature = "ui")]
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
            #[cfg(feature = "ui")]
            last_switch_time: 0.0,
            next_id: 1,
        }
    }
}

/// Controls used for debug camera.
#[derive(Debug, Resource)]
pub struct DebugCameraControls {
    /// Move forward key, default is [`KeyCode::KeyW`].
    pub move_forward: KeyCode,
    /// Move backward key, default is [`KeyCode::KeyS`].
    pub move_backward: KeyCode,
    /// Move left key, default is [`KeyCode::KeyA`].
    pub move_left: KeyCode,
    /// Move right key, default is [`KeyCode::KeyD`].
    pub move_right: KeyCode,
    /// Move up key, default is [`KeyCode::KeyE`].
    pub move_up: KeyCode,
    /// Move down key, default is [`KeyCode::KeyQ`].
    pub move_down: KeyCode,
    /// Base key used to use switcher, default is [`KeyCode::ShiftLeft`].
    pub switcher_special: KeyCode,
    /// Select next debug camera, default is [`KeyCode::Tab`].
    pub switcher_next: KeyCode,
    /// Spawn new debug camera, default is [`KeyCode::F1`].
    pub new_debug_camera: KeyCode,
    /// Return to game camera, default is [`KeyCode::Escape`].
    pub return_to_game_camera: KeyCode,
}

impl Default for DebugCameraControls {
    fn default() -> Self {
        Self {
            move_forward: KeyCode::KeyW,
            move_backward: KeyCode::KeyS,
            move_left: KeyCode::KeyA,
            move_right: KeyCode::KeyD,
            move_up: KeyCode::KeyE,
            move_down: KeyCode::KeyQ,
            switcher_special: KeyCode::ShiftLeft,
            switcher_next: KeyCode::Tab,
            new_debug_camera: KeyCode::F1,
            return_to_game_camera: KeyCode::Escape,
        }
    }
}

/// Container which contains data about last used origin camera.
#[derive(Debug)]
pub struct DebugCameraLastUsedOriginCameraData {
    /// Entity of last used origin camera.
    pub camera: Entity,
    /// Cursor of window before switching to debug camera.
    pub cursor: CursorOptions,
}

/// Debug camera component. Apply to entity to make it debug camera.
#[derive(Component, Debug, Clone)]
#[non_exhaustive]
pub struct DebugCamera {
    /// Speed increase during flight.
    pub speed_increase: f32,
    /// Sensitivity of managing speed multiplier via mouse wheel.
    pub speed_multiplier: f32,
    /// Range of speed multiplier.
    pub speed_multiplier_range: RangeInclusive<f32>,
    /// Sensitivity of camera rotation.
    pub sensitivity: f32,
    /// Base speed of camera.
    pub base_speed: f32,
    /// Focus on camera. Manage it activation.
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
    speed_level: f32,
}

#[allow(clippy::too_many_arguments)]
#[allow(clippy::type_complexity)]
fn switcher(
    mut commands: Commands,
    #[cfg(not(feature = "ui"))] mut debug_cameras: Query<(
        Entity,
        &mut DebugCamera,
        &DebugCameraData,
    )>,
    #[cfg(feature = "ui")] mut debug_cameras: Query<(
        Entity,
        &mut DebugCamera,
        &DebugCameraData,
        Option<&ui::DebugCameraPreview>,
    )>,
    mut global: ResMut<DebugCameraGlobalData>,
    #[cfg(not(feature = "ui"))] cameras: Query<(), (With<Camera>, Without<DebugCamera>)>,
    #[cfg(feature = "ui")] cameras: Query<
        (),
        (
            With<Camera>,
            Without<DebugCamera>,
            Without<ui::PreviewCamera>,
        ),
    >,
    keys: Res<ButtonInput<KeyCode>>,
    controls: Res<DebugCameraControls>,
    #[cfg(feature = "ui")] mut popup_event: EventWriter<PopupEvent>,
    #[cfg(feature = "ui")] time: Res<Time>,
) {
    if !keys.pressed(controls.switcher_special) {
        if let Some(selected_camera) = global.selected_camera.take() {
            if selected_camera + 1 != global.last_used_debug_cameras.len() {
                let entity = global.last_used_debug_cameras[selected_camera];
                debug_cameras.get_mut(entity).unwrap().1.focus = true;
            }
        }
        return;
    }

    // Spawn new
    if keys.just_pressed(controls.new_debug_camera) {
        commands.spawn(DebugCamera::default());
        return;
    }

    // Switch to game camera
    if keys.just_pressed(controls.return_to_game_camera) {
        if cameras.is_empty() {
            bevy::log::info!("Unable to switch to game camera, no any camera exist");
            #[cfg(feature = "ui")]
            popup_event.write(PopupEvent::new(
                PopupPosition::BelowCenter,
                1.0,
                move |ui| {
                    ui.strong("Unable to switch to game camera, no any camera exist");
                },
            ));
        } else {
            for mut debug_camera in debug_cameras.iter_mut() {
                debug_camera.1.focus = false;
            }
        }
    }

    // Switch to selected debug camera
    #[cfg(not(feature = "ui"))]
    let event = keys.just_pressed(controls.switcher_next);
    #[cfg(feature = "ui")]
    let event = select_next_camera_key_event(&mut global, &keys, &controls, &time);

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

#[cfg(feature = "ui")]
fn select_next_camera_key_event(
    global: &mut ResMut<DebugCameraGlobalData>,
    keys: &Res<ButtonInput<KeyCode>>,
    controls: &Res<DebugCameraControls>,
    time: &Res<Time>,
) -> bool {
    if keys.just_pressed(controls.switcher_next) {
        global.last_switch_time = time.elapsed_secs() + SELECTOR_NEXT_ELEMENT_THRESHOLD_IN_SECONDS;
        return true;
    }

    if keys.pressed(controls.switcher_next)
        && global.last_switch_time + SELECTOR_NEXT_ELEMENT_IN_SECONDS < time.elapsed_secs()
    {
        global.last_switch_time = time.elapsed_secs();
        true
    } else {
        false
    }
}

fn spawn_debug_camera_if_any_camera_exist(
    mut commands: Commands,
    mut mouse_motion: EventReader<MouseMotion>,
    mut mouse_wheel: EventReader<MouseWheel>,
    #[cfg(not(feature = "ui"))] query: Query<(), With<Camera>>,
    #[cfg(feature = "ui")] query: Query<(), (With<Camera>, Without<ui::PreviewCamera>)>,
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
