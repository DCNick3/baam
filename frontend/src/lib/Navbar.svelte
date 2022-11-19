<script lang="ts">
	import NavLinks from './NavLinks.svelte';
	import NavLogin from './NavLogin.svelte';
</script>

<div class="flex selection:flex-row border-b shadow-md border-slate-300 z-10">
	<div class="flex flex-row w-fit">
		<a href="/" class="m-2 text-xl text-slate-900 hover:text-slate-700 mt-4 ml-3 mr-4"> baam </a>
		<NavLinks />
	</div>
	<div class="flex-grow" />
	<NavLogin />
</div>
