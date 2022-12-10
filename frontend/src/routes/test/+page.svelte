<script lang="ts">
  function causeError() {
    throw new Error('A wild error appears!');
  }
</script>

<div class="m-10 overflow-y-scroll">
  <h1>Welcome to the test zone!</h1>

  <button class="mt-2 bg-red-600 p-3 hover:bg-red-900" type="button" on:click={causeError}>
    Throw error
  </button>
</div>
