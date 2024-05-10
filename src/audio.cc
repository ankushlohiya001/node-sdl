#include "audio.h"
#include "napi.h"

AudioSet::Short::Short(const Napi::CallbackInfo &info) : ObjectWrap(info) {
  std::string src = info[0].As<Napi::String>().Utf8Value();
  if (info[1].IsUndefined()) {
    audio = new Audio(src.c_str());
  } else {
    int channels = info[1].As<Napi::Number>();
    audio = new Audio(src.c_str(), channels);
  }
}

void AudioSet::Short::play(const Napi::CallbackInfo &info) {
  if (info[0].IsUndefined()) {
    audio->play();
  } else {
    int loops = info[0].As<Napi::Number>();
    audio->play(loops);
  }
}

void AudioSet::Short::pause(const Napi::CallbackInfo &info) { audio->pause(); }

void AudioSet::Short::resume(const Napi::CallbackInfo &info) {
  audio->resume();
}

void AudioSet::Short::stop(const Napi::CallbackInfo &info) { audio->stop(); }

void AudioSet::Short::set_volume(const Napi::CallbackInfo &info,
                                 const Napi::Value &volume) {
  audio->set_volume(volume.As<Napi::Number>());
}

Napi::Value AudioSet::Short::get_volume(const Napi::CallbackInfo &info) {
  int volume = audio->get_volume();
  return Napi::Value::From(info.Env(), volume);
}

void AudioSet::Short::set_position(const Napi::CallbackInfo &info) {
  int angle = info[0].As<Napi::Number>();
  int distance = info[1].As<Napi::Number>();
  audio->set_position(angle, distance);
}

void AudioSet::Short::destroy(const Napi::CallbackInfo &info) {
  audio->destroy();
}

Napi::FunctionReference *AudioSet::Short::constructor =
    new Napi::FunctionReference();

Napi::Function AudioSet::Short::GetClass(Napi::Env &env) {
  Napi::Function func = DefineClass(
      env, "Audio",
      {
          InstanceAccessor<&Short::get_volume, &Short::set_volume>("volume"),
          InstanceMethod("play", &Short::play),
          InstanceMethod("pause", &Short::pause),
          InstanceMethod("resume", &Short::resume),
          InstanceMethod("stop", &Short::stop),
          InstanceMethod("setPosition", &Short::set_position),
          InstanceMethod("destroy", &Short::destroy),
      });

  *(Short::constructor) = Napi::Persistent(func);
  return func;
}

void AudioSet::Init(Napi::Env &env, Napi::Object &exports) {
  exports.Set(Napi::String::New(env, "Audio"), Short::GetClass(env));
}
