import {
  delete_,
  get,
  post,
  put,
  ResponseType
} from 'positron-components/backend';

export enum ApprovalState {
  Approved = 'approved',
  Rejected = 'rejected',
  Pending = 'pending'
}

export interface Vacation {
  uuid: string;
  start_date: string;
  end_date: string;
  approval: ApprovalState;
  user: string;
}

export const listVacations = async (
  fetch: typeof window.fetch = window.fetch
) => {
  let res = await get<Vacation[]>('/api/vacation', {
    fetch,
    res_type: ResponseType.Json
  });

  if (Array.isArray(res)) {
    return res;
  }
};

export interface CreateVacation {
  start_date: string;
  end_date: string;
}

export interface CreateVacationResponse {
  uuid: string;
}

export const createVacation = async (vacation: CreateVacation) => {
  let res = await post<CreateVacationResponse>('/api/vacation', {
    body: vacation,
    res_type: ResponseType.Json
  });

  if (typeof res === 'object') {
    return res;
  }
};

export const deleteVacation = async (uuid: string) => {
  return await delete_(`/api/vacation`, {
    body: { uuid }
  });
};

export const setVacationState = async (uuid: string, state: ApprovalState) => {
  return await put(`/api/vacation`, {
    body: { uuid, state }
  });
};
