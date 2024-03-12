# bevy_dev
[![crates.io](https://img.shields.io/crates/v/bevy_dev)](https://crates.io/crates/bevy_dev)
[![docs.rs](https://docs.rs/bevy_dev/badge.svg)](https://docs.rs/bevy_dev)

Dev tools for [Bevy Engine](https://bevyengine.org/). For faster prototyping.

![Showcase](/images/prototype_material/showcase.webp)

### Features
- [x] [Prototype materials](#prototype-materials) - simple, metrically correct, PBR compatible and randomly painted mesh for better differentiation of prototype objects.

## Usage
Add `DevPlugins` to Bevy's App.
```rust
use bevy::prelude::*;
use bevy_dev::prelude::*;

#[bevy_main]
fn main() {
    App::new()
        .add_plugins((DefaultPlugins, DevPlugins))
        .run();
}
```

### Prototype materials
Simple, metrically correct, PBR compatible and randomly painted mesh for better differentiation of prototype objects.

In pure Bevy probably you will create a prototype floor like that:
```rust
commands.spawn(MaterialMeshBundle {
    mesh: meshes.add(Cuboid::new(50.0, 2.0, 50.0)),
    material: materials.add(Color::RED.into()),
    ..default()
});
```
a solid red or any other color which mixing in eyes. Scene with colors like that it will quickly become unreadable, what you can see on the screenshot below:
![Misleading textures](/images/prototype_material/misleading_textures.webp)

But with tool from this create you can archive better results just by change few chars:
```rust
commands.spawn(PrototypeMaterialMeshBundle {
    mesh: meshes.add(Cuboid::new(50.0, 2.0, 50.0)),
    material: "floor",
    ..default()
});
```
Previous red color changed to string, why? Because in this case you can simple describe what you want to add here in future like `player hat` or whatever you want. Color is random generated based on this string, which means you will get the same color for every next program run.
And this will be the result of this small changes:
![Prototype material](https://raw.githubusercontent.com/Vixenka/bevy_dev/master/images/prototype_material/showcase.webp)

## Bevy compability
| bevy   | bevy_dev      |
|--------|---------------|
| 0.13.0 | 0.2           |
| 0.12.1 | 0.1 - 0.1.1   |

## License
bevy_dev is licensed under the [MIT](/LICENSE) license.
