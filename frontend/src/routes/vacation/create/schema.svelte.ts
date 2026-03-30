import z from 'zod';

export const information = z.object({
  start_date: z.date('Invalid date').default(new Date(new Date().getTime())),
  end_date: z
    .date('Invalid date')
    .default(new Date(new Date().getTime() + 24 * 60 * 60 * 1000))
});
