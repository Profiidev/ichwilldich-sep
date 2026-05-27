<script lang="ts">
  import { ApprovalState } from '$lib/client';

  interface Props {
    status: ApprovalState;
  }

  let { status }: Props = $props();
</script>

<p
  class={status === ApprovalState.APPROVED
    ? 'text-green-600'
    : status === ApprovalState.REJECTED
      ? 'text-red-600'
      : 'text-yellow-600'}
>
  {status}
</p>
