# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/),
and this project adheres to [Semantic Versioning](https://semver.org/).

## [Unreleased]

## [0.3.0] - 2025-02-25

### Added

  - Change the project's name to Open95.
  - Move key generation functions to a struct.
  - Add unit tests to the code to make sure key generation is working properly.

## [0.2.0] - 2023-12-26

### Changed

  - Multi-threading in generating product keys is now handled by rayon.
  - Command-line argument parsing is now handled by `clap`.

## [0.1.0] - 2023-10-21

### Added

  - Initial release of `win95-keygen-rs`.

[Unreleased]: https://github.com/walker84837/open95/compare/v0.3.0...HEAD
[0.3.0]: https://github.com/walker84837/open95/compare/0.2.0...v0.3.0
[0.2.0]: https://github.com/walker84837/open95/compare/v0.1.0...0.2.0
[0.1.0]: https://github.com/walker84837/open95/releases/tag/v0.1.0
