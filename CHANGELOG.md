## 0.2.2
- Force application takes into account the `GlobalTransform` of the parent

## 0.2.1
- Fixed bug in documentation

## 0.2.0
- Bumped support for `bevy 0.13` and `bevy_xpbd 0.4.2`
- Added `InternalForce::Global` and `InternalForce::Local` variants.

## 0.1.1
- Added the `ParentingSystemSet`
- Now manually clears all `ExternalForce`s, since sometimes bevy_xpbd wouldn't

## 0.1.0
- Initial Release