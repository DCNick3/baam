export interface ApiUser {
  username: string;
  name?: string;
}
export type ApiEmptyJson = Record<string, never>;
export type ApiPong = 'pong';
export interface ApiSession {
  id: number;
  title?: string;
  active: boolean;

  // iso string
  start_time: string;
}
export type Session = Omit<ApiSession, 'start_time'> & { start_time: Date };

export interface ApiNewSession {
  title?: string;
}

export interface ApiGetSession {
  id: number;
}

export type ApiDeleteSession = ApiGetSession;

export interface ApiAttendanceMarkRef {
  session_id: number;
  username: string;
}

export interface ApiAttendanceMark {
  username: string;

  // iso string
  mark_time: string;
  is_manual: boolean;
}
export type AttendanceMark = Omit<ApiAttendanceMark, 'mark_time'> & { mark_time: Date };
