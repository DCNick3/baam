import * as api from '$lib/api';
import { Session } from '$lib/session';

const sessions: Session[] = [];
async function getSessionList(fetch: typeof window.fetch) {
  const list = await api.sessions.list(fetch);
  while (sessions.length > 0) {
    sessions.pop();
  }
  return list
    .map(
      (session) =>
        new Session(
          session.id,
          formatSessionTime(session.start_time),
          session.title || '[Untitled Session]',
          14
        )
    )
    .map((session) => sessions.push(session));
}

const month = [
  '0 month',
  'Jan',
  'Feb',
  'Mar',
  'Apr',
  'May',
  'Jun',
  'Jul',
  'Aug',
  'Sep',
  'Oct',
  'Nov',
  'Dec'
];

function formatSessionTime(sessionTime: Date) {
  const currentDate = new Date();
  const currentYear = currentDate.getFullYear();
  if (sessionTime.getFullYear() === currentYear) {
    return `${sessionTime.getDate()} ${
      month[sessionTime.getMonth()]
    }, ${sessionTime.getHours()}:${sessionTime.getMinutes()}`;
  } else {
    return `${sessionTime.getDate()}.${sessionTime.getMonth()}.${
      month[sessionTime.getFullYear()]
    }, ${sessionTime.getHours()}:${sessionTime.getMinutes()}`;
  }
}

export async function load({ fetch }: { fetch: typeof window.fetch }) {
  await getSessionList(fetch);
  return {
    sessions
  };
}

export const ssr = true;
export const csr = true;