<script lang="ts">
	import BaamIcon from '$lib/BaamIcon.svelte';
</script>

<div class="flex flex-col h-full w-full">
	<div class="flex-grow flex flex-col md:flex-row">
		<div
			class="w-full md:w-fit
							border-b md:border-b-0 border-r-0 md:border-r
							border-slate-300
							flex flex-row justify-center"
		>
			<div
				class="p-5 flex flex-col text-slate-800
							 -ml-10 md:ml-0"
			>
				<!-- reserve some space !-->
				<div class="h-5 md:h-20" />
				<div class="menu-section">Collect attendance</div>
				<a class="menu-item" href="/AttendanceCheck">
					<BaamIcon name="new-session" size="5xl" class="mr-5" />
					New session
				</a>
				<a class="menu-item" href="/AttendanceCheck">
					<BaamIcon name="new-session" size="5xl" class="mr-5" />
					Last session
				</a>
				<a class="menu-item" href="/AttendanceSessions">
					<BaamIcon name="session-list" size="5xl" class="mr-5" />
					All sessions
				</a>
				<!-- reserve some space !-->
				<div class="h-5 mr-2 md:h-20" />
				<div class="menu-section">Check in</div>
				<a class="menu-item" href="/s">
					<BaamIcon name="check-in" size="5xl" class="mr-5" />
					Scan QR
				</a>
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
		@apply w-fit mr-2 ml-10 mt-4 text-3xl transition-colors hover-underline;
	}
	.menu-item:hover {
		@apply text-slate-600;
	}
</style>
