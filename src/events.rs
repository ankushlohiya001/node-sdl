use napi::{Env, JsFunction, bindgen_prelude::*};
use napi_derive::napi;
use sdl2_sys as sys;

#[napi]
pub struct EventWatcher {
    eve: sys::SDL_Event,
    cb_ref: Option<ThreadsafeFunction<(u32, u32), ErrorStrategy::Fatal>>,
}

#[napi]
impl EventWatcher {
    #[napi(constructor)]
    pub fn new() -> Self {
        Self {
            eve: unsafe { std::mem::zeroed() },
            cb_ref: None,
        }
    }

    #[napi]
    pub fn set_callback(&mut self, cb: JsFunction) -> Result<()> {
        let tsfn = cb.create_threadsafe_function(0, |ctx| {
            let (event_type, window_id): (u32, u32) = ctx.value;
            ctx.env.create_uint32(event_type).and_then(|t| {
                ctx.env.create_uint32(window_id).map(|w| vec![t, w])
            })
        })?;
        self.cb_ref = Some(tsfn);
        Ok(())
    }

    #[napi]
    pub fn poll_event(&mut self) -> Result<()> {
        unsafe {
            while sys::SDL_PollEvent(&mut self.eve) != 0 {
                if let Some(ref cb) = self.cb_ref {
                    // Extract common event data
                    let event_type = self.eve.type_;
                    let window_id = self.eve.window.windowID;
                    cb.call((event_type, window_id), ThreadsafeFunctionCallMode::NonBlocking);
                }
            }
        }
        Ok(())
    }
}
