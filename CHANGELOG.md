# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.6.0] - 2025-08-12

### Changed

- Bump crate edition to 2024.
- Fix some clippy issues.
- Bump `random_color` to `1`.
- Bump `rust-embed` to `8`.
- Bump `uuid` to `1`.
- Bump `bevy_egui` to `0.36`.

## [0.5.0] - 2025-05-27

### Changed

- Bump `bevy` to `0.16`.

## [0.4.0] - 2025-05-27

### Changed

- Bump `bevy` to `0.15`.
- Remove or temporary delete `PrototypeMaterialMeshBundle`.

## [0.3.1] - 2024-04-24

### Fixed

- [Startup fail for `ui` feature when `bevy_egui` plugin is already added.](https://github.com/Vixenka/bevy_dev/pull/3)

## [0.3.0] - 2024-04-13

### Added

- Debug camera - tool for getting another perspective to the scene.
- Rust feature `ui` - enables UI from this crate based on `egui`.
- Popup - simple notifications for user, require `ui` feature enabled.
- Changelog history (this file).

### Fixed

- [Fix orientation of procedural material texture on walls.](https://github.com/Vixenka/bevy_dev/issues/2)

## [0.2.0] - 2024-03-12

### Changed

- Bump `bevy` to `0.13.0`.
- Bump `rust-embed` to `8.3.0`.

## [0.1.1] - 2024-01-24

### Changed

- Improve documentation of prototype materials.
- Bump to latest crate versions.

## [0.1.0] - 2024-01-02

### Added

- Prototype material - simple, metrically correct, PBR compatible and randomly painted mesh for better differentiation of prototype objects.

[unreleased]: https://github.com/Vixenka/bevy_dev/compare/v0.5.0...HEAD
[0.5.0]: https://github.com/Vixenka/bevy_dev/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/Vixenka/bevy_dev/compare/v0.3.1...v0.4.0
[0.3.1]: https://github.com/Vixenka/bevy_dev/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/Vixenka/bevy_dev/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/Vixenka/bevy_dev/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/Vixenka/bevy_dev/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/Vixenka/bevy_dev/releases/tag/v0.1.0
