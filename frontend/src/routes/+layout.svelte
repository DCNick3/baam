<script>
  import '../app.css';
  export const prerender = false;
  export const ssr = false;

  import Navbar from '$lib/Navbar.svelte';
</script>

<div class="grid grid-rows-[auto_minmax(0,_1fr)] h-full">
  <Navbar />
  <div class="flex-grow">
    <slot />
  </div>
</div>
