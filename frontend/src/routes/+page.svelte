<script lang="ts">
  import BaamIcon from '$lib/BaamIcon.svelte';
</script>

<div class="flex flex-col h-full w-full">
  <div class="flex-grow flex flex-col ">
    <div
      class="w-full
							border-b md:border-b-0 border-r-0 md:border-r
							border-slate-300
							flex flex-row justify-center"
    >
      <div
        class="p-5 flex flex-col text-slate-800
							 -ml-10 md:ml-0"
      >
        <!-- reserve some space !-->
        <div class="h-5 md:h-20" />
        <div class="menu-section">Collect attendance</div>
        <a class="menu-item" href="/AttendanceCheck">
          <BaamIcon name="new-session" size="5xl" class="mr-5" />
          New session
        </a>
        <a class="menu-item" href="/AttendanceCheck">
          <BaamIcon name="new-session" size="5xl" class="mr-5" />
          Last session
        </a>
        <a class="menu-item" href="/AttendanceSessions">
          <BaamIcon name="session-list" size="5xl" class="mr-5" />
          All sessions
        </a>
        <!-- reserve some space !-->
        <div class="h-5 mr-2 md:h-20" />
        <div class="menu-section">Check in</div>
        <a class="menu-item" href="/s">
          <BaamIcon name="check-in" size="5xl" class="mr-5" />
          Scan QR
        </a>
      </div>
    </div>
    <div class="baam-blog flex-grow text-justify ml-10 mt-10 mx-auto">
      <!--<h2 class="post-list-heading">Latest update:</h2>--><span class="post-meta"
        >Aug 9, 2021</span
      >
      <h3>
        <a
          class="post-link"
          href="https://blog.baam.duckdns.org//jekyll/update/2021/08/09/release_v_1_1_0.html"
          target="_blank"
          rel="noreferrer"
        >
          What is new?
        </a>
      </h3>
      <h2 id="authentication-update">Authentication update!</h2>
      <h3 id="changelog-no-more-need-to-constantly-enter-your-university-email-and-password">
        Changelog: No more need to constantly enter your university email and password!
      </h3>

      <p>We changed the way we handle authentication, fixing a lot of problems with it:</p>
      <ul>
        <li>no more frequent re-logins requred!</li>
        <li>
          finally fixed big cookie monster by making smaller cookies to appease him (no more <code
            class="language-plaintext highlighter-rouge">413 Entity Too Large</code
          > errors due to cookies growing too large)
        </li>
        <li>now you no longer are promted to re-login after logging out</li>
      </ul>
    </div>
  </div>
</div>

<style>
  .menu-section {
    @apply w-fit pr-2 pl-2 border-slate-300 border-b text-4xl text-slate-500;
  }
  .menu-item {
    @apply w-fit mr-2 ml-10 mt-4 text-3xl transition-colors hover-underline;
  }
  .menu-item:hover {
    @apply text-slate-600;
  }
  /* styles for the blog */
  .baam-blog .post-meta {
    font-size: 14px;
    color: #828282;
  }
  .baam-blog h3 {
    font-size: 1.75rem;
  }
  .baam-blog h2 {
    font-size: 2rem;
  }
  .baam-blog a {
    color: #0366d6;
  }
  .baam-blog ul {
    list-style: initial;
    margin: 0;
    padding-left: 2em;
  }
  .baam-blog code {
    font-size: 87.5%;
    color: #e83e8c;
    word-break: break-word;
  }
</style>
