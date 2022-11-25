<script lang="ts">
  import MenuHeader from '../lib/MenuHeader.svelte';

  import MenuItem from '../lib/MenuItem.svelte';

  import BaamBlog from '$lib/BaamBlog.svelte';
</script>

<div class="flex h-full w-full flex-col overflow-y-scroll">
  <div class="flex flex-grow flex-col">
    <div class="flex w-full flex-row justify-center">
      <div class="flex flex-col lg:flex-row lg:justify-center">
        <div
          class="-ml-10 flex flex-col p-5
                  text-slate-800 md:ml-0"
        >
          <!-- reserve some space !-->
          <div class="h-5 md:h-20" />
          <MenuHeader text="Collect Attendance" />
          <MenuItem text="New session" icon="new-session" href="/sessions/new" />
          <MenuItem text="Last Session" icon="new-session" href="/sessions/last" />
          <MenuItem text="All Sessions" icon="session-list" href="/sessions" />
        </div>
        <div class="lg:w-20" />
        <div
          class="-ml-10 flex flex-col p-5
                  text-slate-800 md:ml-0"
        >
          <!-- reserve some space !-->
          <div class="lg:h-20" />
          <MenuHeader text="Check In" />
          <MenuItem text="Scan QR" icon="check-in" href="/s" />
        </div>
      </div>
    </div>
    <BaamBlog />
  </div>
</div>
