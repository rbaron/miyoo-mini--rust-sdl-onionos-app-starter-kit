Barebones demo of a Rust + [SDL](https://www.libsdl.org/) app running on the [Miyoo Mini+](https://www.miyoogame.com/product/miyoo-mini-plus-retro-handheld-game-console/) with [Onion OS](https://github.com/OnionUI/Onion).

# How it works
[rust-bindgen](https://github.com/rust-lang/rust-bindgen) automatically generates Rust bindings for the C [SDL](https://www.libsdl.org/) library. This is why we need `unsafe` blocks around the SDL function calls -- it's probably good to wrap those calls into a safer, rustier API when building a real app.

The cross-compiler is a Docker-based toolchain slightly modified from [shauninman/union-miyoomini-toolchain](https://github.com/shauninman/union-miyoomini-toolchain) to include Rust bits.

Note that the Miyoo Mini+ has only built-in support for the legacy SDL1.2, which is what this demo app uses. The library is dynamically linked during the build, which means we can also run this demo wherever the SDL1.2 library and headers are available.

# On the Miyoo Mini Plus

<p align="center" style="max-width: 300px">
 <img src="https://github.com/user-attachments/assets/68cfc2a6-705b-4011-bd1d-06f85e029d98" alt="Photo of the Miyoo Mini+ running this demo app" />
</p>

## Building
```bash
# Build the cross-compile toolchain Docker image.
$ make build-image

# Cross-compile the app for the Miyoo Mini+ using the Docker toolchain.
# It also packages the compiled binary into a OnionOS App in RustSDLDemo/.
$ make package-app
```

## Installing
Copy `./RustSDLDemo` directory to `/mnt/SDCARD/App` in your Miyoo Mini+ running Onion OS.
If SSH is enabled:

```bash
$ scp -r RustSDLDemo onion@192.168.178.197:/mnt/SDCARD/App
```

# On macos

<p align="center" style="max-width: 300px">
 <img src="https://github.com/user-attachments/assets/3676fd6f-f531-4a59-b68d-3be6436d9aa1" alt="Screenshot of this demo app running on macos" />
</p>

## Pre-requisites
```bash
$ brew install sdl12-compat sdl_image sdl_ttf
```

These are legacy, deprecated libraries. You may need to use [this workaround](https://stackoverflow.com/questions/73586208/can-you-install-disabled-homebrew-packages).

## Running
```bash
$ cargo run
```
