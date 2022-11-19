<script lang="ts">
  import type { Student } from '$lib/student.js';
  import Button from './Button.svelte';
  export let students: Array<Student> = [];
</script>

<div
  class="{$$props.class} flex ml-2 mr-2 flex-col border-[1px] border-gray-300 rounded-t-md rounded-b-md"
>
  <ul class="flex-grow-0 flex-shrink">
    {#each students as student, i}
      <li class="border-t-[1px] first:border-t-0 last:border-b-[1px] border-gray-300">
        <div class="flex">
          <div class="p-1 pl-3 pr-3 bg-gray-200 text-gray-800">{i + 1}</div>
          <div class="p-1 pl-2 bg-white">
            <div class="text-gray-800">{student.email}</div>
          </div>
        </div>
      </li>
    {/each}
  </ul>
  <div class="flex-grow" />
  <div class="flex-grow-0 flex-shrink flex flex-row border-t-[1px] rounded-bl-md">
    <div class="p-2 bg-gray-200 text-gray-700">Add student</div>
    <input class="pl-2 focus:outline-none text-gray-800 flex-grow" type="text" />
    <Button class="rounded-t-none rounded-l-none border-r-[1px]" type="Success">Add</Button>
  </div>
</div>
