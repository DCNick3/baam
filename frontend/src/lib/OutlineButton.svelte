<script lang="ts">
  export let type: string;
  export let disabled = false;

  let button_style = '';
  switch (type) {
    case 'Primary':
      button_style = 'border-blue-500 text-blue-500 hover:bg-blue-500';
      break;
    case 'Secondary':
      button_style = 'border-slate-600 text-slate-600 hover:bg-slate-600';
      break;

    case 'Success':
      button_style = 'border-green-500 text-green-500 hover:bg-green-500';
      break;
    case 'Danger':
      button_style = 'border-red-500 text-red-500 hover:bg-red-500';
      break;
    default:
      throw new Error('NonExistentButtonStyleUsed');
  }
</script>

<button
  on:click
  {disabled}
  class="{$$props.class} duration-400 h-fit w-fit rounded-md border bg-opacity-0 py-2 px-4 transition ease-in-out {button_style} hover:text-slate-200"
>
  <slot />
</button>
