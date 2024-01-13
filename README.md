# bevy_xpbd_3d_parenting
Allows children of a `bevy_xpbd_3d` `RigidBody` to exert forces on their parents.

## Installation
Currently not published on crates.io, so you'll need to use a git dependency:
```toml
# Use the latest git release of bevy_xpbd_3d_parenting
[dependencies.bevy_xpbd_3d_parenting]
git = "https://github.com/ActuallyHappening/bevy_xpbd_3d_parenting.git"
default-features = false
```

## Usage examples
Run:
```sh
cargo r --example rotating --features bevy_xpbd_3d/async-collider
```

## Compatibility table
| Bevy | Bevy XPBD | Bevy XPBD 3D Parenting |
| ---- | --------- | ---------------------- |
| 0.12 | 0.3.3     | 0.1.0									|