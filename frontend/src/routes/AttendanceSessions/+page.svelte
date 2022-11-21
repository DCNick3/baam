<script lang="ts">
  import Button from '$lib/Button.svelte';
  import type { Session } from '$lib/session';
  import Bomb from '~icons/baam/bomb';
  export let data: { sessions: Session[] };

  let sessions: Array<Session> = data.sessions;
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
</script>

<div class="flex flex-row justify-center w-full text-left overflow-y-scroll">
  <div class="flex flex-col sm:w-4/5 w-full mt-3">
    {#if sessions.length > 0}
      <div class="flex flex-row border-gray-300 text-slate-800">
        <label
          class="lbl-checkbox p-[15px] pl-[20px] pr-[20px] mt-2 mb-4 rounded-full hover:odd:bg-neutral-50"
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
            <div class="p-3 min-w-max">
              <Button type="Primary">Export</Button>
            </div>
            <div class="p-3 min-w-max pl-0">
              <Button type="Danger">Delete</Button>
            </div>
          {:else}
            <div class="p-3 min-w-max">
              <Button disabled={true} type="Secondary">Export</Button>
            </div>
            <div class="p-3 min-w-max pl-0">
              <Button disabled={true} type="Secondary">Delete</Button>
            </div>
          {/if}
        </div>
      </div>
      {#each sessions as session}
        <div
          class="even:border-t-[1px] even:border-b-[1px] last:border-b-[1px] border-gray-300 text-slate-800"
        >
          <div class="flex flex-row w-full">
            <div class="sm:mt-[14px] mt-auto mb-auto">
              <label
                class="lbl-checkbox p-[15px] pl-[20px] pr-[20px] -z-10 rounded-full hover:odd:bg-neutral-50"
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
                <span class="p-3 mt-[3px] text-md min-w-max">{session.date}</span>
                <div class="p-3 text-lg overflow-hidden min-w-[200px] sm:block hidden">
                  {session.title}
                </div>
                <div class="flex-grow" />
                <div class="p-3 mt-[7px] text-sm text-slate-500 min-w-[100px] text-right">
                  {session.numberOfStudents} ppl.
                </div>
              </div>
              <div class="pl-3 pb-3 text-lg overflow-hidden min-w-[200px] block sm:hidden">
                {session.title}
              </div>
            </div>
          </div>
        </div>
      {/each}
    {:else}
      <div class="items-center flex flex-row justify-center">
        <div class="opacity-60 w-28 ml-3">
          <Bomb focusable={false.toString()} width={70} height={70} viewBox="0 0 {80} {80}" />
        </div>
        <div class="flex flex-row h-[30%] text-center items-center">
          <div class="text-2xl">Oops, you have no sessions yet</div>
        </div>
      </div>
    {/if}
  </div>
</div>
