const AudioCore = require("./binding").Audio;

const AudioStatus = {
  LOADED: 0x001,
  PLAYING: 0x002,
  PAUSED: 0x003,
  STOPPED: 0x004,
  ENDED: 0x005,
};

class Audio extends AudioCore {
  static create(opts) {
    return new Audio(opts);
  }

  constructor(opts) {
    super(opts.src, opts.channel);
    // this.status = super.is_loaded ? AudioStatus.LOADED : null;
    this.status = AudioStatus.LOADED;
  }

  play(loopCount) {
    if (this.status == AudioStatus.LOADED) {
      super.play(loopCount);
      this.status = AudioStatus.PLAYING;
    } else {
      throw "Audio not loaded!!";
    }
  }

  increaseVolume(factor = 5) {
    if (super.volume + factor <= 100) {
      super.volume += factor;
    }
  }

  decreaseVolume(factor = 5) {
    if (super.volume - factor >= 0) {
      super.volume -= factor;
    }
  }

  pause() {
    if (this.status == AudioStatus.PLAYING) {
      super.pause();
      this.status = AudioStatus.PAUSED;
    } else {
      throw "Audio not playing!!";
    }
  }

  resume() {
    if (this.status == AudioStatus.PAUSED) {
      super.resume();
      this.status = AudioStatus.PLAYING;
    } else if (this.status == AudioStatus.ENDED) {
      throw "Audio playback ended!!";
    }
  }

  stop() {
    if (this.status != AudioStatus.STOPPED) {
      super.stop();
      super.destroy();
      this.status = AudioStatus.STOPPED;
    }
  }
}

module.exports = Audio;
