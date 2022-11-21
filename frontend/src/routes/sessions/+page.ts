import { Session } from '$lib/session';
import { load_with_api } from '$lib/api';

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

export const load = load_with_api(async ({ api }) => {
  const list = await api.sessions.list();
  return {
    sessions: list.map(
      (session) =>
        new Session(
          session.id,
          formatSessionTime(session.start_time),
          session.title || '[Untitled Session]',
          -1
        )
    )
  };
});
