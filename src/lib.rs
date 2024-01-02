pub mod prelude;
pub mod prototype_material;

use bevy::prelude::*;
use rust_embed::RustEmbed;

pub struct DevPlugins;

impl Plugin for DevPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins(prototype_material::PrototypeMaterialPlugin);
    }
}

#[derive(RustEmbed)]
#[folder = "assets"]
pub(crate) struct DevAssets;
