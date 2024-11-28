#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub mod miyoo_btns {
    use super::*;

    pub const BTN_A: SDLKey = SDLKey_SDLK_SPACE;
    pub const BTN_B: SDLKey = SDLKey_SDLK_LCTRL;
    pub const BTN_START: SDLKey = SDLKey_SDLK_RETURN;
    pub const BTN_LEFT: SDLKey = SDLKey_SDLK_LEFT;
    pub const BTN_RIGHT: SDLKey = SDLKey_SDLK_RIGHT;
    pub const BTN_UP: SDLKey = SDLKey_SDLK_UP;
    pub const BTN_DOWN: SDLKey = SDLKey_SDLK_DOWN;
}
