import z from 'zod';

export const information = z.object({
  end_date: z
    .date('Invalid date')
    .default(new Date(new Date().getTime() + 24 * 60 * 60 * 1000)),
  start_date: z.date('Invalid date').default(new Date())
});
