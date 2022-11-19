<script lang="ts">
  import { Camera } from '$lib/camera';

  const camera = new Camera();

  let loading_future = camera.open_next();
</script>

{#await loading_future}
  <h1>Opening camera...</h1>
{/await}
