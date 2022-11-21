export type ApiEmpty = Record<string, never>;

export interface ApiUser {
  username: string;
  name?: string;
}

export interface ApiLogin {
  username: string;
  name: string;
}

export interface ApiSession {
  id: number;
  title?: string;
  active: boolean;

  // ISO 8601 date string
  start_time: string;
}
export type Session = Omit<ApiSession, 'start_time'> & { start_time: Date };

export interface ApiSessionWithMarks extends ApiSession {
  marks: ApiAttendanceMark[];
}
export type SessionWithMarks = Omit<ApiSessionWithMarks, 'start_time' | 'marks'> & {
  start_time: Date;
  marks: AttendanceMark[];
};

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

  // ISO 8601 date string
  mark_time: string;
  is_manual: boolean;
}
export type AttendanceMark = Omit<ApiAttendanceMark, 'mark_time'> & { mark_time: Date };
