# WayClicker

An autoclicker/macro designed for wayland compositors.

## Support Platforms
- [X] Wayland*

### Notes
I've only tested this on `Fedora 43, KDE Plasma 6.7.3, Wayland`, aka my setup. In practice however, most of this code might run on any system. It doesn't mean i'll support those systems, use something like [OpAutoClicker](https://www.opautoclicker.com/)
instead.

## Usage
There are 2 modes to this program, CLI-based and UI-based.

### Commands
Opens up the UI version
```sh
$ wayclick
```
Run the CLI
```sh
$ wayclick start # Starts the autoclicker
$ wayclick stop # stops the autoclicker
$ wayclick toggle # Toggles the autoclicker
```

I **Highly** recommend using the system to assign a shortcut to either the `stop` or `toggle` commands. Due to being on wayland, listening to keyboard inputs is possible but a bit iffy, hence its something i won't pioritise.

## Tests
There are no tests, just because i'm not fully sure on how to automate testing a tool such as this.
