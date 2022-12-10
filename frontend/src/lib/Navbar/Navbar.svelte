<script lang="ts">
  import NavLinks from './NavLinks.svelte';
  import NavLogin from './NavLogin.svelte';
</script>

<div class="z-10 flex border-b border-slate-300 shadow-md selection:flex-row">
  <div class="flex w-fit flex-row">
    <a
      href="/"
      class="m-2 mt-4 ml-3 mr-0 text-2xl text-slate-900 hover:text-slate-700 lg:mr-4 lg:text-3xl"
    >
      baam
    </a>
    <NavLinks />
  </div>
  <div class="flex-grow" />
  <div class="-mt-2 mr-2 -ml-2 lg:ml-2">
    <NavLogin />
  </div>
</div>
