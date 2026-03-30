import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = ({ cookies, url }) => {
  let cookie = cookies.get('ichwilldich_sep_jwt');

  if (cookie && url.pathname === '/login') {
    redirect(302, '/');
  }
};
