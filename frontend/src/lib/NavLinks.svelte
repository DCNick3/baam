<script lang="ts">
  import NavItem from './NavItem.svelte';
</script>

<div class="flex flex-row w-fit">
  <NavItem link="/sessions/new" text="New Session" icon="new-session" />
  <NavItem link="/sessions" text="Collected Sessions" icon="session-list" />
  <NavItem link="/s" text="Check in" icon="check-in" />
  <!--	<NavItem link="/Feedback" text="Feedback" icon="" />-->
</div>
