Simple, metrically correct, PBR compatible and randomly painted mesh for better differentiation of prototype objects.

# Example
In pure Bevy probably you will create a prototype floor like that:
```rust
commands.spawn(MaterialMeshBundle {
    mesh: meshes.add(Cuboid::new(50.0, 2.0, 50.0)),
    material: materials.add(Color::RED.into()),
    ..default()
});
```
a solid red or any other color which mixing in eyes. Scene with colors like that it will quickly become unreadable, what you can see on the screenshot below:
![Misleading textures](https://raw.githubusercontent.com/Vixenka/bevy_dev/master/images/prototype_material/misleading_textures.webp)

But with tool from this create you can archive better results just by change few chars:
```rust
commands.spawn((
    Mesh3d(meshes.add(Cuboid::new(50.0, 2.0, 50.0))),
    PrototypeMaterial::new("floor"),
    Transform::default(),
));
```

Previous red color changed to string, why? Because in this case you can simple describe what you want to add here in future like `player hat` or whatever you want. Color is random generated based on this string, which means you will get the same color for every next program run.
And this will be the result of this small changes:
![Prototype material](https://raw.githubusercontent.com/Vixenka/bevy_dev/master/images/prototype_material/showcase.webp)
