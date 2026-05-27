import { listVacations } from '$lib/client';
import type { PageLoad } from './$types';

export const load: PageLoad = ({ fetch }) => {
  const vacations = listVacations({
    fetch
  }).then(({ data }) => data);
  return {
    vacations
  };
};
