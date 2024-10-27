# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased] - 
### Plan
- Extract RON config
- Extract engine into a crate

## [0.1.1] - 2024-10-27
### Added
- CLI: signal handling with Tokio
- parser: ignore articles
- parser: add `ls` alias (I've typed it in many types by mistake to try to see the room exits - let it be there)

## [0.1.0] - 2023-11-13
- Initial playable game prototype.

### Added
- Player movement and room navigation.
- Path blockers using access levels.
- Basic story and narrative setup.
- Examining, reading items, and interacting with the environment.
- Picking up items and interacting with containers.
- Simple food mechanics.
- Multiple win and lose conditions.
- Basic CLI.

### Known Roadmap
- Planned improvements for action system, parser, puzzles, and story depth.
- Planned UI enhancements including colors, text animations, and ASCII illustrations (?).
