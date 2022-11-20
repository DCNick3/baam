<script lang="ts">
  import QRCode from 'qrcode';

  let qr_svg: string;

  QRCode.toString('I am a pony!', { errorCorrectionLevel: 'H', type: 'svg' })
    .then((url) => {
      qr_svg = url;
      console.log(url);
    })
    .catch((err) => {
      console.error(err);
    });
</script>

<div>
  {@html qr_svg}
</div>
