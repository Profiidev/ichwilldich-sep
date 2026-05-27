import type { HandleFetch } from '@sveltejs/kit';
import { BACKEND_URL } from '$env/static/private';

const backendUrl = new URL(BACKEND_URL);

export const handleFetch: HandleFetch = async ({ event, request, fetch }) => {
  const url = new URL(request.url);
  if (url.pathname.startsWith('/api/')) {
    url.hostname = backendUrl.hostname;
    url.port = backendUrl.port;
    url.protocol = backendUrl.protocol;

    request = new Request(url.toString(), request);

    const cookie = event.request.headers.get('cookie');
    if (cookie) {
      request.headers.set('cookie', cookie);
    }
  }
  return fetch(request);
};
