#!/bin/sh
cd /mnt/SDCARD/App/RustSDLDemo/

if pgrep rust-sdl-demo >/dev/null; then
	killall -9 bindgen-sdl
fi

/mnt/SDCARD/App/RustSDLDemo/rust-sdl-demo 2>&1 | tee /tmp/rust-sdl-demo.log
