<script lang="ts">
</script>

<div class="flex flex-col h-full w-full">
	<div class="flex-grow flex flex-col md:flex-row">
		<div
			class="w-full md:w-fit
							border-b md:border-b-0 border-r-0 md:border-r
							border-slate-300
							flex flex-row"
		>
			<div class="p-5 flex flex-col text-slate-800">
				<!-- reserve some space !-->
				<div class="h-5 md:h-20" />
				<div class="menu-section">Collect attendance</div>
				<div class="menu-item">New session</div>
				<div class="menu-item">Last session</div>
				<div class="menu-item">All sessions</div>
				<!-- reserve some space !-->
				<div class="h-5 mr-2 md:h-20" />
				<div class="menu-section">Check in</div>
				<div class="menu-item">Scan QR</div>
			</div>
		</div>
		<div class="flex-grow p-2">BLOG SPACE</div>
	</div>
</div>

<style>
	.menu-section {
		@apply w-fit pr-2 pl-2 border-slate-300 border-b text-4xl text-slate-500;
	}
	.menu-item {
		@apply w-fit mr-2 ml-10 mt-4 text-3xl inline-block relative transition-colors;
	}
	.menu-item:hover {
		@apply text-slate-600;
	}
	/* Implement underline animation */
	.menu-item::after {
		content: '';
		@apply absolute w-full h-[1px] bottom-0 left-0 bg-slate-300 transition-transform scale-x-0 origin-bottom;
	}
	.menu-item:hover::after {
		@apply scale-x-100;
	}
</style>
