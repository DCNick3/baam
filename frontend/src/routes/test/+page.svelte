<script lang="ts">
  // import { api } from '$lib/api';
  import { onMount } from 'svelte';

  let text: Promise<string> | undefined;

  onMount(async () => {
    //text = api
    //  .login(fetch, { name: 'aboba', username: 'aboba' })
    //  .then(() => {
    //    return api.me(fetch);
    //  })
    //  .then((x) => JSON.stringify(x));
  });
</script>

<pre>
  <code>
    {#if text}
      {#await text then res}
        {res}
      {:catch err}
        {err}
      {/await}
    {/if}
  </code>
</pre>
