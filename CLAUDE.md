# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust game project using the Bevy game engine (v0.16.1). The project is called "black-white-legends" and is currently in early development with a minimal hello world implementation.

## Development Commands

### Building
```bash
# Build the project
cargo build

# Build with optimizations (release mode)
cargo build --release
```

### Running
```bash
# Run the game in debug mode
cargo run

# Run in release mode (optimized)
cargo run --release
```

### Testing
```bash
# Run all tests
cargo test

# Run a specific test
cargo test <test_name>

# Run tests with output displayed
cargo test -- --nocapture
```

### Code Quality
```bash
# Run clippy for linting
cargo clippy

# Apply clippy fixes automatically
cargo clippy --fix

# Format code
cargo fmt

# Check formatting without making changes
cargo fmt --check
```

## Architecture

The project uses Bevy, a data-driven game engine built in Rust that uses an Entity Component System (ECS) architecture:

- **main.rs**: Entry point that initializes the Bevy App and registers systems
- Systems are functions that run each frame (Update) or at specific points in the game loop
- Currently has a single `hello_world` system that prints to console

When adding new features:
- Game logic should be implemented as Bevy systems
- Use Bevy's built-in plugins for common functionality (windowing, input, rendering, etc.)
- Components define data, Systems define behavior
- Resources are shared data accessible across systems

## Bevy-Specific Patterns

- Use `App::new()` to create the application
- Add systems with `.add_systems()` specifying the schedule (Startup, Update, etc.)
- Implement systems as functions with Query/Res/ResMut parameters for ECS access
- Use Bevy's prelude (`use bevy::prelude::*;`) for common imports