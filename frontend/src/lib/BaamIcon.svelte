<script lang="ts">
  export let name: string;
  export let size = '3xl';
  export let focusable = false;

  const sizes: { [key: string]: string } = {
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

  import check_in from '~icons/baam/check-in';
  import new_session from '~icons/baam/new-session';
  import session_list from '~icons/baam/session-list';

  let icons = [
    {
      box: 32,
      name: 'check-in',
      component: check_in
    },
    {
      box: 32,
      name: 'new-session',
      component: new_session
    },
    {
      box: 32,
      name: 'session-list',
      component: session_list
    }
  ];

  let { component: Component, box } =
    icons.find((e) => e.name === name) ??
    (function () {
      throw new Error(`Icon with name ${name} was not found`);
    })();
</script>

<Component
  class={'inline ' + $$props.class}
  focusable={focusable.toString()}
  width={real_size}
  height={real_size}
  viewBox="0 0 {box} {box}"
/>
