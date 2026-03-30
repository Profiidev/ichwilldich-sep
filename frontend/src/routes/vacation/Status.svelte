<script lang="ts">
  import { ApprovalState } from '$lib/backend/vacation.svelte';

  interface Props {
    status: ApprovalState;
  }

  let { status }: Props = $props();
</script>

<p
  class={status === ApprovalState.Approved
    ? 'text-green-600'
    : status === ApprovalState.Rejected
      ? 'text-red-600'
      : 'text-yellow-600'}
>
  {status}
</p>
