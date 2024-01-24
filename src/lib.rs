/*!
 * Dev tools for [Bevy Engine](https://bevyengine.org/). For faster prototyping.
 *
 * ![Showcase](https://raw.githubusercontent.com/Vixenka/bevy_dev/master/images/prototype_material/showcase.webp)
 *
 * ### Features
 * - [x] [Prototype materials](prototype_material/index.html) - simple, metrically correct, PBR compatible and randomly painted mesh for better differentiation of prototype objects.
 *
 * ### Initialization
 * To start using features of this crate you need to initialize features in your Bevy's app.
 * You can just use a [`DevPlugins`] plugin to enable all default features or you can add only features you need by adding feature's plugins directly.
 */

pub mod prelude;
pub mod prototype_material;

use bevy::prelude::*;
use rust_embed::RustEmbed;

/// Plugin which enables default development features from `bevy_dev` crate.
/// # Remarks
/// This plugin contains this plugins:
/// - [`prototype_material::PrototypeMaterialPlugin`]
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
        app.add_plugins(prototype_material::PrototypeMaterialPlugin);
    }
}

#[derive(RustEmbed)]
#[folder = "assets"]
pub(crate) struct DevAssets;
