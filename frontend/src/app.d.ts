// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
// and what to do when importing types
declare namespace App {
  interface Locals {
    session_cookie?: string;
  }
  // interface PageData {}
  // interface Platform {}
}

declare module 'virtual:icons/*' {
  export { SvelteComponentDev as default } from 'svelte/internal';
}

declare module '~icons/*' {
  import { SvelteComponentTyped } from 'svelte';
  export default class extends SvelteComponentTyped<svelte.JSX.IntrinsicElements['svg']> {}
}
