#![doc = include_str!("../docs/features/prototype_material.md")]

use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use bevy::{
    asset::weak_handle,
    image::{CompressedImageFormats, ImageFormat, ImageSampler, ImageType},
    prelude::*,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{AsBindGroup, ShaderRef},
    },
};
use random_color::{Luminosity, RandomColor};

use crate::DevAssets;

const SHADER_PATH: &str = "shaders/prototype_material.wgsl";
const SHADER_HANDLE: Handle<Shader> = weak_handle!("0ced3da7-55d3-43be-9e04-5637b0e9ceef");

/// Plugin for [`crate::prototype_material`] feature. Attachts resources and initialization system.
/// # Remarks
/// This plugin is necessary to use [`crate::prototype_material`] feature. It is added to [`App`] by [`crate::DevPlugins`].
pub struct PrototypeMaterialPlugin;

impl Plugin for PrototypeMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<PrototypeMaterialAsset>::default())
            .insert_resource(PrototypeMaterialResource::default())
            .add_systems(PostUpdate, initialization);
    }
}

/// Component which includes [`PrototypeMaterialAsset`] to [`Entity`] in the next [`PostUpdate`].
#[derive(Component, Debug, Clone, Copy)]
pub struct PrototypeMaterial {
    color: Color,
}

impl PrototypeMaterial {
    /// Creates a prototype material with procedural color.
    /// # Arguments
    /// * `feature_name` - Describe the feature that this prototype material is for, e.g. `floor` or `wall`. It is used to generate a procedural color that is the same every time the program is run.
    pub fn new(feature_name: &str) -> Self {
        let mut hasher = DefaultHasher::new();
        feature_name.hash(&mut hasher);
        let hash = hasher.finish();

        let rgb = RandomColor::new()
            .luminosity(Luminosity::Bright)
            .seed(hash)
            .to_rgb_array();

        Self {
            color: Color::srgb_u8(rgb[0], rgb[1], rgb[2]),
        }
    }
}

/// A [`Material`] that uses a [`PrototypeMaterialAsset`] shader.
#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
pub struct PrototypeMaterialAsset {
    #[uniform(0)]
    pub color: LinearRgba,
    #[texture(1)]
    #[sampler(2)]
    pub base_texture: Handle<Image>,
}

impl Material for PrototypeMaterialAsset {
    fn vertex_shader() -> ShaderRef {
        SHADER_HANDLE.into()
    }

    fn fragment_shader() -> ShaderRef {
        SHADER_HANDLE.into()
    }
}

#[derive(Resource, Default, Clone, Debug)]
struct PrototypeMaterialResource {
    base_texture: Option<Handle<Image>>,
}

fn initialization(
    mut commands: Commands,
    mut entities: Query<(Entity, &PrototypeMaterial), Changed<PrototypeMaterial>>,
    mut resource: ResMut<PrototypeMaterialResource>,
    mut images: ResMut<Assets<Image>>,
    mut shaders: ResMut<Assets<Shader>>,
    mut materials: ResMut<Assets<PrototypeMaterialAsset>>,
) {
    if entities.is_empty() {
        return;
    }

    if resource.base_texture.is_none() {
        resource.base_texture = Some(
            images.add(
                Image::from_buffer(
                    &DevAssets::get("textures/prototype.png")
                        .expect("Prototype material texture is not embedded")
                        .data,
                    ImageType::Format(ImageFormat::Png),
                    CompressedImageFormats::all(),
                    false,
                    ImageSampler::Default,
                    RenderAssetUsages::all(),
                )
                .expect("Unable to load prototype material texture"),
            ),
        );

        shaders.insert(
            &SHADER_HANDLE,
            Shader::from_wgsl(
                String::from_utf8(
                    DevAssets::get(SHADER_PATH)
                        .expect("Prototype material shader is not embedded")
                        .data
                        .into(),
                )
                .expect("Prototype material shader is not valid UTF-8"),
                SHADER_PATH,
            ),
        )
    }

    for (entity, material) in entities.iter_mut() {
        commands
            .entity(entity)
            .insert(MeshMaterial3d(materials.add(PrototypeMaterialAsset {
                color: material.color.to_linear(),
                base_texture: resource.base_texture.clone().unwrap(),
            })));
    }
}

/// A component bundle for entities with a [`Mesh`] and a [`PrototypeMaterial`]'s logic.
#[derive(Default, Clone)]
pub struct PrototypeMaterialMeshBundle {
    pub mesh: Handle<Mesh>,
    /// Describe the feature that this prototype material is for, e.g. `floor` or `wall`. It is used to generate a random color that is the same every time the program is run.
    pub material: &'static str,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    /// User indication of whether an entity is visible
    pub visibility: Visibility,
    /// Inherited visibility of an entity.
    pub inherited_visibility: InheritedVisibility,
    /// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering
    pub view_visibility: ViewVisibility,
}

// // Generated via `cargo expand` command and next slightly modified.
// unsafe impl bevy::ecs::bundle::Bundle for PrototypeMaterialMeshBundle {
//     fn component_ids(
//         components: &mut bevy::ecs::component::Components,
//         storages: &mut bevy::ecs::storage::Storages,
//         ids: &mut impl FnMut(bevy::ecs::component::ComponentId),
//     ) {
//         <Handle<Mesh> as bevy::ecs::bundle::Bundle>::component_ids(components, storages, &mut *ids);
//         <PrototypeMaterial as bevy::ecs::bundle::Bundle>::component_ids(
//             components, storages, &mut *ids,
//         );
//         <Transform as bevy::ecs::bundle::Bundle>::component_ids(components, storages, &mut *ids);
//         <GlobalTransform as bevy::ecs::bundle::Bundle>::component_ids(
//             components, storages, &mut *ids,
//         );
//         <Visibility as bevy::ecs::bundle::Bundle>::component_ids(components, storages, &mut *ids);
//         <InheritedVisibility as bevy::ecs::bundle::Bundle>::component_ids(
//             components, storages, &mut *ids,
//         );
//         <ViewVisibility as bevy::ecs::bundle::Bundle>::component_ids(
//             components, storages, &mut *ids,
//         );
//     }
//     unsafe fn from_components<__T, __F>(_ctx: &mut __T, _func: &mut __F) -> Self
//     where
//         __F: FnMut(&mut __T) -> bevy::ecs::ptr::OwningPtr<'_>,
//     {
//         panic!("PrototypeMaterialMeshBundle cannot be constructed from components because it contains a non-component field: `material`")
//     }
// }

// impl bevy::ecs::bundle::DynamicBundle for PrototypeMaterialMeshBundle {
//     #[allow(unused_variables)]
//     #[inline]
//     fn get_components(
//         self,
//         func: &mut impl FnMut(bevy::ecs::component::StorageType, bevy::ecs::ptr::OwningPtr<'_>),
//     ) {
//         self.mesh.get_components(&mut *func);
//         PrototypeMaterial::new(self.material).get_components(&mut *func);
//         self.transform.get_components(&mut *func);
//         self.global_transform.get_components(&mut *func);
//         self.visibility.get_components(&mut *func);
//         self.inherited_visibility.get_components(&mut *func);
//         self.view_visibility.get_components(&mut *func);
//     }
// }
