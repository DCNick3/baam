<script lang="ts">
  export let text: string;
  export let link: string;
  export let icon: string;

  import BaamIcon from './BaamIcon.svelte';
</script>

<a
  href={link}
  class="hover-underline m-4 ml-3 mr-0 text-xl text-slate-900 transition-colors hover:text-slate-600 sm:ml-20 md:ml-3 lg:ml-8 lg:text-2xl"
>
  <BaamIcon size="5xl" name={icon} />
  <span class="hidden md:inline-block">{text}</span>
</a>
