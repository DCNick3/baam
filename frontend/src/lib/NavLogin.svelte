<script lang="ts">
  import OutlineButton from './OutlineButton.svelte';
  import { user } from './store';
</script>

<div class="flex flex-col w-fit">
  <p class="m-4 mr-1 mb-1 text-slate-900 min-w-max">Hello, {$user?.name ?? '???'}</p>
  <div class="m-2 mt-0 flex flex-row justify-end">
    <OutlineButton type="Secondary">Logout</OutlineButton>
  </div>
</div>
