<script lang="ts">
  import BaseForm from '@profidev/pleiades/components/form/base-form.svelte';
  import FormDateInput from '@profidev/pleiades/components/form/form-date-input.svelte';
  import { information } from './schema.svelte';
  import type { StageProps } from '@profidev/pleiades/components/form/types';

  let { initialValue, onsubmit, footer, isLoading, data }: StageProps =
    $props();

  let form: BaseForm<typeof information> | undefined = $state();

  export const getValue = () => {
    return form?.getValue();
  };
</script>

<BaseForm
  schema={information}
  {onsubmit}
  {footer}
  {initialValue}
  bind:this={form}
  bind:isLoading
>
  {#snippet children({ props })}
    <FormDateInput
      {...props}
      key="start_date"
      label="Start Date"
      placeholder="Enter date"
    />
    <FormDateInput
      {...props}
      key="end_date"
      label="End Date"
      placeholder="Enter date"
    />
  {/snippet}
</BaseForm>
