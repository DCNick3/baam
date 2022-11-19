<script lang="ts">
	import Sessionname from '$lib/SessionName.svelte';
	import StudentsList from '$lib/StudentsList.svelte';
	import OutlineButton from '$lib/OutlineButton.svelte';

	import { Student } from '$lib/student.js';

	const students: Array<Student> = [];
	students[0] = new Student('', 'n.strygin@innopolis.university');
	students[1] = new Student('', 'vy.sergeev@innopolis.university');
	students[2] = new Student('', 'n.strygin@innopolis.university');
	students[3] = new Student('', 'n.strygin@innopolis.university');
	students[4] = new Student('', 'n.strygin@innopolis.university');
	students[5] = new Student('', 'n.strygin@innopolis.university');
</script>

<div class="flex flex-col h-[90vh]">
	<div class="flex-grow grid grid-flow-row grid-cols-2">
		<div class="flex flex-col">
			<div class="flex flex-row">
				<OutlineButton class="m-2 mb-4" type="Danger">Scrap</OutlineButton>
				<Sessionname class="flex-grow mr-2" />
			</div>
			<StudentsList class="flex-grow" {students} />
		</div>
		<div class="flex flex-col" />
	</div>
</div>
