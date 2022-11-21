<script lang="ts">
  import Button from '../../../lib/Button.svelte';
  import QRcode from '../../../lib/QRcode.svelte';
  import SessionFeed from '../../../lib/SessionFeed.svelte';
  import SessionCodeTimer from '../../../lib/generate_session_code.js';
  import { Student } from '../../../lib/student.js';
  import { onDestroy } from 'svelte';

  // import Swiper core and required modules
  import { Navigation } from 'swiper';

  import { Swiper, SwiperSlide } from 'swiper/svelte';

  // Import Swiper styles
  import 'swiper/css';
  import 'swiper/css/navigation';
  import 'swiper/css/pagination';
  import 'swiper/css/scrollbar';
  import Export from '$lib/Export.svelte';

  const students: Array<Student> = [];
  students[0] = new Student('', 'n.strygin@innopolis.university');
  students[1] = new Student('', 'vy.sergeev@innopolis.university');
  students[2] = new Student('', 'n.strygin@innopolis.university');
  students[3] = new Student('', 'n.strygin@innopolis.university');
  students[4] = new Student('', 'n.strygin@innopolis.university');
  students[5] = new Student('', 'n.strygin@innopolis.university');
  students[6] = new Student('', 'n.strygin@innopolis.university');
  students[7] = new Student('', 'n.strygin@innopolis.university');
  students[8] = new Student('', 'n.strygin@innopolis.university');
  students[9] = new Student('', 'n.strygin@innopolis.university');
  students[10] = new Student('', 'n.strygin@innopolis.university');
  students[11] = new Student('', 'n.strygin@innopolis.university');
  students[12] = new Student('', 'n.strygin@innopolis.university');
  students[13] = new Student('', 'n.strygin@innopolis.university');
  students[14] = new Student('', 'n.strygin@innopolis.university');
  students[15] = new Student('', 'n.strygin@innopolis.university');
  students[16] = new Student('', 'n.strygin@innopolis.university');
  students[17] = new Student('', 'n.strygin@innopolis.university');
  students[18] = new Student('', 'n.strygin@innopolis.university');
  students[19] = new Student('', 'n.strygin@innopolis.university');
  students[20] = new Student('', 'n.strygin@innopolis.university');
  students[21] = new Student('', 'n.strygin@innopolis.university');
  students[22] = new Student('', 'n.strygin@innopolis.university');
  students[23] = new Student('', 'n.strygin@innopolis.university');

  let qr_enabled = true;
  function flipState() {
    qr_enabled = !qr_enabled;
  }
  let qr_code_data = '';
  function construct_qr_data(session_code: string) {
    console.log(session_code);
    qr_code_data = 'https://baam.duckdns.com/s#' + session_code;
  }
  let sess_time = new SessionCodeTimer('YNxExINfvxmC0q6g', 12, new Date(), 1000, construct_qr_data);
  console.log('Running SessionCodeTimer');
  sess_time.run();
  onDestroy(() => sess_time.stop());

  let sess_name = 'Untitled Attendance Session 1';
</script>

<div class="swiper-container lg:hidden">
  <Swiper
    modules={[Navigation]}
    spaceBetween={50}
    slidesPerView={1}
    navigation
    cssMode={true}
    on:slideChange={() => console.log('slide change')}
  >
    <SwiperSlide>
      <div class="flex flex-col h-full">
        {#if qr_enabled}
          <div class="w-full px-5 pt-4 mb-3">
            <Button class="w-[100%]" type="Primary" on:click={flipState}
              >Finish showing QR code</Button
            >
          </div>
          <!-- !!!DO NOT ADD ANY MORE WRAPPERS. It breaks vertical QR code resizing! -->
          <div class="w-full contents mb-10">
            <QRcode qr_data={qr_code_data} />
          </div>
        {:else}
          <div class="w-full px-5 pt-4 mb-3">
            <Button class="w-[100%]" type="Secondary" on:click={flipState}>Show QR code</Button>
          </div>
          <div class="px-5">
            <Export {sess_name} />
          </div>
        {/if}
      </div>
    </SwiperSlide>
    <SwiperSlide>
      <div class="flex flex-grow px-3 sm:px-20 pt-2 pb-10 w-full h-full max-h-full overflow-hidden">
        <SessionFeed bind:sess_name {students} />
      </div>
    </SwiperSlide>
  </Swiper>
</div>

<div
  class="hidden lg:grid grid-cols-[minmax(min-content,_43em)_minmax(30em,_1fr)] h-full max-h-full "
>
  <div class="flex flex-grow  pl-5 pt-2 pb-10 pr-3 h-full max-h-full overflow-hidden">
    <SessionFeed bind:sess_name {students} />
  </div>
  <div class="flex flex-col h-100% overflow-hidden">
    {#if qr_enabled}
      <div class="w-full px-5 pt-4 mb-3">
        <Button class="w-[100%]" type="Primary" on:click={flipState}>Finish showing QR code</Button>
      </div>
      <!-- !!!DO NOT ADD ANY MORE WRAPPERS. It breaks vertical QR code resizing! -->
      <div class="w-full contents mb-10">
        <QRcode qr_data={qr_code_data} />
      </div>
    {:else}
      <div class="w-full px-5 pt-4 mb-3">
        <Button class="w-[100%]" type="Secondary" on:click={flipState}>Show QR code</Button>
      </div>
      <div class="px-5">
        <Export {sess_name} />
      </div>
    {/if}
  </div>
</div>

<style>
  .swiper-container > :global(.swiper) {
    height: 100%;
  }
</style>
