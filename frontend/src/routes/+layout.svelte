<script>
	import '../app.css';
	export const prerender = false;
	export const ssr = false;

	import Navbar from '$lib/navbar.svelte';
</script>

<Navbar />
<body>
	<slot />
</body>
