<script lang="ts">
  export let type: string;
  export let disabled = false;

  let button_style = '';
  switch (type) {
    case 'Primary':
      button_style = 'bg-blue-700 enabled:hover:bg-blue-800 border-blue-900';
      break;
    case 'Secondary':
      button_style = 'bg-slate-600 enabled:hover:bg-slate-700 border-slate-800';
      break;
    case 'Success':
      button_style = 'bg-green-600 enabled:hover:bg-green-700 border-green-800';
      break;
    case 'Danger':
      button_style = 'bg-red-600 enabled:hover:bg-red-700 border-red-800';
      break;
    default:
      throw new Error('NonExistentButtonStyleUsed');
  }
</script>

<button
  {disabled}
  class="{$$props.class} rounded-md w-fit text-slate-200 border-1 border-b-[1px] border-r-[2px] py-2 px-4 shadow-lg {button_style}"
>
  <slot />
</button>
