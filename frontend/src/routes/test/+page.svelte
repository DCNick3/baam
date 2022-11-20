<script lang="ts">
  import * as api from '$lib/api';
  const text = api
    .login({ name: 'aboba', username: 'aboba' })
    .then((res) => {
      return api.me();
    })
    .then((x) => JSON.stringify(x));
</script>

<pre>
  <code>
    {#await text then res}
      {res}
    {:catch err}
      {err}
    {/await}
  </code>
</pre>
