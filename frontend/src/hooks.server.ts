import type { HandleFetch } from '@sveltejs/kit';

// export const handle: Handle = async ({ event, resolve }) => {
//   const response = await resolve(event);
// //   response.headers.set('x-custom-header', 'potato');
//
// return response;
// };

const BACKEND = new URL('http://localhost:8080');

export const handleFetch: HandleFetch = async ({ request, fetch }) => {
  const url = new URL(request.url);
  if (url.pathname.startsWith('/api')) {
    url.hostname = BACKEND.hostname;
    url.port = BACKEND.port;
    url.protocol = BACKEND.protocol;

    request = new Request(url.toString(), request);
  }

  return fetch(request);
};
