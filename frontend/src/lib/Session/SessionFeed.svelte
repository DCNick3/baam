<script lang="ts">
  import Sessionname from './SessionName.svelte';
  import StudentsList from './StudentsList.svelte';
  import OutlineButton from '$lib/OutlineButton.svelte';
  import type { Student } from '$lib/API/student';

  export let students: Array<Student>;
  export let sess_name = '';
</script>

<div class="flex w-full flex-grow flex-col">
  <div class="flex flex-row">
    <OutlineButton class="m-2 mb-4" type="Danger">Scrap</OutlineButton>
    <Sessionname bind:name={sess_name} class="mr-2 flex-grow" />
  </div>
  <StudentsList class="flex-grow" bind:students />
</div>
