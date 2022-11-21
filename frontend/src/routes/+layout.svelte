<script>
  import '../app.css';

  import Navbar from '$lib/Navbar.svelte';
</script>

<div class="grid grid-rows-[auto_minmax(0,_1fr)] grid-cols-1 w-full h-full">
  <Navbar />
  <div class="contents">
    <slot />
  </div>
</div>
