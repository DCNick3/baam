<script lang="ts">
  import Button from '$lib/Button.svelte';
  import { Session } from '$lib/session.js';
  let sessions: Array<Session> = [];
  sessions[0] = new Session(1, '27 Oct, 18:43', 'Frontend Workshop', 30);
  sessions[1] = new Session(2, '27 Oct, 18:43', 'Frontend Workshop', 35);
  sessions[2] = new Session(3, '27 Oct, 18:43', 'Frontend Workshop', 26);
  sessions[3] = new Session(4, '27 Oct, 18:43', 'Frontend Workshop', 10);
  sessions[4] = new Session(5, '27 Oct, 18:43', 'Frontend Workshop', 1);
  sessions[5] = new Session(6, '27 Oct, 18:43', 'Frontend Workshop', 6);
  sessions[6] = new Session(7, '27 Oct, 18:43', 'Frontend Workshop', 1000);
  sessions[7] = new Session(8, '27 Oct, 18:43', 'Frontend Workshop', 1001);
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

<div class="flex flex-row justify-center w-full text-left">
  <div class="flex flex-col sm:w-4/5 mt-3">
    <div class="flex flex-row border-gray-300 text-slate-800">
      <label
        class="lbl-checkbox p-[10px] pl-[15px] pr-[15px] mt-4 mb-4 pt-2 pb-3 rounded-full hover:odd:bg-neutral-50"
      >
        <input
          aria-label="Select all sessions"
          type="checkbox"
          checked={allSelected}
          on:change={check_all_boxes}
        />
      </label>
      <div class="flex flex-row">
        {#if selection.length > 0}
          <div class="p-3 min-w-max">
            <Button type="Primary">Export {selection.length} sessions</Button>
          </div>
          <div class="p-3 min-w-max pl-0">
            <Button type="Primary">Delete {selection.length} sessions</Button>
          </div>
        {:else}
          <div class="p-3 min-w-max">
            <Button disabled={true} type="Secondary">Export {selection.length} sessions</Button>
          </div>
          <div class="p-3 min-w-max pl-0">
            <Button disabled={true} type="Secondary">Delete {selection.length} sessions</Button>
          </div>
        {/if}
      </div>
    </div>
    {#each sessions as session}
      <div
        class="flex flex-row even:border-t-[1px] even:border-b-[1px] last:border-b-[1px] border-gray-300 text-slate-800"
      >
        <div class="mt-[14px]">
          <label
            class="lbl-checkbox p-[10px] pl-[15px] pr-[15px] rounded-full hover:odd:bg-neutral-50"
          >
            <input
              aria-label="Select session {session.title}"
              type="checkbox"
              bind:group={selection}
              value={session.id}
            />
          </label>
        </div>
        <span class="p-3 mt-[3px] text-md min-w-max">{session.date}</span>
        <div class="p-3 text-lg overflow-scroll min-w-[200px]">{session.title}</div>
        <div class="flex-grow" />
        <div class="p-3 mt-[7px] text-sm text-slate-500 min-w-[100px] text-right">
          {session.numberOfStudents} ppl.
        </div>
      </div>
    {/each}
  </div>
</div>
