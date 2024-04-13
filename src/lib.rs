/*!
 * Dev tools for [Bevy Engine](https://bevyengine.org/). For faster prototyping.
 *
 * [![Showcase](https://raw.githubusercontent.com/Vixenka/bevy_dev/master/images/debug_camera/showcase.webp)](https://github.com/Vixenka/bevy_dev/assets/44348304/073d635c-3d58-4c36-8e01-8a8686f5060b)
 *
 * ### Features
 * - [x] [Debug camera](debug_camera/index.html) - tool for getting another perspective to the scene, also known as fly camera.
 * - [x] [Prototype materials](prototype_material/index.html) - simple, metrically correct, PBR compatible and randomly painted mesh for better differentiation of prototype objects.
 *
 * ### Initialization
 * To start using features of this crate you need to initialize features in your Bevy's app.
 * You can just use a [`DevPlugins`] plugin to enable all default features or you can add only features you need by adding feature's plugins directly.
 */

pub mod debug_camera;
pub mod prelude;
pub mod prototype_material;

#[cfg(feature = "ui")]
pub mod ui;

use bevy::prelude::*;
use rust_embed::RustEmbed;

/// Plugin which enables default development features from `bevy_dev` crate.
/// # Remarks
/// This plugin contains this plugins:
/// - [`debug_camera::DebugCameraPlugin`]
/// - [`prototype_material::PrototypeMaterialPlugin`]
/// - [`ui::DebugUiPlugin`] if `ui` feature is enabled
/// # Examples
/// You need to add this plugin to your Bevy's app to use features. Or you can add only features you need by adding feature's plugins directly.
/// ```
/// use bevy::prelude::*;
/// use bevy_dev::prelude::*;
///
/// let mut app = App::new();
/// app.add_plugins((DefaultPlugins, DevPlugins));
/// ```
pub struct DevPlugins;

impl Plugin for DevPlugins {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "ui")]
        app.add_plugins(ui::DebugUiPlugin);

        app.add_plugins(debug_camera::DebugCameraPlugin::default())
            .add_plugins(prototype_material::PrototypeMaterialPlugin);
    }
}

#[derive(RustEmbed)]
#[folder = "assets"]
pub(crate) struct DevAssets;
