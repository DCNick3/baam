import type { PlaywrightTestConfig } from '@playwright/test';

const config: PlaywrightTestConfig = {
  webServer: {
    command: 'yarn build && yarn preview',
    url: 'https://localhost:4173/',
    ignoreHTTPSErrors: true
  },
  use: {
    baseURL: 'https://localhost:4173/',
    ignoreHTTPSErrors: true
  },
  testMatch: 'tests/**/.*(test|spec).(js|ts)'
};

export default config;
