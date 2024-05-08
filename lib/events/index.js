const EventType = require("../types").EventType;
const WindowEvent = require("./window_event");
const MouseEvent = require("./mouse_event");
const KeyboardEvent = require("./keyboard_event");
// const DropEvent = require("./drop_event");
const EventWatcher = require("../binding").EventWatcher;

function eventHandler(eventWatcher, eventType, window) {
  switch (eventType) {
    case EventType.WINDOWEVENT:
      WindowEvent.initWindowEvents(eventWatcher, window);
      break;
    case EventType.KEYDOWN:
    case EventType.KEYUP:
      KeyboardEvent.initKeyboardEvents(eventWatcher, window);
      break;
    case EventType.MOUSEMOTION:
    case EventType.MOUSEBUTTONDOWN:
    case EventType.MOUSEBUTTONUP:
    case EventType.MOUSEWHEEL:
      MouseEvent.initMouseEvents(eventWatcher, window);
      break;
    // case EventType.DROPFILE:
    // case EventType.DROPTEXT:
    // case EventType.DROPBEGIN:
    // case EventType.DROPCOMPLETE:
    //   DropEvent.initDropEvents(eventWatcher.getWindowEvent);
    // break;
    case EventType.QUIT:
      //win.close();
      break;
  }
}

function setupEventWatcher(windowList) {
  const eventWatcher = new EventWatcher();

  eventWatcher.setCallback((eventType, winId) => {
    let window = windowList.get(winId);

    if (!window) return;

    eventHandler(eventWatcher, eventType, window);
  });
  return eventWatcher;
}

module.exports = { setupEventWatcher };
