# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

- Prototype materials - simple, metrically correct, PBR compatible and randomly painted mesh for better differentiation of prototype objects.

[unreleased]: https://github.com/Vixenka/bevy_dev/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/Vixenka/bevy_dev/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/Vixenka/bevy_dev/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/Vixenka/bevy_dev/releases/tag/v0.1.0
