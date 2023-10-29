<script lang="ts">
	import '$lib/assets/scss/spey.scss';
	import '$lib/assets/scss/debug.scss';

	import speyPng from '$lib/assets/images/spey.png';
	import roundChevronLeft from '$lib/assets/svgs/round-chevron-left.svg';
	import roundChevronRight from '$lib/assets/svgs/round-chevron-right.svg';

	let className: string = '';
	export { className as class };
	
	export let textLink: boolean = true;
</script>

<div class={`spey-link nav-edge left nav-block nav-container ${className}`}>
	<figure class="smol chevronned-spey">
		<img class="smol chevron" src={roundChevronLeft} alt="left chevron" />
		<img class="smol spey-pfp" src={speyPng} alt="yours truly" />
		<img class="smol chevron" src={roundChevronRight} alt="right chevron" />
	</figure>
	{#if textLink}
		<a href="/">speykious.dev</a>
	{/if}
</div>
