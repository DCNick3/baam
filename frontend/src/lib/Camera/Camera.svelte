<script lang="ts">
  import { onDestroy, onMount } from 'svelte';
  import {
    MediaStreamManager,
    MediaStreamManagerError,
    MediaStreamManagerErrorKind
  } from './MediaStreamManager';
  import Button from '../Button.svelte';
  import CameraVideo from './CameraVideo.svelte';
  import type { VideoState } from './VideoState';

  const camera = new MediaStreamManager();

  let video_state: VideoState;

  // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
  let loading_future: Promise<MediaStream> = undefined!;

  function open_next_camera() {
    console.log('Opening the next camera...');
    return (loading_future = camera.open_next().then((stream) => {
      console.log(`Camera opened! (stream.id = ${stream.id})`);

      return stream;
    }));
  }

  onMount(() => {
    open_next_camera();
    window.scrollTo(0, 1);
  });

  onDestroy(() => {
    console.log('Closing camera...');
    camera.close();
  });
</script>

{#await loading_future}
  <h1>Opening camera...</h1>
{:then stream}
  <!--  <h1>Camera opened: {stream.id}</h1>-->
  <CameraVideo {stream} bind:state={video_state} />
  <!--  <h2>State: {video_state}</h2>-->
{:catch error}
  {#if error instanceof MediaStreamManagerError}
    {#if error.kind === MediaStreamManagerErrorKind.NoCamera}
      <h1>No camera found</h1>
    {:else}
      <h1>Other error: {error.kind}: {error.reason}</h1>
    {/if}
  {:else}
    <h1>Unknown error: {error.message}</h1>
  {/if}
  <Button type="Primary" on:click|once={open_next_camera}>Try again</Button>
{/await}
