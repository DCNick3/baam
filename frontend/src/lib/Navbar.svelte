<script lang="ts">
  import NavLinks from './NavLinks.svelte';
  import NavLogin from './NavLogin.svelte';
</script>

<div class="flex selection:flex-row border-b shadow-md border-slate-300 z-10">
  <div class="flex flex-row w-fit">
    <a
      href="/"
      class="m-2 text-2xl lg:text-3xl text-slate-900 hover:text-slate-700 mt-4 ml-3 mr-0 lg:mr-4"
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
