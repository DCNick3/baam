<script lang="ts">
  import SessionFeed from '$lib/SessionFeed.svelte';

  import { Student } from '$lib/student.js';

  const students: Array<Student> = [];
  students[0] = new Student('', 'n.strygin@innopolis.university');
  students[1] = new Student('', 'vy.sergeev@innopolis.university');
  students[2] = new Student('', 'n.strygin@innopolis.university');
  students[3] = new Student('', 'n.strygin@innopolis.university');
  students[4] = new Student('', 'n.strygin@innopolis.university');
  students[5] = new Student('', 'n.strygin@innopolis.university');
</script>

<div class="flex flex-col h-[90vh]">
  <div class="flex-grow flex flex-row">
    <div class="flex flex-grow min-w-max w-1/2 pl-5 pt-2">
      <SessionFeed {students} />
    </div>
    <div class="flex flex-col w-1/2" />
  </div>
</div>
