<script lang="ts">
  export let sess_name = '';
  // TODO: we probably want to pass session data in here
</script>

<div class="flex flex-col">
  <div class="text-xl">Export {sess_name}:</div>
  <div class="mt-4 flex flex-row">
    <button
      class="border-1 ml-0 h-fit
        w-fit
        rounded-md border-b-[1px] border-r-[2px] border-lime-800 
        bg-lime-600 py-2 px-4 text-slate-200 shadow-lg enabled:hover:bg-lime-700"
    >
      Moodle
    </button>
    <button
      class="border-1 ml-2 h-fit
        w-fit 
    rounded-md border-b-[1px] border-r-[2px] border-stone-700 
    bg-stone-500 py-2 px-4 text-slate-200 shadow-lg enabled:hover:bg-stone-600"
    >
      Raw
    </button>
    <button
      class="border-1 ml-2 h-fit
        w-fit 
        rounded-md border-b-[1px] border-r-[2px] border-stone-600 
        bg-stone-400 py-2 px-4 text-slate-200 shadow-lg enabled:hover:bg-stone-500"
    >
      Extremely Raw
    </button>
  </div>
</div>
