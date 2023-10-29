<script lang="ts">
	import iconHome from '$lib/assets/svgs/icon-home.svg';
	import iconCodeBlocks from '$lib/assets/svgs/icon-code-blocks.svg';
	import iconStar from '$lib/assets/svgs/icon-star.svg';
	import iconArticle from '$lib/assets/svgs/icon-article.svg';

	import NavTab from '$lib/components/navbar/NavTab.svelte';

	let className: string = '';
	export { className as class };
</script>

<div class={`nav-tabs nav-container ${className}`}>
	<NavTab class="active nav-block" href="/" icon={iconHome} title="Home" />
	<NavTab class="nav-block" href="/projects" icon={iconCodeBlocks} title="Projects" />
	<NavTab class="nav-block" href="/anime" icon={iconStar} title="Anime" />
	<NavTab class="nav-block" href="/blog" icon={iconArticle} title="Blog" />
</div>
