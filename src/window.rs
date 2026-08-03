use napi::bindgen_prelude::*;
use napi_derive::napi;
use sdl2_sys as sys;
use std::ffi::{CStr, CString};

#[napi]
pub struct Window {
  ref_ptr: *mut sys::SDL_Window,
}

// NAPI-RS requires types sent to JS to be Send. Since JS is single-threaded,
// we wrap the raw SDL_Window pointer safely.
unsafe impl Send for Window {}

#[napi]
impl Window {
  #[napi(constructor)]
  pub fn new(x: i32, y: i32, w: i32, h: i32, title: String, flags: u32) -> Result<Self> {
    let c_title = CString::new(title)?;
    let window = unsafe { sys::SDL_CreateWindow(c_title.as_ptr(), x, y, w, h, flags) };

    if window.is_null() {
      return Err(Error::new(
        Status::GenericFailure,
        "Failed to create window",
      ));
    }
    Ok(Self { ref_ptr: window })
  }

  #[napi(getter)]
  pub fn get_id(&self) -> u32 {
    unsafe { sys::SDL_GetWindowID(self.ref_ptr) }
  }

  #[napi(getter)]
  pub fn get_title(&self) -> String {
    unsafe {
      let title = sys::SDL_GetWindowTitle(self.ref_ptr);
      CStr::from_ptr(title).to_string_lossy().into_owned()
    }
  }

  #[napi(setter)]
  pub fn set_title(&mut self, title: String) -> Result<()> {
    let c_title = CString::new(title)?;
    unsafe { sys::SDL_SetWindowTitle(self.ref_ptr, c_title.as_ptr()) };
    Ok(())
  }

  #[napi(getter)]
  pub fn get_size(&self) -> Result<serde_json::Value> {
    let mut w = 0;
    let mut h = 0;
    unsafe { sys::SDL_GetWindowSize(self.ref_ptr, &mut w, &mut h) };
    Ok(serde_json::json!({ "w": w, "h": h }))
  }

  #[napi(setter)]
  pub fn set_size(&mut self, w: i32, h: i32) {
    unsafe { sys::SDL_SetWindowSize(self.ref_ptr, w, h) };
  }

  #[napi]
  pub fn show(&self) {
    unsafe { sys::SDL_ShowWindow(self.ref_ptr) };
  }

  #[napi]
  pub fn hide(&self) {
    unsafe { sys::SDL_HideWindow(self.ref_ptr) };
  }

  #[napi]
  pub fn maximize(&self) {
    unsafe { sys::SDL_MaximizeWindow(self.ref_ptr) };
  }

  #[napi]
  pub fn clear_surface(&self) {
    unsafe {
      let mut w = 0;
      let mut h = 0;
      sys::SDL_GetWindowSize(self.ref_ptr, &mut w, &mut h);

      let rect = sys::SDL_Rect { x: 0, y: 0, w, h };
      let surf = sys::SDL_GetWindowSurface(self.ref_ptr);
      sys::SDL_FillRect(surf, &rect, 0x000000);
    }
  }

  #[napi]
  pub fn render(&self) {
    unsafe { sys::SDL_UpdateWindowSurface(self.ref_ptr) };
  }

  #[napi]
  pub fn destroy(&mut self) {
    if !self.ref_ptr.is_null() {
      unsafe { sys::SDL_DestroyWindow(self.ref_ptr) };
      self.ref_ptr = std::ptr::null_mut();
    }
  }

  #[napi]
  pub fn is_destroyed(&self) -> bool {
    self.ref_ptr.is_null()
  }
}
