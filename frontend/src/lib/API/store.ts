import { browser } from '$app/environment';
import type { Writable } from 'svelte/store';
import { writable, get } from 'svelte/store';
import type { ApiUser } from './models';

// this allows us to store svelte store thing in local storage
// it's not used by anything yet though :P

// eslint-disable-next-line @typescript-eslint/no-unused-vars
const val = <T>(key: string, initValue: T): Writable<T> => {
  const store = writable(initValue);
  if (!browser) return store;

  const storedValueStr = localStorage.getItem(key);
  if (storedValueStr != null) store.set(JSON.parse(storedValueStr));

  store.subscribe((val: T | (null | undefined)) => {
    if (val === null || val === undefined) {
      localStorage.removeItem(key);
    } else {
      localStorage.setItem(key, JSON.stringify(val));
    }
  });

  window.addEventListener('storage', () => {
    const storedValueStr = localStorage.getItem(key);
    if (storedValueStr == null) return;

    const localValue: T = JSON.parse(storedValueStr);
    if (localValue !== get(store)) store.set(localValue);
  });

  return store;
};

export const timeOffsets = writable([0]);
export const user = writable(<ApiUser | null>null);
