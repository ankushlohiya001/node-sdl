use napi::bindgen_prelude::*;
use napi_derive::napi;
use sdl2_sys as sys;
use std::ffi::CString;

#[napi]
pub struct Audio {
  chunk: *mut sys::Mix_Chunk,
  channel: i32,
}

unsafe impl Send for Audio {}

#[napi]
impl Audio {
  #[napi(constructor)]
  pub fn new(src: String, channel: Option<i32>) -> Result<Self> {
    let c_src = CString::new(src)?;

    unsafe {
      // Open device defaults logic translated from sound.h
      if sys::Mix_OpenAudio(
        sys::MIX_DEFAULT_FREQUENCY as i32,
        sys::MIX_DEFAULT_FORMAT as u16,
        sys::MIX_DEFAULT_CHANNELS as i32,
        2048,
      ) == -1
      {
        return Err(Error::new(
          Status::GenericFailure,
          "Failed to init Mix_OpenAudio",
        ));
      }
    }

    let chunk = unsafe {
      sys::Mix_LoadWAV_RW(
        sys::SDL_RWFromFile(c_src.as_ptr(), b"rb\0".as_ptr() as *const _),
        1,
      )
    };
    if chunk.is_null() {
      return Err(Error::new(
        Status::GenericFailure,
        "Failed to load audio file",
      ));
    }

    Ok(Self {
      chunk,
      channel: channel.unwrap_or(-1),
    })
  }

  #[napi(getter)]
  pub fn is_loaded(&self) -> bool {
    !self.chunk.is_null()
  }

  #[napi(getter)]
  pub fn is_playing(&self) -> bool {
    unsafe { sys::Mix_Playing(self.channel) != 0 }
  }

  #[napi(getter)]
  pub fn get_volume(&self) -> i32 {
    unsafe { sys::Mix_Volume(self.channel, -1) }
  }

  #[napi(setter)]
  pub fn set_volume(&mut self, volume: f64) {
    let scaled = ((volume / 100.0) * 128.0) as i32;
    unsafe { sys::Mix_Volume(self.channel, scaled) };
  }

  #[napi]
  pub fn play(&self, loops: Option<i32>) {
    let loop_count = loops.map(|l| l - 1).unwrap_or(0);
    unsafe { sys::Mix_PlayChannelTimed(self.channel, self.chunk, loop_count, -1) };
  }

  #[napi]
  pub fn pause(&self) {
    unsafe { sys::Mix_Pause(self.channel) };
  }

  #[napi]
  pub fn resume(&self) {
    unsafe { sys::Mix_Resume(self.channel) };
  }

  #[napi]
  pub fn stop(&self) {
    unsafe { sys::Mix_HaltChannel(self.channel) };
  }

  #[napi]
  pub fn set_position(&self, angle: i16, distance: u8) {
    unsafe { sys::Mix_SetPosition(self.channel, angle, distance) };
  }

  #[napi]
  pub fn destroy(&mut self) {
    if !self.chunk.is_null() {
      unsafe { sys::Mix_FreeChunk(self.chunk) };
      self.chunk = std::ptr::null_mut();
    }
  }
}
