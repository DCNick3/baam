<script lang ts>
	let name = 'Untitled Attendance Session';
	let style_classes = '';
	let interrupt = false;

	import debounce from 'lodash/debounce';

	const sleep = (ms: number) => new Promise((r) => setTimeout(r, ms));

	const returnToNormalStyle = async function () {
		await sleep(1200);
		if (!interrupt) {
			style_classes = 'duration-700 shadow-none border-gray-300';
		}
	};

	const handleBorderStyle = function (state: boolean) {
		if (!state) {
			interrupt = true;
			style_classes = 'shadow-lg border-gray-300';
		} else {
			interrupt = true;
			style_classes = 'duration-150 border-green-300 shadow-lg shadow-green-400';
			interrupt = false;
			returnToNormalStyle();
		}
	};

	const sendContentToServer = debounce((e) => {
		name = e.target.value;
		handleBorderStyle(true);
	}, 1000);

	const handleInput = function (e: Event) {
		handleBorderStyle(false);
		sendContentToServer(e);
	};
</script>

<div
	class="{$$props.class} flex flex-row border-[1px] transition-all ease-out shadow-gray-400 mb-4 mt-2 {style_classes}"
>
	<div class="p-2 bg-gray-200 text-gray-800">Session name</div>
	<input
		class="pl-2 border-l-[1px] border-gray-300 text-gray-800 focus:outline-none flex-grow"
		type="text"
		on:input={handleInput}
	/>
</div>
