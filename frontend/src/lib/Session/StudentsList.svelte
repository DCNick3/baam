<script lang="ts">
  import { Student } from '$lib/API/student';
  import Button from '$lib/Button.svelte';
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
  class="{$$props.class} ml-2 mr-2 flex max-h-full min-h-[140px] min-w-[150px] flex-col rounded-t-md rounded-b-md border-[1px] border-gray-300"
>
  <ul class="flex-shrink flex-grow-0 overflow-y-scroll" bind:this={list}>
    {#each students as student, i}
      <li class="min-w-[150px] border-t-[1px] border-gray-300 first:border-t-0 last:border-b-[1px]">
        <div class="flex">
          <div class="min-w-[40px] bg-gray-200 p-1 text-center text-gray-800">
            {i + 1}
          </div>
          <div class="bg-white p-1 pl-2">
            <div class="text-gray-800">{student.email}</div>
          </div>
        </div>
      </li>
    {/each}
  </ul>
  <div class="flex-grow" />
  <div class="flex flex-shrink flex-grow-0 flex-row rounded-bl-md border-t-[1px]">
    <div class="min-w-max bg-gray-200 p-2 text-gray-700">Add student</div>
    <input
      class="min-w-[150px] flex-grow overflow-auto pl-2 text-gray-800 focus:outline-none"
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
