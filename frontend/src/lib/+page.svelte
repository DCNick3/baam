<script lang="ts">
	import Button from '$lib/button.svelte';
</script>

<div class="flex flex-col h-[90vh] w-full">
	<div class="flex-grow grid grid-flow-row grid-rows-[10%_25%_65%] grid-cols-1">
		<div />
		<div class="flex-grow grid grid-flow-row grid-cols-[10%_40%_40%_10%] grid-rows-3">
			<div />
			<div class="text-3xl text-center mt-5">Collect Attendance</div>
			<div class="text-3xl text-center mt-5">Check in</div>
			<div />

			<div />
			<div class="flex flex-row justify-center mt-5">
				<Button class="text-xl" type="Primary">New Session</Button>
			</div>
			<div class="flex flex-row justify-center mt-5">
				<Button class="text-xl" type="Primary">Scan QR</Button>
			</div>
			<div />

			<div />
			<div class="flex flex-row justify-center mt-5">
				<Button class="" type="Secondary">Last Session</Button>
				<Button class="ml-2 md:ml-5" type="Secondary">All sessions</Button>
			</div>
			<div />
			<div />
		</div>
		<div class="flex flex-row justify-center text-4xl text-center align-middle">BLOG SPACE</div>
	</div>
</div>
