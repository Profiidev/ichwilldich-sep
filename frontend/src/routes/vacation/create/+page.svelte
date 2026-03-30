<script lang="ts">
  import { toast } from 'positron-components/components/util/general';
  import type { Stage } from '$lib/components/form/types.svelte';
  import MultiStepForm from '$lib/components/form/MultiStepForm.svelte';
  import Information from './Information.svelte';
  import { createVacation } from '$lib/backend/vacation.svelte';
  import { goto, invalidate } from '$app/navigation';

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

    let res = await createVacation(anyData);

    if (typeof res === 'string') {
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

<MultiStepForm {stages} onsubmit={submit} cancelHref="/vacation" />
