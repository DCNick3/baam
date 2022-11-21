<script lang="ts">
  import Sessionname from '$lib/SessionName.svelte';
  import StudentsList from '$lib/StudentsList.svelte';
  import OutlineButton from '$lib/OutlineButton.svelte';
  import type { Student } from '$lib/student.js';

  export let students: Array<Student>;
  export let sess_name = '';
</script>

<div class="flex flex-col flex-grow w-full">
  <div class="flex flex-row">
    <OutlineButton class="m-2 mb-4" type="Danger">Scrap</OutlineButton>
    <Sessionname bind:name={sess_name} class="flex-grow mr-2" />
  </div>
  <StudentsList class="flex-grow" bind:students />
</div>
