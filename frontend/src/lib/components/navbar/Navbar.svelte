<script lang="ts">
	import '$lib/assets/scss/spey.scss';
	import '$lib/assets/scss/debug.scss';
	import '$lib/assets/scss/navbar.scss';

	import SpeykiousDev from '$lib/components/navbar/SpeykiousDev.svelte';

	import NavTabs from '$lib/components/navbar/NavTabs.svelte';
	import Menu from './Menu.svelte';

	let windowWidth: number;
	let menuOpen: boolean;
</script>

<svelte:window bind:innerWidth={windowWidth} />

<nav>
	{#if windowWidth >= 1080}
		<SpeykiousDev />
		<NavTabs />

		<div class="nav-edge right nav-block nav-container">
			<div class="todo">(LANGUAGE SECTION, TODO)</div>
		</div>
	{:else if windowWidth >= 616}
		<div class="vertical-container stretched">
			<div class="nav-container">
				<SpeykiousDev class="stretched" />
				<div class="nav-edge right nav-block nav-container">
					<div class="todo">(LANGUAGE SECTION, TODO)</div>
				</div>
			</div>
			<div class="nav-container">
				<NavTabs />
			</div>
		</div>
	{:else}
		<div class="vertical-container stretched">
			<div class="nav-container">
				<SpeykiousDev class="stretched" textLink={windowWidth >= 360} />
				<Menu bind:open={menuOpen} />
			</div>
			{#if menuOpen}
				<div class="nav-container">
					<NavTabs class="vertical-container" />
				</div>
			{/if}
		</div>
	{/if}
</nav>
