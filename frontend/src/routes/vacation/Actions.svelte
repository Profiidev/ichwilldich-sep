<script lang="ts">
  import Trash from '@lucide/svelte/icons/trash';
  import Check from '@lucide/svelte/icons/check';
  import X from '@lucide/svelte/icons/x';
  import RotateCcw from '@lucide/svelte/icons/rotate-ccw';
  import { Button } from 'positron-components/components/ui/button';

  interface Props {
    disabled: boolean;
    remove: () => void;
    approve: () => void;
    deny: () => void;
    reset: () => void;
    pending: boolean;
  }

  let { remove, approve, deny, disabled, pending, reset }: Props = $props();
</script>

<div class="flex w-full space-x-4">
  {#if pending}
    <Button
      size="icon"
      variant="secondary"
      {disabled}
      class="ml-auto cursor-pointer"
      onclick={deny}
    >
      <X class="text-red-600" />
    </Button>
    <Button
      size="icon"
      variant="secondary"
      {disabled}
      class="cursor-pointer"
      onclick={approve}
    >
      <Check class="text-green-600" />
    </Button>
  {:else}
    <Button
      size="icon"
      variant="secondary"
      {disabled}
      class="ml-auto cursor-pointer"
      onclick={reset}
    >
      <RotateCcw />
    </Button>
  {/if}
  <Button
    size="icon"
    class="cursor-pointer"
    variant="destructive"
    onclick={remove}
    {disabled}
  >
    <Trash />
  </Button>
</div>
