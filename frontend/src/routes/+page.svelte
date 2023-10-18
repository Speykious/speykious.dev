<script lang="ts">
	import '$lib/assets/scss/spey.scss';
	import '$lib/assets/scss/debug.scss';
	import '$lib/assets/scss/navbar.scss';

	import speyPng from '$lib/assets/images/spey.png';
	import roundChevronLeft from '$lib/assets/svgs/round-chevron-left.svg';
	import roundChevronRight from '$lib/assets/svgs/round-chevron-right.svg';

	import iconHome from '$lib/assets/svgs/icon-home.svg';
	import iconCodeBlocks from '$lib/assets/svgs/icon-code-blocks.svg';
	import iconStar from '$lib/assets/svgs/icon-star.svg';
	import iconArticle from '$lib/assets/svgs/icon-article.svg';
</script>

<main class="main-layout">
	<nav>
		<div class="nav-edge left nav-block nav-container">
			<figure class="smol chevronned-spey">
				<img class="smol chevron" src={roundChevronLeft} alt="left chevron" />
				<img class="smol spey-pfp" src={speyPng} alt="yours truly" />
				<img class="smol chevron" src={roundChevronRight} alt="right chevron" />
			</figure>
			<a href="/">speykious.dev</a>
		</div>

		<a href="/" class="tab active nav-block centered-content">
			<img class="tab-icon" src={iconHome} alt="Home" />
			<div>Home</div>
		</a>
		<a href="/projects" class="disabled tab nav-block centered-content">
			<img class="tab-icon" src={iconCodeBlocks} alt="Projects" />
			<div>Projects</div>
		</a>
		<a href="/anime" class="disabled tab nav-block centered-content">
			<img class="tab-icon" src={iconStar} alt="Anime" />
			<div>Anime</div>
		</a>
		<a href="/blog" class="disabled tab nav-block centered-content">
			<img class="tab-icon" src={iconArticle} alt="Blog" />
			<div>Blog</div>
		</a>

		<div class="nav-edge right nav-block nav-container">
			<div class="todo">(LANGUAGE SECTION, TODO)</div>
		</div>
	</nav>

	<div class="main-body centered-content debug-3">
		<div class="todo">Warning: unresponsive design as of now. WIP</div>
		<div id="legendary-div">Behold, the legendary Centered Div™</div>
	</div>
</main>

<style lang="scss">
	.todo {
		font-size: 10px;
		opacity: 30%;
	}
</style>
