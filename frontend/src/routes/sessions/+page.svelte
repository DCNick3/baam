<script lang="ts">
  import Button from '$lib/Button.svelte';
  import Bomb from '~icons/baam/bomb';
  import { api } from '$lib/API/api';
  import type { PageData } from './$types';
  import { invalidate } from '$app/navigation';

  export let data: PageData;

  $: sessions = data.sessions;
  let selection: Array<number> = [];

  $: allSelected = sessions.length === selection.length;

  function check_all_boxes() {
    if (allSelected) {
      selection = [];
    } else {
      selection = [];
      sessions.forEach((ses) => {
        selection.push(ses.id);
      });
    }
  }

  async function batch_delete() {
    // iterate over selection and delete each session
    // TODO: add an API for batch deletion
    await Promise.all(
      selection.map((id) => {
        api.sessions.delete({ id });
      })
    );
    selection = [];
    await invalidate('/api/sessions');
    console.log('deleted & invalidated');
  }
</script>

<div class="flex w-full flex-row justify-center overflow-y-scroll text-left">
  <div class="mt-3 flex w-full flex-col sm:w-4/5">
    {#if sessions.length > 0}
      <div class="flex flex-row border-gray-300 text-slate-800">
        <label
          class="lbl-checkbox mt-2 mb-4 rounded-full p-[15px] pl-[20px] pr-[20px] hover:odd:bg-neutral-50"
        >
          <input
            class="scale-150"
            aria-label="Select all sessions"
            type="checkbox"
            checked={allSelected}
            on:change={check_all_boxes}
          />
        </label>
        <div class="flex flex-row">
          {#if selection.length > 0}
            <div class="min-w-max p-3">
              <Button type="Primary">Export</Button>
            </div>
            <div class="min-w-max p-3 pl-0">
              <Button type="Danger" on:click={batch_delete}>Delete</Button>
            </div>
          {:else}
            <div class="min-w-max p-3">
              <Button disabled={true} type="Secondary">Export</Button>
            </div>
            <div class="min-w-max p-3 pl-0">
              <Button disabled={true} type="Secondary">Delete</Button>
            </div>
          {/if}
        </div>
      </div>
      {#each sessions as session}
        <div
          class="border-gray-300 text-slate-800 last:border-b-[1px] even:border-t-[1px] even:border-b-[1px]"
        >
          <div class="flex w-full flex-row">
            <div class="mt-auto mb-auto sm:mt-[14px]">
              <label
                class="lbl-checkbox -z-10 rounded-full p-[15px] pl-[20px] pr-[20px] hover:odd:bg-neutral-50"
              >
                <input
                  class="scale-150"
                  aria-label="Select session {session.title}"
                  type="checkbox"
                  bind:group={selection}
                  value={session.id}
                />
              </label>
            </div>
            <div class="flex-grow">
              <div class="flex flex-row">
                <span class="text-md mt-[3px] min-w-max p-3">{session.date}</span>
                <div class="hidden min-w-[200px] overflow-hidden p-3 text-lg sm:block">
                  {session.title}
                </div>
                <div class="flex-grow" />
                <div class="mt-[7px] min-w-[100px] p-3 text-right text-sm text-slate-500">
                  {session.numberOfStudents} ppl.
                </div>
              </div>
              <div class="block min-w-[200px] overflow-hidden pl-3 pb-3 text-lg sm:hidden">
                {session.title}
              </div>
            </div>
          </div>
        </div>
      {/each}
    {:else}
      <div class="flex flex-row items-center justify-center">
        <div class="ml-3 w-28 opacity-60">
          <Bomb focusable={false.toString()} width={70} height={70} viewBox="0 0 {80} {80}" />
        </div>
        <div class="flex h-[30%] flex-row items-center text-center">
          <div class="text-2xl">Oops, you have no sessions yet</div>
        </div>
      </div>
    {/if}
  </div>
</div>
