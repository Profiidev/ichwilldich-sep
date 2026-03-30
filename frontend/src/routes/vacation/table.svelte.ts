import type { ColumnDef } from '@tanstack/table-core';
import * as DataTable from 'positron-components/components/ui/data-table';
import { createColumn } from 'positron-components/components/table/helpers.svelte';
import { Permission } from '$lib/permissions.svelte';
import { type UserInfo } from '$lib/backend/user.svelte';
import { ApprovalState, type Vacation } from '$lib/backend/vacation.svelte';
import Actions from './Actions.svelte';

export const columns = ({
  deleteVacation,
  approveVacation,
  rejectVacation,
  resetVacation,
  user
}: {
  deleteVacation: (user: Vacation) => void;
  approveVacation: (user: Vacation) => void;
  rejectVacation: (user: Vacation) => void;
  resetVacation: (user: Vacation) => void;
  user?: UserInfo;
}): ColumnDef<Vacation>[] => [
  createColumn('start_date', 'Start Date', (value: string) =>
    new Date(value).toLocaleString(navigator.languages || [navigator.language])
  ),
  createColumn('end_date', 'End Date', (value: string) =>
    new Date(value).toLocaleString(navigator.languages || [navigator.language])
  ),
  createColumn('approval', 'Status'),
  createColumn('user', 'User'),
  createColumn('uuid', 'UUID'),
  {
    accessorKey: 'actions',
    header: () => {},
    cell: ({ row }) => {
      let disabled = !user?.permissions.includes(Permission.VACATION_MANAGE);

      return DataTable.renderComponent(Actions, {
        disabled,
        remove: () => deleteVacation(row.original),
        approve: () => approveVacation(row.original),
        deny: () => rejectVacation(row.original),
        pending: row.original.approval === ApprovalState.Pending,
        reset: () => resetVacation(row.original)
      });
    },
    enableHiding: false
  }
];
