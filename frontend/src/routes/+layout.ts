import '../app.css';

import * as Sentry from '@sentry/svelte';
import { BrowserTracing } from '@sentry/tracing';

import * as store from '$lib/store';
import { ApiError, load_with_api } from '$lib/api';

const sentry_options = {
  dsn: import.meta.env.VITE_SENTRY_DSN,
  environment: import.meta.env.MODE,
  integrations: [new BrowserTracing({})],
  tracesSampleRate: 1.0
};
console.log('Initializing Sentry with options', sentry_options);
// setup sentry
Sentry.init(sentry_options);

export const load = load_with_api(async ({ api }) => {
  // MOCK: make sure the user is logged in
  // eslint-disable-next-line no-constant-condition
  while (true) {
    try {
      const user = await api.me();
      store.user.set(user);
      return {
        user
      };
    } catch (e) {
      // if we get 401 error try to log in
      if (e instanceof ApiError) {
        if (e.code === 401) {
          // console.log('401 error, trying to log in');
          const user = {
            name: 'Nikita',
            username: 'nikita'
          };

          await api.login(user);

          // console.log('logged in!');

          continue;

          // store.user.set(user);
          // return {
          //   user
          // };
        }
      }
      throw e;
    }
  }
});
