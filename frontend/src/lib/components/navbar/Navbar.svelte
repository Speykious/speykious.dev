<script lang="ts">
	import '$lib/assets/scss/spey.scss';
	import '$lib/assets/scss/debug.scss';
	import '$lib/assets/scss/navbar.scss';

	import iconHome from '$lib/assets/svgs/icon-home.svg';
	import iconCodeBlocks from '$lib/assets/svgs/icon-code-blocks.svg';
	import iconStar from '$lib/assets/svgs/icon-star.svg';
	import iconArticle from '$lib/assets/svgs/icon-article.svg';

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
		<li><NavTab class="active nav-block" href="/" icon={iconHome} title="Home" /></li>
		<li><NavTab class="disabled nav-block" href="/" icon={iconCodeBlocks} title="Projects" /></li>
		<li><NavTab class="disabled nav-block" href="/" icon={iconStar} title="Anime" /></li>
		<li><NavTab class="disabled nav-block" href="/" icon={iconArticle} title="Blog" /></li>
	</ul>
</nav>
