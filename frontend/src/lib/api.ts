import { browser } from '$app/environment';
import axios, { type AxiosRequestConfig, type AxiosResponse } from 'axios';
import type {
  ApiAttendanceMark,
  ApiAttendanceMarkRef,
  ApiDeleteSession,
  ApiEmptyJson,
  ApiGetSession,
  ApiNewSession,
  ApiPong,
  ApiSession,
  ApiUser,
  AttendanceMark,
  Session
} from './models';
import store from './store';

const BACKEND = 'http://localhost:8080/api';

export const api = axios.create({
  // FIXME: provide prober ssr url
  baseURL: browser ? '/api' : BACKEND
});

// number of offsets to store
const OFFSET_WINDOW = 8;

type RequestDataTime = { startTime: Date };

api.interceptors.request.use(
  (config: AxiosRequestConfig<unknown>) => ({
    metadata: { startTime: new Date() } as RequestDataTime,
    ...config
  }),
  (error) => Promise.reject(error)
);

api.interceptors.response.use(
  (response) => {
    if (!response.request) return response;
    // FIXME: use the header instead of this 💩
    // const serverTime = new Date(response.headers['X-Precise-Time'] as string).getTime();
    const serverTime = new Date().getTime();
    const rtt: number = (
      (response.config as { metadata: RequestDataTime }).metadata as RequestDataTime
    ).startTime.getTime();

    const currentServerTime = serverTime + rtt / 2;
    const currentTime = new Date().getTime();
    const offset = currentServerTime - currentTime;

    store.timeOffsets.update((offsets) => {
      offsets.push(offset);

      if (offsets.length > OFFSET_WINDOW) {
        offsets.shift();
      }
      return offsets;
    });

    return response;
  },
  (error) => Promise.reject(error)
);

interface ApiError {
  error: string;
  request_id: string;
  trace_id: string;
  span_id: string;
}

export function isApiError(error: unknown): error is AxiosResponse<ApiError, unknown> {
  return (
    typeof error === 'object' &&
    error !== null &&
    'data' in error &&
    typeof (error as { data: unknown }).data === 'object' &&
    (error as { data: unknown }).data !== null &&
    ['error', 'request_id', 'trace_id', 'span_id'].every(
      (x) => x in (error as { data: Record<string, unknown> }).data
    )
  );
}

export function showError(error: unknown) {
  if (!browser) return;
  if (isApiError(error)) {
    alert(JSON.stringify(error.data));
  }
  alert(String(error));
}

function apiRequest<reqType, respType>(
  doRequest: reqType extends undefined
    ? () => Promise<AxiosResponse<respType, unknown>>
    : (data: reqType) => Promise<AxiosResponse<respType, unknown>>
): (req?: reqType) => Promise<respType> {
  return async (req) => {
    try {
      let response: AxiosResponse<respType, unknown> | null = null;
      if (req == undefined) {
        response = await (doRequest as () => Promise<AxiosResponse<respType, unknown>>)();
      } else {
        response = await doRequest(req);
      }
      return response.data;
    } catch (error) {
      if (axios.isAxiosError(error)) {
        if (error.response) {
          throw error.response as AxiosResponse<ApiError, unknown>;
        }
      }

      throw error;
    }
  };
}

export const ping = apiRequest<undefined, ApiPong>(() => api.get('/ping'));
export const error = apiRequest<undefined, never>(() => api.get('/error'));
export const login = apiRequest<ApiUser, ApiEmptyJson>((data) => api.post('/login', data));
export const me = apiRequest<undefined, ApiUser>(() => api.get('/me'));

export const sessions = {
  list: async () => {
    const caller = apiRequest<undefined, ApiSession[]>(() => api.get('/sessions'));
    const resp = await caller();
    console.log('got resp');
    return resp.map((x) => ({ ...x, start_time: new Date(x.start_time) }));
  },

  post: apiRequest<ApiNewSession, ApiEmptyJson>((data) => api.post('/sessions', data)),
  get: async (req: ApiGetSession) => {
    const caller = apiRequest<ApiGetSession, ApiSession>((data) => api.get(`/sessions/${data.id}`));
    const resp = await caller(req);
    return { ...resp, start_time: new Date(resp.start_time) } as Session;
  },
  delete: apiRequest<ApiDeleteSession, ApiSession>((data) => api.delete(`/sessions/${data.id}`)),

  add_mark: async (req: ApiAttendanceMarkRef) => {
    const caller = apiRequest<ApiAttendanceMarkRef, ApiAttendanceMark>((data) =>
      api.get(`/sessions/${data.session_id}/marks/${data.username}`)
    );
    const resp = await caller(req);
    return { ...resp, mark_time: new Date(resp.mark_time) } as AttendanceMark;
  },
  delete_mark: async (req: ApiAttendanceMarkRef) => {
    const caller = apiRequest<ApiAttendanceMarkRef, ApiAttendanceMark>((data) =>
      api.delete(`/sessions/${data.session_id}/marks/${data.username}`)
    );
    const resp = await caller(req);
    return { ...resp, mark_time: new Date(resp.mark_time) } as AttendanceMark;
  }
};
