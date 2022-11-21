import { browser } from '$app/environment';
import type {
  ApiAttendanceMark,
  ApiAttendanceMarkRef,
  ApiDeleteSession,
  ApiEmpty,
  ApiGetSession,
  ApiLogin,
  ApiNewSession,
  ApiSession,
  ApiSessionWithMarks,
  ApiUser,
  AttendanceMark,
  Session,
  SessionWithMarks
} from './models';
// import store from './store';

type Fetch = typeof window.fetch;

export class ApiError extends Error {
  code: number;
  data: RawApiError;
  constructor(code: number, data: RawApiError) {
    super(`API error ${code}: ${data.error}`);
    this.name = 'ApiError';
    this.code = code;
    this.data = data;
  }
}

export interface RawApiError {
  error: string;
  request_id: string;
  trace_id: string;
  span_id: string;
}

class Fetcher {
  fetch: Fetch;
  baseURL: string;

  constructor(fetch_?: Fetch, baseURL?: string) {
    this.fetch = fetch_ ?? fetch;
    this.baseURL = baseURL ?? '/';
  }

  request<D, T>(method: string, url: string, data: D): Promise<T>;
  request<D, T, R>(method: string, url: string, data: D, map: (data: T) => R): Promise<R>;
  request<D, T, R>(method: string, url: string, data: D, map?: (data: T) => R): Promise<T | R>;

  async request<D, T, R>(
    method: string,
    url: string,
    data: D,
    map?: (data: T) => R
  ): Promise<T | R> {
    const customHeaders: { [name: string]: string } = {};

    let encoded_data;
    if (data) {
      encoded_data = JSON.stringify(data);
      customHeaders['content-type'] = 'application/json';
    }

    url = url.replace(/^(?!.*\/\/)\/?/, this.baseURL + '/');

    const response = await this.fetch(url, {
      method: (method || 'get').toUpperCase(),
      body: encoded_data,
      headers: customHeaders,
      credentials: 'include'
    });
    const text = await response.text();

    const result_data = JSON.parse(text);

    if (response.status >= 200 && response.status < 300) {
      if (map) {
        return map(result_data);
      } else {
        return result_data;
      }
    } else {
      throw new ApiError(response.status, result_data);
    }
  }

  get<T>(url: string): Promise<T>;
  get<T, R>(url: string, map: (data: T) => R): Promise<R>;
  get<T, R>(url: string, map?: (data: T) => R): Promise<T | R> {
    return this.request('get', url, null, map);
  }

  post<D, T>(url: string, data: D): Promise<T>;
  post<D, T, R>(url: string, data: D, map: (data: T) => R): Promise<R>;
  post<D, T, R>(url: string, data: D, map?: (data: T) => R): Promise<T | R> {
    return this.request('post', url, data, map);
  }

  put<D, T>(url: string, data: D): Promise<T>;
  put<D, T, R>(url: string, data: D, map: (data: T) => R): Promise<R>;
  put<D, T, R>(url: string, data: D, map?: (data: T) => R): Promise<T | R> {
    return this.request('put', url, data, map);
  }

  delete<T>(url: string): Promise<T>;
  delete<T, R>(url: string, map: (data: T) => R): Promise<R>;
  delete<T, R>(url: string, map?: (data: T) => R): Promise<T | R> {
    return this.request('delete', url, map);
  }

  patch<D, T>(url: string, data: D): Promise<T>;
  patch<D, T, R>(url: string, data: D, map: (data: T) => R): Promise<R>;
  patch<D, T, R>(url: string, data: D, map?: (data: T) => R): Promise<T | R> {
    return this.request('patch', url, data, map);
  }
}

export function showError(error: unknown) {
  // TODO: report to the tracing service
  if (!browser) return;
  if (error instanceof ApiError) {
    // the backend returned an error
    alert(error.message);
  } else if (error instanceof TypeError) {
    // probably a network error
    alert('Network error: ' + error.message);
  } else {
    alert('Unknown error while using an API: ' + JSON.stringify(error));
  }
}

function map_session(s: ApiSession): Session {
  return {
    ...s,
    start_time: new Date(s.start_time)
  };
}

function map_session_with_marks(s: ApiSessionWithMarks): SessionWithMarks {
  return {
    ...s,
    start_time: new Date(s.start_time),
    marks: s.marks.map(map_attendance_mark)
  };
}

function map_attendance_mark(m: ApiAttendanceMark): AttendanceMark {
  return {
    ...m,
    mark_time: new Date(m.mark_time)
  };
}

export function make_api(fetch: Fetch) {
  const api = new Fetcher(fetch, '/api');
  return {
    me: () => api.get<ApiUser>('/me'),
    login: (data: ApiLogin) => api.post<ApiLogin, ApiEmpty>('/login', data),
    sessions: {
      list: () => api.get<ApiSession[], Session[]>('/sessions', (data) => data.map(map_session)),
      get: ({ id }: ApiGetSession) =>
        api.get<ApiSessionWithMarks, SessionWithMarks>(`/sessions/${id}`, map_session_with_marks),
      new: (data: ApiNewSession) =>
        api.post<ApiNewSession, ApiSession, Session>('/sessions', data, map_session),
      delete: (data: ApiDeleteSession) =>
        api.delete<ApiSession, Session>(`/sessions/${data.id}`, map_session),

      add_mark: (data: ApiAttendanceMarkRef) =>
        api.put<undefined, ApiAttendanceMark, AttendanceMark>(
          `/sessions/${data.session_id}/marks/${data.username}`,
          undefined,
          map_attendance_mark
        ),
      delete_mark: (data: ApiAttendanceMarkRef) =>
        api.delete<ApiAttendanceMark, AttendanceMark>(
          `/sessions/${data.session_id}/marks/${data.username}`,
          map_attendance_mark
        )
    }
  };
}

export const api = make_api(fetch);

type Api = ReturnType<typeof make_api>;

export function load_with_api<T, R>(
  args: (param: T & { api: Api }) => Promise<R>
): (param: T & { fetch: Fetch }) => Promise<R> {
  return async (param) => {
    return args({ ...param, api: make_api(param.fetch) });
  };
}
