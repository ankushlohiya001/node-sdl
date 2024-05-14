#ifndef Sound_H
#define Sound_H
#include <SDL_mixer.h>

static bool device_open = false;
class Audio {
  int channel = MIX_CHANNEL_POST;
  Mix_Chunk *chunk;

public:
  static void open_device(int frequency, int format, int channels,
                          int chunksize) {
    if (device_open)
      return;
    int res = Mix_OpenAudio(frequency, format, channels, chunksize);
    device_open = res != -1;
  }

  static void open_device_defaults() {
    Audio::open_device(MIX_DEFAULT_FREQUENCY, MIX_DEFAULT_FORMAT,
                       MIX_DEFAULT_CHANNELS, 2048);
  }

  Audio(const char *src, int channel) {
    load(src);
    this->channel = channel;
  }

  Audio(const char *src) { // to be played on current channel
    load(src);
    channel = -1;
  }

  Audio(Mix_Chunk *chunk) {
    this->chunk = chunk;
    channel = -1;
  }

  Audio(Mix_Chunk *chunk, int channel) {
    this->chunk = chunk;
    this->channel = channel;
  }

  void load(const char *src) {
    open_device_defaults();
    chunk = Mix_LoadWAV(src);
  }

  void play(int loops) { Mix_PlayChannel(channel, chunk, loops - 1); }

  void play() { Mix_PlayChannel(channel, chunk, 0); }

  void play_timed(int duration) {
    Mix_PlayChannelTimed(channel, chunk, -1, duration);
  }

  void pause() { Mix_Pause(channel); }

  void resume() { Mix_Resume(channel); }

  void stop() { Mix_HaltChannel(channel); }

  bool is_playing() { return Mix_Playing(channel); }

  int get_volume() { return Mix_Volume(channel, -1); }

  void set_volume(double volume) {
    int scaled = (volume / 100.0) * 128.0;
    Mix_Volume(channel, scaled);
  }

  void set_panning(int left, int right) {
    Mix_SetPanning(channel, left, right);
  }

  void set_position(int angle, int distance) {
    Mix_SetPosition(channel, angle, distance);
  }

  void on_finish(void (*channel_finished)(int)) {
    Mix_ChannelFinished(channel_finished);
  }

  Mix_Chunk *get_chunk() { return chunk; }

  void destroy() { Mix_FreeChunk(chunk); }
};

class Music {
  Mix_Music *music;

  Music(const char *src) { music = Mix_LoadMUS(src); }
  Music(Mix_Music *music) { this->music = music; }

  void play(int loops) { Mix_PlayMusic(music, loops); }

  void play() { Mix_PlayMusic(music, 0); }

  void pause() { Mix_PauseMusic(); }

  void resume() { Mix_ResumeMusic(); }

  void stop() { Mix_HaltMusic(); }

  int get_volume() { return Mix_Volume(MIX_CHANNEL_POST, -1); }

  void set_volume(double volume) {
    int scaled = (volume / 100.0) * 128.0;
    Mix_VolumeMusic(volume);
  }

  void set_panning(int left, int right) {
    Mix_SetPanning(MIX_CHANNEL_POST, left, right);
  }

  void set_position(int angle, int distance) {
    Mix_SetPosition(MIX_CHANNEL_POST, angle, distance);
  }

  Mix_Music *get_music() { return music; }

  void destroy() { Mix_FreeMusic(music); }
};

#endif // !Sound_H
