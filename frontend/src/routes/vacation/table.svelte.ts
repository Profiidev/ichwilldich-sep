import type { ColumnDef } from '@tanstack/table-core';
import * as DataTable from '@profidev/pleiades/components/ui/data-table';
import {
  createColumn,
  createColumnHeader
} from '@profidev/pleiades/components/table/helpers.svelte';
import { Permission } from '$lib/permissions.svelte';
import Actions from './Actions.svelte';
import Status from './Status.svelte';
import { ApprovalState, type UserInfo, type Vacation } from '$lib/client';

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
  {
    ...createColumnHeader('approval', 'Status'),
    cell: ({ row }) => {
      const status = row.getValue<ApprovalState>('approval');
      return DataTable.renderComponent(Status, {
        status
      });
    }
  },
  createColumn('user', 'User'),
  createColumn('uuid', 'UUID'),
  {
    accessorKey: 'actions',
    cell: ({ row }) => {
      const disabled = !user?.permissions.includes(Permission.VACATION_MANAGE);

      return DataTable.renderComponent(Actions, {
        approve: () => approveVacation(row.original),
        deny: () => rejectVacation(row.original),
        disabled,
        pending: row.original.approval === ApprovalState.PENDING,
        remove: () => deleteVacation(row.original),
        reset: () => resetVacation(row.original)
      });
    },
    enableHiding: false,
    header: () => {}
  }
];
