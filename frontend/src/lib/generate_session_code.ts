import PreciseTimer from '$lib/precise_timer';
import hmacSHA1 from 'crypto-js/hmac-sha1';
import WordArray from 'crypto-js/lib-typedarrays';
import CryptoJS from 'crypto-js';
import varint from 'varint';

function bswap(x: number): number {
  x = x | 0;
  const b0 = x & 0xff;
  const b1 = (x >> 8) & 0xff;
  const b2 = (x >> 16) & 0xff;
  const b3 = (x >> 24) & 0xff;
  return b3 | (b2 << 8) | (b1 << 16) | (b0 << 24);
}

function truncate(array: WordArray, length: number): WordArray {
  const result = WordArray.create();
  result.sigBytes = length;
  result.words = array.words.slice(0, Math.ceil(length / 4));
  return result;
}

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
    t0: Date,
    tx: number,
    callback: (x: string) => void,
    bytes_to_slice = 4
  ) {
    this.secret = CryptoJS.enc.Base64.parse(secret);
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

  encodeNumberToWordArray(x: number) {
    return WordArray.create([bswap(x)]);
  }

  efficientEncodeToWordArray(x: number) {
    const bytes = varint.encode(x);

    const words = [];
    let bbytes = [];
    for (const [i, b] of bytes.entries()) {
      bbytes.push(b);
      if (i % 4 === 3) {
        words.push(bbytes);
        bbytes = [];
      }
    }
    if (bbytes.length > 0) {
      while (bbytes.length < 4) {
        bbytes.push(0);
      }
      words.push(bbytes);
      bbytes = [];
    }
    const word_array = [];
    for (const word of words) {
      word_array.push(word[3] | (word[2] << 8) | (word[1] << 16) | (word[0] << 24));
    }
    return WordArray.create(word_array, bytes.length);
  }

  generateSessionCode(sess_id: number, counter: number, secret: WordArray): string {
    const index_encoded = this.encodeNumberToWordArray(counter);
    let code = hmacSHA1(index_encoded, secret);
    code = truncate(code, this.bytes_to_slice);
    code = code.concat(this.efficientEncodeToWordArray(sess_id));
    code = code.concat(this.efficientEncodeToWordArray(counter));
    let encoded_code = code.toString(CryptoJS.enc.Base64);
    // Make base64 urlsafe
    encoded_code = encoded_code.replace('+', '-');
    encoded_code = encoded_code.replace('/', '_');
    encoded_code = encoded_code.replace('=', '');

    return encoded_code;
  }

  getSessionCode() {
    this.counter = this.counter + 1;

    const encoded_code = this.generateSessionCode(this.sess_id, this.counter, this.secret);

    this.callback(encoded_code);
  }

  stop() {
    this.timer.stop();
  }
}
