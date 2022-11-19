<script lang="ts">
  import Button from '$lib/Button.svelte';
  import SessionFeed from '$lib/SessionFeed.svelte';

  import { Student } from '$lib/student.js';

  const students: Array<Student> = [];
  students[0] = new Student('', 'n.strygin@innopolis.university');
  students[1] = new Student('', 'vy.sergeev@innopolis.university');
  students[2] = new Student('', 'n.strygin@innopolis.university');
  students[3] = new Student('', 'n.strygin@innopolis.university');
  students[4] = new Student('', 'n.strygin@innopolis.university');
  students[5] = new Student('', 'n.strygin@innopolis.university');

  let qr_enabled = true;
  function flipState() {
    console.log('USED');
    qr_enabled = !qr_enabled;
  }
</script>

<div class="flex flex-col h-full">
  <div class="flex-grow flex flex-row">
    <div class="flex flex-grow min-w-max w-1/2 pl-5 pt-2 pb-10">
      <SessionFeed {students} />
    </div>
    <div class="flex flex-col w-1/2">
      {#if qr_enabled}
        <div class="w-full pt-4 pr-5">
          <Button class="w-full" type="Primary" on:click={flipState}>Finish showing QR code</Button>
        </div>
      {:else}
        <div class="w-full pt-4 pr-5">
          <Button class="w-full" type="Secondary" on:click={flipState}>Show QR code</Button>
        </div>
      {/if}
    </div>
  </div>
</div>
