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
  students[6] = new Student('', 'n.strygin@innopolis.university');
  students[7] = new Student('', 'n.strygin@innopolis.university');
  students[8] = new Student('', 'n.strygin@innopolis.university');
  students[9] = new Student('', 'n.strygin@innopolis.university');
  students[10] = new Student('', 'n.strygin@innopolis.university');
  students[11] = new Student('', 'n.strygin@innopolis.university');
  students[12] = new Student('', 'n.strygin@innopolis.university');
  students[13] = new Student('', 'n.strygin@innopolis.university');
  students[14] = new Student('', 'n.strygin@innopolis.university');
  students[15] = new Student('', 'n.strygin@innopolis.university');
  students[16] = new Student('', 'n.strygin@innopolis.university');
  students[17] = new Student('', 'n.strygin@innopolis.university');
  students[18] = new Student('', 'n.strygin@innopolis.university');
  students[19] = new Student('', 'n.strygin@innopolis.university');
  students[20] = new Student('', 'n.strygin@innopolis.university');
  students[21] = new Student('', 'n.strygin@innopolis.university');
  students[22] = new Student('', 'n.strygin@innopolis.university');
  students[23] = new Student('', 'n.strygin@innopolis.university');

  let qr_enabled = true;
  function flipState() {
    console.log('USED');
    qr_enabled = !qr_enabled;
  }
</script>

<div class="grid lg:grid-cols-[minmax(min-content,_50em)_minmax(30em,_1fr)] h-full">
  <div class="flex flex-grow  pl-5 pt-2 pb-10">
    <SessionFeed {students} />
  </div>
  <div class="flex flex-col lg:block hidden">
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
