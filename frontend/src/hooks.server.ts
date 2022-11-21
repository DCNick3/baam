import type { Handle, HandleFetch } from '@sveltejs/kit';
import * as set_cookie_parser from 'set-cookie-parser';

export const handle: Handle = async ({ event, resolve }) => {
  const response = await resolve(event);
  //   response.headers.set('x-custom-header', 'potato');

  return response;
};

const BACKEND = new URL('http://localhost:8080');

export const handleFetch: HandleFetch = async ({ event, request, fetch }) => {
  const url = new URL(request.url);
  if (url.pathname.startsWith('/api')) {
    url.hostname = BACKEND.hostname;
    url.port = BACKEND.port;
    url.protocol = BACKEND.protocol;

    // pass session cookie to backend
    const session = event.cookies.get('session');
    if (session) {
      request.headers.set('cookie', `session=${session}`);
    }

    request = new Request(url.toString(), request);
  }

  const resp = await fetch(request);

  // pass session cookie from backend to frontend
  const setCookie = resp.headers.get('set-cookie');
  if (setCookie) {
    set_cookie_parser.parse(setCookie).forEach((cookie) => {
      if (cookie.name === 'session') {
        event.cookies.set('session', cookie.value, {
          path: '/',
          maxAge: cookie.maxAge,
          expires: cookie.expires,
          httpOnly: cookie.httpOnly,
          sameSite: <'lax' | 'strict' | 'none' | undefined>cookie.sameSite,
          secure: cookie.secure
        });
      }
    });
  }

  return resp;
};
