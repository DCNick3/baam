export default class PreciseTimer {
  fn: () => void;
  start_time: number;
  duration: number;
  timer?: NodeJS.Timeout;

  constructor(fn: () => void, start_time: number, duration: number) {
    this.fn = fn;
    this.start_time = start_time;
    this.duration = duration;
    this.timer = undefined;
  }

  run(): void {
    let delta = (new Date().getTime() - this.start_time) / this.duration;
    this.fn();

    delta = (new Date().getTime() - this.start_time) / this.duration;
    delta -= Math.round(delta);
    this.timer = setTimeout(() => this.run(), (1 - delta) * this.duration);
  }

  stop(): void {
    clearTimeout(this.timer);
  }
}
