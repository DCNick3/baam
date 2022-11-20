import PreciseTimer from '$lib/precise_timer';
import hmacSHA1 from 'crypto-js/hmac-sha1';
import WordArray from 'crypto-js/lib-typedarrays';

export default class SessionCodeTimer {
  t0: number;
  tx: number;
  callback: (x: string) => void;
  timer: PreciseTimer;
  counter: number;
  sess_id: number;
  secret: WordArray;
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
    t0: number,
    tx: number,
    bytes_to_slice = 4,
    callback: (x: string) => void
  ) {
    this.secret = CryptoJS.enc.Base64.parse(secret);
    this.sess_id = sess_id;
    this.t0 = t0;
    this.tx = tx;
    this.counter = Math.floor((Date.now() - t0) / tx);
    this.bytes_to_slice = bytes_to_slice;
    this.callback = callback;
    this.timer = new PreciseTimer(this.getSessionCode, this.t0, this.tx);
  }

  run() {
    this.timer.run();
  }

  encodeNumberToWordArray(x: number) {
    x = x | 0;
    const b0 = x & 0xff;
    const b1 = (x >> 8) & 0xff;
    const b2 = (x >> 16) & 0xff;
    const b3 = (x >> 24) & 0xff;
    return WordArray.create([b0, b1, b2, b3]);
  }

  getSessionCode() {
    this.counter = this.counter + 1;
    let code = hmacSHA1(this.secret, this.encodeNumberToWordArray(this.counter));
    code = WordArray.create(code.words.slice(this.bytes_to_slice));
    code = code.concat(this.encodeNumberToWordArray(this.sess_id));
    code = code.concat(this.encodeNumberToWordArray(this.counter));
    const encoded_code = code.toString(CryptoJS.enc.Base64);
    this.callback(encoded_code);
  }

  stop() {
    this.stop();
  }
}
