<script lang="ts">
  export let name = 'Untitled Attendance Session';
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
      style_classes = 'shadow-lg border-gray-300 shadow-gray-400 ';
    } else {
      interrupt = true;
      style_classes = 'duration-700 shadow-green-400 border-green-300 shadow-lg ';
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
  let first_selection = true;
  const handleFocus = function (e: Event) {
    if (first_selection) {
      (e.target as HTMLInputElement).select();
      first_selection = false;
    }
  };
</script>

<div
  class="{$$props.class} {style_classes} mb-4 mt-2 flex min-w-[150px] flex-row border-[1px] transition-all ease-out"
>
  <div class="hidden min-w-max bg-gray-200 p-2 text-gray-800 sm:block">Session name</div>
  <input
    class="min-w-[150px] flex-grow overflow-auto border-gray-300 pl-2 text-gray-800 focus:outline-none sm:border-l-[1px]"
    type="text"
    on:focus={handleFocus}
    on:input={handleInput}
    bind:value={name}
  />
</div>
