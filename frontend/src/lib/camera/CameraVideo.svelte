<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { VideoState } from './VideoState';
  import ViewfinderOverlay from '$lib/camera/ViewfinderOverlay.svelte';
  import { ResizeObserver } from '@juggle/resize-observer';
  import type { Size } from './Size';

  export let stream: MediaStream;

  let video: HTMLVideoElement;
  let canvas: HTMLCanvasElement;
  let wrapper: HTMLElement;
  let container: HTMLElement;

  $: if (video) {
    video.srcObject = stream;
  }

  let canvasCtx: CanvasRenderingContext2D | null;

  $: if (canvas) {
    canvasCtx = canvas.getContext('2d');
  }

  let video_size: Size | null = null;
  let wrapper_size: Size | null = null;

  let resize_observer: ResizeObserver = new ResizeObserver(() => {
    if (!wrapper) return;

    let r = wrapper.getBoundingClientRect();
    wrapper_size = {
      width: r.width,
      height: r.height
    };
  });

  $: {
    resize_observer.disconnect();
    if (wrapper) {
      resize_observer.observe(wrapper);
    }
  }

  $: {
    // compute the neccessary margin for letterboxing
    // unfortunately, this is not possible with CSS alone, as the browsers are not consistent with video handling,
    // so we have to do it manually

    if (wrapper && video_size && wrapper_size) {
      // find the smallest scale factor that fits the video into the wrapper
      const cw = wrapper_size.width / video_size.width;
      const ch = wrapper_size.height / video_size.height;
      let scale = Math.min(cw, ch);

      console.log('scale', scale);

      // do not scale up
      if (scale > 1) scale = 1;
      const pad_x = wrapper_size.width - video_size.width * scale;
      const pad_y = wrapper_size.height - video_size.height * scale;
      console.log('pad', pad_x / 2, pad_y / 2);

      wrapper.style.paddingTop = wrapper.style.paddingBottom = `${pad_y / 2}px`;
      wrapper.style.paddingLeft = wrapper.style.paddingRight = `${pad_x / 2}px`;
    }
  }

  let stop = false;

  export let state: VideoState = VideoState.Initializing;

  function update() {
    // this function runs very often, it should be as fast as possible
    // ideally it should not be required at all
    // but, sadly, I didn't find a way to reliably get notified when the video size changes
    // so, we have to poll it 🙂
    if (stop) return;
    requestAnimationFrame(update);

    if (video) {
      if (video.paused && video.readyState >= video.HAVE_ENOUGH_DATA)
        // Специальное приглашение для Safari
        video.play();

      if (!video.paused && video.readyState >= video.HAVE_ENOUGH_DATA) {
        if (state != VideoState.Playing) {
          state = VideoState.Playing;
        }
      } else {
        if (state != VideoState.Initializing) {
          state = VideoState.Initializing;
        }
      }

      if (
        (!video_size ||
          video.videoWidth != video_size.width ||
          video.videoHeight != video_size.height) &&
        video.videoWidth > 0 &&
        video.videoHeight > 0
      ) {
        console.log('video size changed', video.videoWidth, video.videoHeight);
        video_size = {
          width: video.videoWidth,
          height: video.videoHeight
        };
      }
    }
  }

  onMount(() => {
    update();
  });

  onDestroy(() => {
    stop = true;
    resize_observer.disconnect();
  });
</script>

<!-- margin doesn't work for some reason, so use padding -->
<div class="w-full h-full" bind:this={wrapper}>
  <div class="relative w-fit" bind:this={container}>
    <canvas class="hidden" bind:this={canvas} />
    <!-- svelte-ignore a11y-media-has-caption -->
    <video class="w-full h-full" bind:this={video} autoplay playsinline muted />
    <ViewfinderOverlay {video_size} {container} />
  </div>
</div>
