<script>
  import '../app.css';
  export const prerender = false;
  export const ssr = false;

  import Navbar from '$lib/Navbar.svelte';
  import * as api from '$lib/api';
  import store from '$lib/store';

  (async () => {
    try {
      const me = await api.me();
      store.me.update(() => me);
    } catch (error) {
      if (api.isApiError(error) && error.status === 401) {
        store.me.update(() => undefined);
      } else {
        api.showError(error);
      }
    }
  })().catch((err) => api.showError(err));
</script>

<div class="grid grid-rows-[auto_minmax(0,_1fr)] grid-cols-1 w-full h-full">
  <Navbar />
  <div class="contents">
    <slot />
  </div>
</div>
