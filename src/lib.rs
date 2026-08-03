#![deny(clippy::all)]

use napi_derive::napi;
use sdl2_sys as sys;

pub mod audio;
pub mod events;
pub mod window;

#[napi]
pub fn init(flags: u32) -> i32 {
  unsafe { sys::SDL_Init(flags) }
}

#[napi]
pub fn init_subsystem(flags: u32) -> i32 {
  unsafe { sys::SDL_InitSubSystem(flags) }
}

#[napi]
pub fn quit_subsystem(flags: u32) {
  unsafe { sys::SDL_QuitSubSystem(flags) }
}

#[napi]
pub fn was_init(flags: u32) -> bool {
  unsafe { sys::SDL_WasInit(flags) != 0 }
}

#[napi]
pub fn quit() {
  unsafe { sys::SDL_Quit() }
}
