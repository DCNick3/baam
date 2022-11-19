<script lang="ts">
  import MenuHeader from '../lib/MenuHeader.svelte';

  import MenuItem from '../lib/MenuItem.svelte';

  import BaamBlog from '$lib/BaamBlog.svelte';
  import BaamIcon from '$lib/BaamIcon.svelte';
</script>

<div class="flex flex-col h-full w-full">
  <div class="flex-grow flex flex-col">
    <div class="w-full flex flex-row justify-center">
      <div class="flex flex-col lg:flex-row lg:justify-center">
        <div
          class="p-5 flex flex-col text-slate-800
                  -ml-10 md:ml-0"
        >
          <!-- reserve some space !-->
          <div class="h-5 md:h-20" />
          <MenuHeader text="Collect Attendance" />
          <MenuItem text="New session" icon="new-session" />
          <MenuItem text="Last Session" icon="new-session" />
          <MenuItem text="All Sessions" icon="session-list" />
        </div>
        <div class="lg:w-20" />
        <div
          class="p-5 flex flex-col text-slate-800
                  -ml-10 md:ml-0"
        >
          <!-- reserve some space !-->
          <div class="lg:h-20" />
          <MenuHeader text="Check In" />
          <MenuItem text="Scan QR" icon="check-in" />
        </div>
      </div>
    </div>
    <BaamBlog />
  </div>
</div>
