<script lang="ts">
	import '$lib/assets/scss/spey.scss';
	import '$lib/assets/scss/debug.scss';
	import '$lib/assets/scss/navbar.scss';

	import SpeykiousDev from '$lib/components/navbar/SpeykiousDev.svelte';
	import NavTab from '$lib/components/navbar/NavTab.svelte';

	let windowWidth: number;
</script>

<svelte:window bind:innerWidth={windowWidth} />

<nav>
	<SpeykiousDev />

	<input id="hamburger" type="checkbox" />
	<label class="menu-button-container nav-block nav-container kinda-button" for="hamburger">
		<div class="menu-button" />
	</label>

	<ul class="menu nav-container">
		<slot />
	</ul>
</nav>
