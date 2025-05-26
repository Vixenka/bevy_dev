use bevy::{
    prelude::*,
    render::{
        camera::RenderTarget,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
    },
};
use bevy_egui::{
    egui::{self, Color32, Frame, Margin, Stroke, TextureId},
    EguiContexts,
};

use crate::ui::popup::{PopupEvent, PopupPosition};

use super::{DebugCamera, DebugCameraData, DebugCameraGlobalData};

const PREVIEW_WIDTH: u32 = 200;
const PREVIEW_HEIGHT: u32 = 112;

pub(super) fn debug_camera_selector_ui(
    debug_cameras: &mut Query<(
        Entity,
        &mut DebugCamera,
        &DebugCameraData,
        Option<&DebugCameraPreview>,
    )>,
    global: &mut ResMut<DebugCameraGlobalData>,
    popup_event: &mut EventWriter<PopupEvent>,
) {
    let mut data = Vec::new();
    for entity in global.last_used_debug_cameras.iter() {
        let camera = debug_cameras.get_mut(*entity).unwrap();
        data.push((camera.2.id, camera.3.cloned()));
    }

    let selected_camera = global.selected_camera.unwrap();

    popup_event.write(PopupEvent::new(PopupPosition::Center, 0.0, move |ui| {
        ui.horizontal_wrapped(|ui| {
            for (i, entity) in data.iter().enumerate().rev() {
                ui.allocate_ui(
                    egui::vec2(PREVIEW_WIDTH as f32 + 3.0, PREVIEW_HEIGHT as f32 + 16.0),
                    |ui| {
                        Frame {
                            inner_margin: Margin::same(1),
                            stroke: match selected_camera == i {
                                true => Stroke::new(1.5, Color32::WHITE),
                                false => Stroke::NONE,
                            },
                            ..Default::default()
                        }
                        .show(ui, |ui| {
                            ui.vertical_centered(|ui| {
                                ui.strong(format!("Camera #{}", entity.0));

                                if let Some(preview) = &entity.1 {
                                    ui.add(egui::widgets::Image::new(
                                        egui::load::SizedTexture::new(
                                            preview.texture_id,
                                            egui::vec2(PREVIEW_WIDTH as f32, PREVIEW_HEIGHT as f32),
                                        ),
                                    ));
                                } else {
                                    ui.label("No preview");
                                }
                            });
                        });
                    },
                );
            }
        });
    }));
}

pub(super) struct DebugCameraPreviewPlugin;

impl Plugin for DebugCameraPreviewPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_preview_camera).add_systems(
            Update,
            (attach_image_to_new_debug_camera, render_to_preview),
        );
    }
}

#[derive(Clone, Debug, Component)]
pub(super) struct DebugCameraPreview {
    image: Handle<Image>,
    texture_id: TextureId,
    last_render_time: f32,
}

fn attach_image_to_new_debug_camera(
    mut commands: Commands,
    cameras: Query<Entity, Added<DebugCamera>>,
    mut images: ResMut<Assets<Image>>,
    mut contexts: EguiContexts,
) {
    for entity in cameras.iter() {
        let mut image = Image {
            texture_descriptor: TextureDescriptor {
                label: Some("Debug Camera Preview"),
                size: Extent3d {
                    width: PREVIEW_WIDTH,
                    height: PREVIEW_HEIGHT,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: TextureDimension::D2,
                format: TextureFormat::Bgra8UnormSrgb,
                usage: TextureUsages::TEXTURE_BINDING
                    | TextureUsages::COPY_DST
                    | TextureUsages::RENDER_ATTACHMENT,
                view_formats: &[],
            },
            ..Default::default()
        };

        image.resize(Extent3d {
            width: PREVIEW_WIDTH,
            height: PREVIEW_HEIGHT,
            depth_or_array_layers: 1,
        });

        let handle = images.add(image);

        commands.entity(entity).insert(DebugCameraPreview {
            image: handle.clone(),
            texture_id: contexts.add_image(handle),
            last_render_time: 0.0,
        });
    }
}

#[derive(Debug, Component)]
pub(super) struct PreviewCamera;

fn spawn_preview_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Camera {
            is_active: false,
            ..Default::default()
        },
        PreviewCamera,
    ));
}

#[allow(clippy::type_complexity)]
fn render_to_preview(
    mut preview_camera: Query<
        (&mut Camera, &mut Transform, &mut GlobalTransform),
        With<PreviewCamera>,
    >,
    mut debug_cameras: Query<
        (&mut DebugCameraPreview, &Transform, &GlobalTransform),
        (With<DebugCamera>, Without<PreviewCamera>),
    >,
    global: Res<DebugCameraGlobalData>,
    time: Res<Time>,
) {
    let Ok(mut preview_camera) = preview_camera.single_mut() else {
        return;
    };
    if global.selected_camera.is_none() {
        preview_camera.0.is_active = false;
        return;
    }

    let mut debug_camera = match debug_cameras
        .iter_mut()
        .min_by(|x, y| x.0.last_render_time.total_cmp(&y.0.last_render_time))
    {
        Some(camera) => camera,
        None => return,
    };
    debug_camera.0.last_render_time = time.elapsed_secs();

    let image_render_target = debug_camera.0.image.clone().into();
    preview_camera.0.target = RenderTarget::Image(image_render_target);
    preview_camera.0.is_active = true;

    *preview_camera.1 = *debug_camera.1;
    *preview_camera.2 = *debug_camera.2;
}
