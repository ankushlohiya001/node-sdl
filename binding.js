import bindings from "bindings";

const binding = bindings("node-sdl.node");

export default binding;

export const init = binding.init;
export const initSubsystem = binding.initSubsystem;
export const quitSubsystem = binding.quitSubsystem;
export const wasInit = binding.wasInit;
export const quit = binding.quit;

export const Window = binding.Window;

export const EventWatcher = binding.EventWatcher;

export const Audio = binding.Audio;
