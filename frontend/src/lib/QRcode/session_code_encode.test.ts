import CryptoJS from 'crypto-js';
import { test, expect } from 'vitest';
import { generateSessionCode } from './session_code_encode';

test('test_encoder', () => {
  const encoded_code = generateSessionCode(12, 4, 'YNxExINfvxmC0q6g', 4);
  const code = CryptoJS.enc.Base64url.parse(encoded_code);

  expect(code.toString(CryptoJS.enc.Hex)).toSatisfy((x: string) => x.startsWith('308975b4'));
}, 1000);

test('test_encoder_wrong_counter', () => {
  const encoded_code = generateSessionCode(12, 5, 'YNxExINfvxmC0q6g', 4);
  const code = CryptoJS.enc.Base64url.parse(encoded_code);

  expect(code.toString(CryptoJS.enc.Hex)).toSatisfy((x: string) => !x.startsWith('308975b4'));
}, 1000);
