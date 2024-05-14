#ifndef _NodeSdl_Audio_H_
#define _NodeSdl_Audio_H_

#include "sound.h"
#include <SDL.h>
#include <napi.h>

namespace AudioSet {
class Short : public Napi::ObjectWrap<Short> {
  Audio *audio;

public:
  Short(const Napi::CallbackInfo &);

  void play(const Napi::CallbackInfo &);
  void pause(const Napi::CallbackInfo &);
  void resume(const Napi::CallbackInfo &);
  void stop(const Napi::CallbackInfo &);

  Napi::Value is_loaded(const Napi::CallbackInfo &);

  Napi::Value is_playing(const Napi::CallbackInfo &);

  void set_volume(const Napi::CallbackInfo &, const Napi::Value &);
  Napi::Value get_volume(const Napi::CallbackInfo &);

  void set_position(const Napi::CallbackInfo &);

  void destroy(const Napi::CallbackInfo &);

  static Napi::Function GetClass(Napi::Env &);
  static Napi::FunctionReference *constructor;
};

void Init(Napi::Env &, Napi::Object &);
} // namespace AudioSet

#endif // _NodeSdl_Audio_H_
