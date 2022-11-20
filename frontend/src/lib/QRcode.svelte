<script lang="ts">
  import QRCode from 'qrcode';

  export let qr_data = '';

  let qr_svg = '';

  $: QRCode.toString(qr_data, { errorCorrectionLevel: 'L', type: 'svg' }).then((url) => {
    qr_svg = url;
  });
</script>

<div class="w-full flex-grow qr-parent min-h-0">
  {@html qr_svg}
  <!-- Appease the linter by using this -->
  <svg class="hidden" />
</div>

<style>
  div.qr-parent > :global(svg) {
    width: 100%;
    height: 100%;
  }
</style>
