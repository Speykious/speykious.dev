<script lang="ts">
	import '$lib/assets/scss/spey.scss';
	import '$lib/assets/scss/navbar.scss';

	import iconHome from '$lib/assets/svgs/icon-home.svg';
	import iconCodeBlocks from '$lib/assets/svgs/icon-code-blocks.svg';
	import iconStar from '$lib/assets/svgs/icon-star.svg';
	import iconArticle from '$lib/assets/svgs/icon-article.svg';

	import Navbar from '$lib/components/navbar/Navbar.svelte';
	import NavTab from '$lib/components/navbar/NavTab.svelte';

	let menuChecked: boolean;

	function closeMenu() {
		menuChecked = false;
	}

	import { page } from '$app/stores';
	$: {
		// Close menu when URL changes
		$page.url.pathname;
		closeMenu();
	}
</script>

<svelte:head>
	<meta property="og:url" content={`https://speykious.dev${$page.url.pathname}`} />
</svelte:head>

<main class="main-layout">
	<Navbar bind:menuChecked={menuChecked}>
		<NavTab href="/" icon={iconHome} title="Home" />
		<NavTab href="/" icon={iconCodeBlocks} title="Projects" disabled />
		<NavTab href="/anime/" icon={iconStar} title="Anime" />
		<NavTab href="/" icon={iconArticle} title="Blog" disabled />
	</Navbar>

	<div class="main-body centered-content">
		<slot />
	</div>
</main>
