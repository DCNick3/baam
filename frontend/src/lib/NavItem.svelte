<script lang="ts">
  export let text: string;
  export let link: string;
  export let icon: string;

  import BaamIcon from './BaamIcon.svelte';
</script>

<a
  href={link}
  class="m-4 ml-3 sm:ml-20 md:ml-3 lg:ml-8 mr-0 text-xl lg:text-2xl text-slate-900 hover:text-slate-600 transition-colors hover-underline"
>
  <BaamIcon size="5xl" name={icon} />
  <span class="hidden md:inline-block">{text}</span>
</a>
