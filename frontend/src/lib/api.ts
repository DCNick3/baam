import { browser } from '$app/environment';
import axios, { type Response } from 'redaxios';
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

type Fetch = typeof window.fetch;

function api(fetch: Fetch) {
  return axios.create({
    baseURL: '/api',
    withCredentials: true,
    fetch: fetch
  });
}

// number of offsets to store
const OFFSET_WINDOW = 8;

type RequestDataTime = { startTime: Date };

// api.interceptors.request.use(
//   (config: AxiosRequestConfig<unknown>) => ({
//     metadata: { startTime: new Date() } as RequestDataTime,
//     ...config
//   }),
//   (error) => Promise.reject(error)
// );
//
// api.interceptors.response.use(
//   (response) => {
//     if (!response.request) return response;
//     // FIXME: use the header instead of this 💩
//     // const serverTime = new Date(response.headers['X-Precise-Time'] as string).getTime();
//     const serverTime = new Date().getTime();
//     const rtt: number = (
//       (response.config as { metadata: RequestDataTime }).metadata as RequestDataTime
//     ).startTime.getTime();
//
//     const currentServerTime = serverTime + rtt / 2;
//     const currentTime = new Date().getTime();
//     const offset = currentServerTime - currentTime;
//
//     store.timeOffsets.update((offsets) => {
//       offsets.push(offset);
//
//       if (offsets.length > OFFSET_WINDOW) {
//         offsets.shift();
//       }
//       return offsets;
//     });
//
//     return response;
//   },
//   (error) => Promise.reject(error)
// );

interface ApiError {
  error: string;
  request_id: string;
  trace_id: string;
  span_id: string;
}

export function isApiError(error: unknown): error is Response<ApiError> {
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
  alert(JSON.stringify(error));
}

function apiRequest<reqType, respType>(
  doRequest: reqType extends undefined
    ? (fetch: Fetch) => Promise<Response<respType>>
    : (fetch: Fetch, data: reqType) => Promise<Response<respType>>
): (fetch: Fetch, req?: reqType) => Promise<respType> {
  return async (fetch, req) => {
    try {
      let response: Response<respType> | null = null;
      if (req === undefined) {
        response = await (doRequest as (fetch: Fetch) => Promise<Response<respType>>)(fetch);
      } else {
        response = await doRequest(fetch, req);
      }
      return response.data;
    } catch (error) {
      console.log('API ERROR', JSON.stringify(error));
      showError(error);
      // if (axios.isAxiosError(error)) {
      //   if (error.response) {
      //     showError(error.response);
      //     // throw error.response as AxiosResponse<ApiError, unknown>;
      //   }
      // }

      throw error;
    }
  };
}

export const ping = apiRequest<undefined, ApiPong>((f) => api(f).get('/ping'));
export const error = apiRequest<undefined, never>((f) => api(f).get('/error'));
export const login = apiRequest<ApiUser, ApiEmptyJson>((f, data) => api(f).post('/login', data));
export const me = apiRequest<undefined, ApiUser>((f) => api(f).get('/me'));

export const sessions = {
  list: async (f: Fetch) => {
    const caller = apiRequest<undefined, ApiSession[]>((f) => api(f).get('/sessions'));
    const resp = await caller(f);
    console.log('got resp');
    return resp.map((x) => ({ ...x, start_time: new Date(x.start_time) }));
  },

  post: apiRequest<ApiNewSession, ApiEmptyJson>((f, data) => api(f).post('/sessions', data)),
  get: async (f: Fetch, req: ApiGetSession) => {
    const caller = apiRequest<ApiGetSession, ApiSession>((f, data) =>
      api(f).get(`/sessions/${data.id}`)
    );
    const resp = await caller(f, req);
    return { ...resp, start_time: new Date(resp.start_time) } as Session;
  },
  delete: apiRequest<ApiDeleteSession, ApiSession>((f, data) =>
    api(f).delete(`/sessions/${data.id}`)
  ),

  add_mark: async (f: Fetch, req: ApiAttendanceMarkRef) => {
    const caller = apiRequest<ApiAttendanceMarkRef, ApiAttendanceMark>((f, data) =>
      api(f).get(`/sessions/${data.session_id}/marks/${data.username}`)
    );
    const resp = await caller(f, req);
    return { ...resp, mark_time: new Date(resp.mark_time) } as AttendanceMark;
  },
  delete_mark: async (f: Fetch, req: ApiAttendanceMarkRef) => {
    const caller = apiRequest<ApiAttendanceMarkRef, ApiAttendanceMark>((f, data) =>
      api(f).delete(`/sessions/${data.session_id}/marks/${data.username}`)
    );
    const resp = await caller(f, req);
    return { ...resp, mark_time: new Date(resp.mark_time) } as AttendanceMark;
  }
};
