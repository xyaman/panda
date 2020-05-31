# Changelog
All notable changes to this project will be documented in this file.

## [0.5.3] ??????????????????
### Added
- `Message.add_reaction()`, alias for `http.add_message_reaction()`


### Fixes
- PresenceUpdate event now works properly

## [0.5.2] - 2020-05-26
### Added
- `Message.remove()`, alias for `http.delete_message()`
- Typing Start and Get Pinned Messages route

### Fixes
- Now all models fields are public instead of using methods
- Fixed delete route
- Fixed typo in Message model (attatchment instead of attachment)

## [0.5.1] - 2020-05-26
### Added
- All Channel fields
- All Message fields

### Changes
- `tokio-runtime` is now the default runtime

### Fixes
- Fix typo in Channel id field