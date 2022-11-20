<script lang="ts">
  import Sessionname from '$lib/SessionName.svelte';
  import StudentsList from '$lib/StudentsList.svelte';
  import OutlineButton from '$lib/OutlineButton.svelte';
  import type { Student } from '$lib/student.js';

  export let students: Array<Student>;
</script>

<div class="flex flex-col flex-grow">
  <div class="flex flex-row min-w-max">
    <OutlineButton class="m-2 mb-4" type="Danger">Scrap</OutlineButton>
    <Sessionname class="flex-grow mr-2" />
  </div>
  <StudentsList class="flex-grow" {students} />
</div>
