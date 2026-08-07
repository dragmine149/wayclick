# WayClicker

An autoclicker/macro designed for wayland compositors.

## Installation
Download the program from the release tab, or compile it yourself.

Then place the file in `~/.local/bin` (or path or whatever you use)

There is not yet any official support for package managers.

### Versions
There are two main distributed versions, `wayclick` and `wayclick-lite`

#### Wayclick (recommended)
This is the full version of the application, contains everything and is the recommended download.

This contains the clicker, and a UI to edit the config files alongside it.

#### Wayclick lite
This version contains the minimum amount of things require to autoclick.

Hence things like a settings UI are excluded. This version is both way lighter (memory) and smaller file size, however it does leave config to only `settings.json`

## Usage
To run the program, open the file.
- If running the `lite` mode, the help dialogue will appear
- If running the `full` mode, the UI will appear for every setting.

Additionally, the following command line options can be used
```sh
wayclick start
wayclick stop
wayclick toggle
```

### Profiles
Profile management is in it's basic format currently, hence can only be ran in certain situations and might be very buggy.

To use a profile (that exists), add the profile name after the command. `stop` is the only command that doesn't take a profile.

### Recommendation
I **HIGHLY** recommend you to set up a global shortcut to run the command `wayclick stop` at least, or `wayclick toggle`. 

Wayland and global shortcuts are in a bit of a weird state currently *which i'm not fully familiar with*, hence i won't be tackling that straightaway.

## Features
- Autoclicker
- Settings UI
- Profile Management (settings file only)

### Planned
- Macro editor (in full)
- Profile Management
- Package manager usage
- Global keybinds

### Feature Flags (for rust devs)
#### Default
Default flags, for now is the same as doing `full`

#### Full
Contains everything

#### Macros (unused)
planned feature, but will be used for the macro editor mainly

#### UI
Enable the UI (using [gpui](https://gpui.rs)) in the build.
