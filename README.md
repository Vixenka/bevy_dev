# bevy_dev
Dev tools for [Bevy Engine](https://bevyengine.org/). For faster prototyping.

![Showcase](/images/showcase.webp)

### Features
- [x] [Prototype materials](#prototype-materials)

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
    mesh: meshes.add(shape::Box::new(50.0, 2.0, 50.0).into()),
    material: materials.add(Color::RED.into()),
    ..default()
});
```
a solid red or any other color which mixing in eyes. Scene with colors like that it will quickly become unreadable.

But with tool from this create you can archive better results just by change few chars:
```rust
commands.spawn(PrototypeMaterialMeshBundle {
    mesh: meshes.add(shape::Box::new(50.0, 2.0, 50.0).into()),
    material: "floor",
    ..default()
});
```
Previous red color changed to string, why? Because in this case you can simple describe what you want to add here in future like `player hat` or whatever you want. Color is random generated based on this string, which means you will get the same color for every next program run.

## Bevy compability
| bevy   | bevy_dev |
|--------|---------------|
| 0.12.1 | 0.1           |

## License
bevy_dev is licensed under the [MIT](/LICENSE) license.
