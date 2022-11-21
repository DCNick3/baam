import '../app.css';

import * as store from '$lib/store';
import { ApiError, load_with_api } from '$lib/api';

export const load = load_with_api(async ({ api }) => {
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
