import { load_with_api } from '$lib/api';
import type { PageLoad } from './$types';
import { redirect } from '@sveltejs/kit';

export const load: PageLoad = load_with_api(async ({ api, params }) => {
  if (params.id === 'new') {
    const session = await api.sessions.create({});
    throw redirect(307, `/sessions/${session.id}`);
  } else if (params.id === 'last') {
    throw new Error('Not implemented');
  } else {
    const id = parseInt(params.id);
    const session = await api.sessions.get({ id });
    return {
      session
    };
  }
});
