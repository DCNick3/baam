import CryptoJS from 'crypto-js';
import hmacSHA1 from 'crypto-js/hmac-sha1';
import WordArray from 'crypto-js/lib-typedarrays';
import varint from 'varint';

function truncate(array: WordArray, length: number): WordArray {
  const result = WordArray.create();
  result.sigBytes = length;
  result.words = array.words.slice(0, Math.ceil(length / 4));
  return result;
}

function bswap(x: number): number {
  x = x | 0;
  const b0 = x & 0xff;
  const b1 = (x >> 8) & 0xff;
  const b2 = (x >> 16) & 0xff;
  const b3 = (x >> 24) & 0xff;
  return b3 | (b2 << 8) | (b1 << 16) | (b0 << 24);
}

function encodeNumberToWordArray(x: number) {
  return WordArray.create([bswap(x)]);
}

function efficientEncodeToWordArray(x: number) {
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

export function generateSessionCode(
  sess_id: number,
  counter: number,
  secret_key: string,
  bytes_to_slice: number
): string {
  const secret = CryptoJS.enc.Base64.parse(secret_key);

  const index_encoded = encodeNumberToWordArray(counter);
  let code = hmacSHA1(index_encoded, secret);
  code = truncate(code, bytes_to_slice);
  code = code.concat(efficientEncodeToWordArray(sess_id));
  code = code.concat(efficientEncodeToWordArray(counter));
  const encoded_code = code.toString(CryptoJS.enc.Base64url);

  return encoded_code;
}
