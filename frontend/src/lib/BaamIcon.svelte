<script lang="ts">
  import InlineSVG from 'svelte-inline-svg';

  export let name;
  export let size = '3xl';
  export let focusable = false;

  const sizes = {
    '3xl': '1.875rem',
    '4xl': '2.5rem',
    '5xl': '3.125rem',
    '6xl': '3.75rem'
  };

  $: real_size =
    sizes[size] ??
    (function () {
      throw new Error(`Invalid size: ${size}`);
    })();

  import check_in from '$lib/assets/icons/check-in.svg';
  import new_session from '$lib/assets/icons/new-session.svg';
  import session_list from '$lib/assets/icons/session-list.svg';

  let icons = [
    {
      box: 32,
      name: 'check-in',
      src: check_in
    },
    {
      box: 32,
      name: 'new-session',
      src: new_session
    },
    {
      box: 32,
      name: 'session-list',
      src: session_list
    }
  ];
  let displayIcon = icons.find((e) => e.name === name);
</script>

<InlineSVG
  src={displayIcon.src}
  class={'inline ' + $$props.class}
  {focusable}
  width={real_size}
  height={real_size}
  viewBox="0 0 {displayIcon.box} {displayIcon.box}"
/>
