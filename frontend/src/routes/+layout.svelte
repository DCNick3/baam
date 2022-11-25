<script>
  import '../app.css';

  import Navbar from '$lib/Navbar.svelte';
</script>

<div class="grid h-full w-full grid-cols-1 grid-rows-[auto_minmax(0,_1fr)]">
  <Navbar />
  <div class="contents">
    <slot />
  </div>
</div>
