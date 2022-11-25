<script lang="ts">
</script>

<div class="baam-blog m-10 flex-grow text-justify">
  <!--<h2 class="post-list-heading">Latest update:</h2>--><span class="post-meta">Aug 9, 2021</span>
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

<style>
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
