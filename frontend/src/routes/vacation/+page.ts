import { listVacations } from '$lib/backend/vacation.svelte';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ fetch }) => {
  let vacations = await listVacations(fetch);
  return {
    vacations
  };
};
