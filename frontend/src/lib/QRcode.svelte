<script lang="ts">
  import QRCode from 'qrcode';

  export let qr_data = '';

  let qr_svg: string;

  $: QRCode.toString(qr_data, { errorCorrectionLevel: 'L', type: 'svg' }).then((url) => {
    qr_svg = url;
  });
</script>

<div>
  {@html qr_svg}
</div>
