<script lang="ts">
  import { Button } from 'positron-components/components/ui/button';
  import FormDialog from 'positron-components/components/form/form-dialog.svelte';
  import Plus from '@lucide/svelte/icons/plus';
  import Table from '$lib/components/table/Table.svelte';
  import { columns } from './table.svelte';
  import { z } from 'zod';
  import { toast } from 'positron-components/components/util/general';
  import { invalidate } from '$app/navigation';
  import {
    ApprovalState,
    deleteVacation,
    setVacationState,
    type Vacation
  } from '$lib/backend/vacation.svelte';

  const { data } = $props();

  let selected: Vacation | undefined = $state();
  let deleteOpen = $state(false);
  let isLoading = $state(false);

  const deleteItemConfirm = async () => {
    if (!selected) return;

    isLoading = true;
    let ret = await deleteVacation(selected.uuid);
    isLoading = false;

    if (ret) {
      return { error: 'Failed to delete vacation' };
    } else {
      toast.success(`Vacation deleted successfully`);
      invalidate((url) => url.pathname.startsWith('/api/vacation'));
    }
  };

  const approveVacation = async (item: Vacation) => {
    isLoading = true;
    let ret = await setVacationState(item.uuid, ApprovalState.Approved);
    isLoading = false;

    if (ret) {
      toast.error(`Failed to approve vacation`);
    } else {
      toast.success(`Vacation approved successfully`);
      invalidate((url) => url.pathname.startsWith('/api/vacation'));
    }
  };

  const rejectVacation = async (item: Vacation) => {
    isLoading = true;
    let ret = await setVacationState(item.uuid, ApprovalState.Rejected);
    isLoading = false;

    if (ret) {
      toast.error(`Failed to reject vacation`);
    } else {
      toast.success(`Vacation rejected successfully`);
      invalidate((url) => url.pathname.startsWith('/api/vacation'));
    }
  };

  const resetVacation = async (item: Vacation) => {
    isLoading = true;
    let ret = await setVacationState(item.uuid, ApprovalState.Pending);
    isLoading = false;

    if (ret) {
      toast.error(`Failed to reset vacation`);
    } else {
      toast.success(`Vacation reset successfully`);
      invalidate((url) => url.pathname.startsWith('/api/vacation'));
    }
  };

  const startDeleteVacation = (item: Vacation) => {
    selected = item;
    deleteOpen = true;
  };
</script>

<div class="p-4">
  <div class="ml-7 flex items-center md:m-0">
    <h3 class="text-xl font-medium">Vacations</h3>
    <Button class="ml-auto cursor-pointer" href="/vacation/create">
      <Plus />
      Create
    </Button>
  </div>
  <Table
    data={data.vacations}
    {columns}
    class="mt-4"
    columnData={{
      user: data.user,
      deleteVacation: startDeleteVacation,
      approveVacation,
      rejectVacation,
      resetVacation
    }}
  />
</div>
<FormDialog
  title={`Delete Vacation`}
  description={`Do you really want to delete the vacation?`}
  confirm="Delete"
  confirmVariant="destructive"
  onsubmit={deleteItemConfirm}
  bind:open={deleteOpen}
  bind:isLoading
  schema={z.object({})}
/>
