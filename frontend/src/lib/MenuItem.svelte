<script lang="ts">
  import BaamIcon from '$lib/BaamIcon.svelte';
  export let text: string;
  export let icon: string;
</script>

<a
  class="w-fit mr-2 ml-10 mt-4 text-3xl transition-colors hover-underline hover:text-slate-600"
  href="/AttendanceSessions"
>
  <BaamIcon name={icon} size="5xl" class="mr-5" />
  {text}
</a>
