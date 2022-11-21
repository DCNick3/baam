<script lang="ts">
  export let sess_id = '';
  export let sess_name = '';
</script>

<div class="flex flex-col">
  <div class="text-xl">Export {sess_name}:</div>
  <div class="flex flex-row mt-4">
    <button
      class="bg-lime-600 enabled:hover:bg-lime-700 border-lime-800
        ml-0
        rounded-md w-fit h-fit text-slate-200 
        border-1 border-b-[1px] border-r-[2px] py-2 px-4 shadow-lg"
    >
      Moodle
    </button>
    <button
      class="bg-stone-500 enabled:hover:bg-stone-600 border-stone-700
        ml-2 
    rounded-md w-fit h-fit text-slate-200 
    border-1 border-b-[1px] border-r-[2px] py-2 px-4 shadow-lg"
    >
      Raw
    </button>
    <button
      class="bg-stone-400 enabled:hover:bg-stone-500 border-stone-600
        ml-2 
        rounded-md w-fit h-fit text-slate-200 
        border-1 border-b-[1px] border-r-[2px] py-2 px-4 shadow-lg"
    >
      Extremely Raw
    </button>
  </div>
</div>
