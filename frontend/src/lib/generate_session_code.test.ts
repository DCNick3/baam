import CryptoJS from 'crypto-js';
import SessionCodeTimer from '$lib/generate_session_code';
import { test, expect } from 'vitest';

test('test_encoder', () => {
  // eslint-disable-next-line @typescript-eslint/no-empty-function
  const sess_time = new SessionCodeTimer('YNxExINfvxmC0q6g', 12, new Date(), 1000, () => {});
  sess_time.counter = 4;

  let encoded_code = sess_time.generateSessionCode(
    sess_time.sess_id,
    sess_time.counter,
    sess_time.secret
  );

  // reverse the urlsafe replacings
  encoded_code = encoded_code.replace('_', '/');
  encoded_code = encoded_code.replace('-', '+');

  const code = CryptoJS.enc.Base64.parse(encoded_code);

  expect(code.toString(CryptoJS.enc.Hex)).toSatisfy((x: string) => x.startsWith('308975b4'));
}, 1000);

test('test_encoder_wrong_counter', () => {
  // eslint-disable-next-line @typescript-eslint/no-empty-function
  const sess_time = new SessionCodeTimer('YNxExINfvxmC0q6g', 12, new Date(), 1000, () => {});
  sess_time.counter = 5;

  let encoded_code = sess_time.generateSessionCode(
    sess_time.sess_id,
    sess_time.counter,
    sess_time.secret
  );

  // reverse the urlsafe replacings
  encoded_code = encoded_code.replace('_', '/');
  encoded_code = encoded_code.replace('-', '+');

  const code = CryptoJS.enc.Base64.parse(encoded_code);

  expect(code.toString(CryptoJS.enc.Hex)).toSatisfy((x: string) => !x.startsWith('308975b4'));
}, 1000);
