<script lang="ts">
  import { Student } from '$lib/student';
  import Button from './Button.svelte';
  import { afterUpdate } from 'svelte';
  export let students: Array<Student> = [];
  let list: HTMLElement;

  let student_email = '';

  function add_student() {
    console.log('add_student', student_email);
    students = [...students, new Student('', student_email)];
    student_email = '';
  }

  afterUpdate(() => {
    list.scrollTop = list.scrollHeight;
  });

  function on_key_down(event: KeyboardEvent) {
    if (event.key === 'Enter') {
      add_student();
    }
  }
</script>

<div
  class="{$$props.class} flex ml-2 mr-2 flex-col border-[1px] border-gray-300 rounded-t-md rounded-b-md max-h-full min-h-[140px] min-w-[150px]"
>
  <ul class="flex-grow-0 flex-shrink overflow-y-scroll" bind:this={list}>
    {#each students as student, i}
      <li class="border-t-[1px] first:border-t-0 last:border-b-[1px] border-gray-300 min-w-[150px]">
        <div class="flex">
          <div class="p-1 bg-gray-200 text-gray-800 min-w-[40px] text-center">
            {i + 1}
          </div>
          <div class="p-1 pl-2 bg-white">
            <div class="text-gray-800">{student.email}</div>
          </div>
        </div>
      </li>
    {/each}
  </ul>
  <div class="flex-grow" />
  <div class="flex-grow-0 flex-shrink flex flex-row border-t-[1px] rounded-bl-md">
    <div class="p-2 bg-gray-200 text-gray-700 min-w-max">Add student</div>
    <input
      class="pl-2 focus:outline-none text-gray-800 flex-grow min-w-[150px] overflow-auto"
      type="text"
      bind:value={student_email}
      on:keydown={on_key_down}
    />
    <Button
      class="rounded-t-none rounded-l-none border-r-[1px]"
      type="Success"
      on:click={add_student}>Add</Button
    >
  </div>
</div>

<style>
  .close {
    float: right;
    font-size: 1.5rem;
    font-weight: 700;
    line-height: 1;
    color: #000;
    text-shadow: 0 1px 0 #fff;
    opacity: 0.5;
  }
</style>
