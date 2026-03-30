<script lang="ts">
  import BaseForm from 'positron-components/components/form/base-form.svelte';
  import { information } from './schema.svelte';
  import type { StageProps } from '$lib/components/form/types.svelte';
  import FormDateInput from '$lib/components/form/FormDateInput.svelte';

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
