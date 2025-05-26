# bevy_dev
[![crates.io](https://img.shields.io/crates/v/bevy_dev)](https://crates.io/crates/bevy_dev)
[![docs.rs](https://docs.rs/bevy_dev/badge.svg)](https://docs.rs/bevy_dev)

Dev tools for [Bevy Engine](https://bevyengine.org/). For faster prototyping.

[![Showcase](/images/debug_camera/showcase.webp)](https://github.com/Vixenka/bevy_dev/assets/44348304/073d635c-3d58-4c36-8e01-8a8686f5060b)

### Features
- [x] [Debug camera](/docs/features/debug_camera.md) - tool for getting another perspective to the scene, also known as fly camera.
- [x] [Prototype material](/docs/features/prototype_material.md) - simple, metrically correct, PBR compatible and randomly painted mesh for better differentiation of prototype objects.

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

## Bevy compability
| bevy   | bevy_dev      |
|--------|---------------|
| 0.16.* | 0.5           |
| 0.15.* | 0.4           |
| 0.13.2 | 0.3 - 0.3.1   |
| 0.13.0 | 0.2           |
| 0.12.1 | 0.1 - 0.1.1   |

Read more in the [changelog](/CHANGELOG.md).

## License
bevy_dev is licensed under the [MIT](/LICENSE) license.
