const sdl = require("./binding");
const InitFlag = require("./types").InitFlag;

const Window = require("./window");

const Status = {
  Loaded: 0x10,
  Idle: 0x11,
  Started: 0x12,
  Stopped: 0x13,
};

class App {
  static status = Status.Loaded;

  static mainLoop(eventWatcher, delayMs = 17) {
    if (App.status == Status.Started) {
      console.log("->Main Loop already active!!");
    } else if (App.status != Status.Idle) {
      App.initSDL();
    }
    App.status = Status.Started;
    (function loop() {
      if (App.status == Status.Stopped) return;
      eventWatcher.pollEvent();
      setTimeout(loop, delayMs);
    })();
  }

  static initSDL() {
    if (sdl.init(InitFlag.VIDEO) !== 0) {
      console.log("->Unable to initalise SDL");
      App.status = Status.Stopped;
      App.exit();
      return;
    } else {
      App.status = Status.Idle;
      console.log("->Success of initalised SDL");
    }

    // handle interrupt signal ( Ctrl + C )
    // destroy all window, then exits
    process.on("SIGINT", () => {
      for (let [, win] in Window.list) {
        win.close();
      }
    });
  }

  static exit() {
    if (App.status == Status.Stopped) {
      console.log("->Unable, already exited!!");
    } else if (Window.list.size == 0) {
      App.status = Status.Stopped;
      sdl.quit();
      console.log("->Success to exit SDL");
    }
  }

  static createWindow(opts) {
    return Window.create(App, opts);
  }
}

module.exports = App;
