# WayClicker

An autoclicker/macro designed for wayland compositors.

## Support Platforms
- [X] Wayland*

### Notes
I've only tested this on `Fedora 43, KDE Plasma 6.5.4, Wayland`, aka my setup. In theory it should work for any system which can run the following:
- [zed](https://zed.dev/) (for gpui)
- [enigo](https://crates.io/crates/enigo) (for clicking)

## Usage
There are 2 modes to this program, CLI-based and UI-based.

### Commands
Opens up the UI version
```sh
$ wayclick
$ wayclick UI
```
Run the CLI
```sh
$ wayclick start # Starts the autoclicker
$ wayclick stop # stops the autoclicker
$ wayclick toggle # Toggles the autoclicker
```

I **Highly** recommend using the system to assign a shortcut to either the `stop` or `toggle` commands. Due to being on wayland, listening to keyboard inputs is possible but a bit iffy, hence its something i won't pioritise.

## MSRV

`1.88` is the current Minimum Support Rust Version. Aka, if a dependency requires a MSRV of x.y.z, we will also require that or newer.

## Tests
There are no tests, just because i'm not fully sure on how to automate testing a tool such as this.
