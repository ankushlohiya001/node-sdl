export function init(flags: number): number
export function initSubsystem(flags: number): number
export function quitSubsystem(flags: number): number
export function wasInit(): boolean
export function quit(): void

export class Window {
  position: { x: number, y: number } | [number, number]
  size: { w: number, h: number }
  get cursor(): string
  set cursor(type: number)

  constructor(x: number, y: number, w: number, h: number, title: string, flags: number)
  clearSurface(): void
  updateSurface(pixelData: any, px: number, py: number, wid: number, hei: number): void
  render(): void
  destroy(): void
}

type CallBack = (eventType: any, windowId: number) => void

export class EventWatcher {
  setCallback(cb: CallBack): void
}

export class Audio {
  volume: number
  constructor(src: string, channel: number)
  play(loopCount: number): void
  pause(): void
  resume(): void
  stop(): void
  destroy(): void
}
