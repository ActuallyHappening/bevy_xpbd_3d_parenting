# bevy_xpbd3d_parenting
Allows children of a `bevy_xpbd_3d` `RigidBody` to exert forces on their parents.

## Installation
Currently not published on crates.io because it relies on unreleased aspects of `bevy_xpbd`, so you'll need to use a git dependency:
```toml
# Use the latest git release of bevy_xpbd3d_parenting
[dependencies.bevy_xpbd3d_parenting]
git = "https://github.com/ActuallyHappening/bevy_xpbd3d_parenting.git"
default-features = false

# bevy_xpbd3d_parenting relies on unreleased aspects of bevy_xpbd at the moment
[dependencies.bevy_xpbd_3d]
git = "https://github.com/Jondolf/bevy_xpbd.git"
```

## Usage
See example