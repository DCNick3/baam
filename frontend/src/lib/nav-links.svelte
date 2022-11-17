<script lang="ts">
	import NavItem from './nav-item.svelte';
</script>

<div class="flex flex-row w-fit">
	<NavItem link="/AttendanceCheck" text="New Session" icon="new-session" />
	<NavItem link="/AttendanceSessions" text="Collected Sessions" icon="session-list" />
	<NavItem link="/s" text="Check in" icon="check-in" />
	<!--	<NavItem link="/Feedback" text="Feedback" icon="" />-->
</div>
