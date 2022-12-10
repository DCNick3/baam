import PreciseTimer from '$lib/precise_timer';
import { generateSessionCode } from './session_code_encode';

export default class SessionCodeTimer {
  t0: number;
  tx: number;
  callback: (x: string) => void;
  timer: PreciseTimer;
  counter: number;
  sess_id: number;
  secret: string;
  bytes_to_slice: number;

  /*
    secret -- base64 secret string
    sess_id -- session id number
    t0 -- adjusted for trip time unix timestamp
    tx -- number of seconds in epoch
    bytes_to_slice -- number of bytes to get from hmac
    callback -- callback to call whenever epoch changes
  */
  constructor(
    secret: string,
    sess_id: number,
    t0: Date,
    tx: number,
    callback: (x: string) => void,
    bytes_to_slice = 4
  ) {
    this.secret = secret;
    this.sess_id = sess_id;
    // remove ms and cast to int
    this.t0 = t0.getTime();
    this.tx = tx;
    this.counter = Math.floor((Date.now() - this.t0) / this.tx);
    this.bytes_to_slice = bytes_to_slice;
    this.callback = callback;
    this.timer = new PreciseTimer(() => this.getSessionCode(), this.t0, this.tx);
  }

  run() {
    this.timer.run();
  }

  getSessionCode() {
    this.counter = this.counter + 1;

    const encoded_code = generateSessionCode(
      this.sess_id,
      this.counter,
      this.secret,
      this.bytes_to_slice
    );

    this.callback(encoded_code);
  }

  stop() {
    this.timer.stop();
  }
}
