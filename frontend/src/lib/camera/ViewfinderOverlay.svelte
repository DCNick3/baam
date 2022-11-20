<script lang="ts">
  export let container: HTMLElement;
  export let video_size: { width: number; height: number };

  const BOX_SIZE = { width: 800, height: 800 };

  import { ResizeObserver } from '@juggle/resize-observer';

  let overlay: HTMLElement;
  let container_size: { width: number; height: number } = { width: 0, height: 0 };

  let resize_observer = new ResizeObserver(() => {
    const size = container.getBoundingClientRect();
    container_size = {
      width: size.width,
      height: size.height
    };
  });

  function update(video_size, overlay_size) {
    if (!video_size || !overlay_size) return;

    let box_size = {
      width: Math.min(BOX_SIZE.width, video_size.width),
      height: Math.min(BOX_SIZE.height, video_size.height)
    };

    const h_box = (box_size.width / video_size.width) * overlay_size.width;
    const v_box = (box_size.height / video_size.height) * overlay_size.height;
    const h_pillar = (container_size.width - h_box) / 2;
    const v_pillar = (container_size.height - v_box) / 2;
    const cols_template = `${h_pillar}fr ${h_box}fr ${h_pillar}fr`;
    const rows_template = `${v_pillar}fr ${v_box}fr ${v_pillar}fr`;
    console.log(cols_template, rows_template);
    if (overlay) {
      overlay.style.gridTemplateColumns = cols_template;
      overlay.style.gridTemplateRows = rows_template;
    }
  }

  $: update(video_size, container_size);
  $: {
    resize_observer.disconnect();
    if (container) resize_observer.observe(container);
  }
</script>

<div class="grid absolute inset-0" bind:this={overlay}>
  <!-- row 1 -->
  <div class="bg-mask" />
  <div class="bg-mask" />
  <div class="bg-mask" />
  <!-- row 2 -->
  <div class="bg-mask" />
  <div class="bg-transparent border border-white" />
  <div class="bg-mask" />
  <!-- row 3 -->
  <div class="bg-mask" />
  <div class="bg-mask" />
  <div class="bg-mask" />
  <!--  <h1>{video_size.width}x{video_size.height}</h1>-->
</div>
