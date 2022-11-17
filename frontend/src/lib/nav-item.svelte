<script lang="ts">
	export let text: string;
	export let link: string;
	export let icon: string;

	import BaamIcon from '$lib/baam-icon.svelte';
</script>

<a
	href={link}
	class="m-4 ml-8 mr-0 text-base text-slate-900 hover:text-slate-600 transition-colors hover-underline"
>
	<BaamIcon name={icon} />
	<span class="hidden lg:inline-block">{text}</span>
</a>
