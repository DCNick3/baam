<script lang="ts">
	import Button from '$lib/button.svelte';
	import { Session } from '$lib/session.js';
	let sessions: Array<Session> = [];
	sessions[0] = new Session(1, '27 Oct, 18:43', 'Frontend Workshop', 30);
	sessions[1] = new Session(2, '27 Oct, 18:43', 'Frontend Workshop', 35);
	sessions[2] = new Session(3, '27 Oct, 18:43', 'Frontend Workshop', 26);
	sessions[3] = new Session(4, '27 Oct, 18:43', 'Frontend Workshop', 10);
	sessions[4] = new Session(5, '27 Oct, 18:43', 'Frontend Workshop', 1);
	sessions[5] = new Session(6, '27 Oct, 18:43', 'Frontend Workshop', 6);
	sessions[6] = new Session(7, '27 Oct, 18:43', 'Frontend Workshop', 1000);
	sessions[7] = new Session(8, '27 Oct, 18:43', 'Frontend Workshop', 1001);
	let selection: Array<boolean> = [];
</script>

<div class="flex flex-row justify-center w-full text-left">
	<div class="flex flex-col w-4/5 mt-3">
		{#if selection.length > 0}
			<div
				class="grid grid-flow-row border-t-[1px] grid-cols-[5%_95%] border-gray-300 text-slate-800"
			>
				<div class="p-3" />
				<div class="flex flex-row">
					<div class="p-3"><Button type="Primary">Export {selection.length} sessions</Button></div>
					<div class="p-3 pl-0">
						<Button type="Primary">Delete {selection.length} sessions</Button>
					</div>
				</div>
			</div>
		{:else}
			<div
				class="grid grid-flow-row border-t-[1px] grid-cols-[5%_95%] border-gray-300 text-slate-800"
			>
				<div class="p-3" />
				<div class="flex flex-row">
					<div class="p-3">
						<Button disabled={true} type="Secondary">Export {selection.length} sessions</Button>
					</div>
					<div class="p-3 pl-0">
						<Button disabled={true} type="Secondary">Delete {selection.length} sessions</Button>
					</div>
				</div>
			</div>
		{/if}
		{#each sessions as session}
			<div
				class="grid grid-flow-row grid-cols-[5%_15%_72%_8%] even:border-t-[1px] even:border-b-[1px] last:border-b-[1px] border-gray-300 text-slate-800"
			>
				<div class="mt-[14px]">
					<label
						class="lbl-checkbox p-[10px] pl-[15px] pr-[15px] rounded-full hover:odd:bg-neutral-50"
					>
						<input
							aria-label="Select session {session.title}"
							type="checkbox"
							bind:group={selection}
							value={session.id}
						/>
					</label>
				</div>
				<span class="p-3 mt-[3px] text-md">{session.date}</span>
				<span class="p-3 text-lg">{session.title}</span>
				<span class="p-3 mt-[7px] text-sm text-slate-500">{session.numberOfStudents} ppl.</span>
			</div>
		{/each}
	</div>
</div>
