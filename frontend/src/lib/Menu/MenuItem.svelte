<script lang="ts">
  import BaamIcon from '$lib/BaamIcon.svelte';
  export let text: string;
  export let icon: string;
  export let href: string;
</script>

<a
  class="hover-underline mr-2 ml-10 mt-4 w-fit text-3xl transition-colors hover:text-slate-600"
  {href}
>
  <BaamIcon name={icon} size="5xl" class="mr-5" />
  {text}
</a>
