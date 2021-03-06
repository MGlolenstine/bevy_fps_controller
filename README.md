# bevy_fps_controller
[![Crates.io](https://img.shields.io/crates/v/bevy_fps_controller)](https://crates.io/crates/bevy_fps_controller)
![Crates.io](https://img.shields.io/crates/l/bevy_fps_controller)
![docs.rs](https://img.shields.io/docsrs/bevy_fps_controller)


A basic first-person-shooter player controller for Bevy 0.4

## Controls
* WASD to move
* SPACE to ascend
* LSHIFT to sprint
* ESC to grab/release cursor.

## Note
This is more or less a copy of [bevy_flycam](https://raw.githubusercontent.com/sburris0/bevy_flycam)...

### Changes
* Rigidbody
* Gravity
* Running

## Usage
1. Add to `Cargo.toml` or copy `lib.rs` to your own file
```toml
[dependencies]
bevy = "0.4"
bevy_fps_controller = "*"
```

or

```toml
[dependencies]
bevy = "0.4"
bevy_fps_controller = { git = "https://github.com/mglolenstine/bevy_fps_controller" }
```

2. Include the `FPSControllerPlugin`
```rust
use bevy_fps_controller::FPSControllerPlugin;
```
This will spawn a camera for you. 
Use `NoControllerPlugin` if you do not want this and make sure to use `.with(FPSController)` on your own player or else this plugin won't know what to move.

3. Add the `FPSControllerPlugin`:
```rust
#[bevy_main]
fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(FPSControllerPlugin)
        .run();
}
```

## Customization
To modify player movement speed, sprint multiplier or mouse sensitivity, import `bevy_fps_controller::MovementSettings` and add it as a resource:
```Rust
#[bevy_main]
fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(FPSControllerPlugin)
        .add_resource(MovementSettings {
            sensitivity: 0.00015, // default: 0.00006
            speed: 150.0, // default: 12.0
            speed_multiplier: 1.2, // default: 1.5
        })
        .run();
}
```

## Contributing
PRs are very welcome.
