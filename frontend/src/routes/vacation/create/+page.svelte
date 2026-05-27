<script lang="ts">
  import { toast } from '@profidev/pleiades/components/util/general';
  import Information from './Information.svelte';
  import { goto, invalidate } from '$app/navigation';
  import type { Stage } from '@profidev/pleiades/components/form/types';
  import MultistepForm from '@profidev/pleiades/components/form/multistep-form.svelte';
  import { createVacation } from '$lib/client';

  let stages: Stage[] = [
    {
      title: 'Create User',
      content: Information,
      data: {}
    }
  ];

  const submit = async (rawData: object) => {
    let anyData = rawData as any;
    if (anyData.start_date > anyData.end_date) {
      return {
        error: 'Start date cannot be after end date.',
        field: 'start_date'
      };
    }

    let res = await createVacation({
      body: anyData
    });

    if (res.error) {
      return { error: 'Error creating vacation.' };
    } else {
      toast.success('Vacation created successfully.');
      invalidate((url) => url.pathname.startsWith('/api/vacation'));
      setTimeout(() => {
        goto('/vacation');
      });
    }
  };
</script>

<MultistepForm {stages} onsubmit={submit} cancelHref="/vacation" />
