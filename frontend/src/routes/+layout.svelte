<script>
	import '../app.css';
	export const prerender = false;
	export const ssr = false;

	import Navbar from '$lib/navbar.svelte';
</script>

<div class="flex flex-col h-full">
	<Navbar />
	<div class="flex-grow">
		<slot />
	</div>
</div>
