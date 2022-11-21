import * as api from '$lib/api';
import { Session } from '$lib/session';

const sessions: Session[] = [];
async function getSessionList() {
  const list = await api.sessions.list();
  return list
    .map(
      (session) =>
        new Session(
          session.id,
          formatSessionTime(session.start_time),
          session.title || '[Untitled Session]',
          -1
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

export async function load() {
  await getSessionList();
  return {
    sessions
  };
}

export const ssr = false;
export const csr = true;
