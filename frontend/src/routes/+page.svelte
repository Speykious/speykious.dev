<script lang="ts">
	import '$lib/assets/scss/spey.scss';
	import '$lib/assets/scss/debug.scss';

	import archBtw from '$lib/assets/svgs/icon-arch.svg';

	import Navbar from '$lib/components/navbar/Navbar.svelte';

	let speySetup: HTMLElement;

	let pageWidth: number;
	let pageHeight: number;

	function onMouseMove(e: MouseEvent) {
		const parallaxX = e.pageX / pageWidth;
		const parallaxY = e.pageY / pageHeight;

		speySetup.style.top = `${-10 * (1 - parallaxY)}%`;
		speySetup.style.left = `${-10 * (1 - parallaxX)}%`;
	}
</script>

<svelte:window
	on:mousemove={onMouseMove}
	bind:innerWidth={pageWidth}
	bind:innerHeight={pageHeight}
/>

<main class="main-layout">
	<Navbar />

	<div class="main-body centered-content">
		<figure id="spey-setup-bg" bind:this={speySetup} />
		<div class="center-panel centered-content" id="legendary-div">
			Behold, the  Centered Div™
		</div>
		<footer>
			<div class="footer-text">This website is not sponsored by the Rust Foundation</div>
			<div class="footer-text">
				<div>I use Arch btw</div>
				<img src={archBtw} alt="Arch btw" />
			</div>
		</footer>
	</div>
</main>

<style lang="scss">
	.main-body > * {
		z-index: 1;
	}

	.center-panel {
		display: flex;
		flex-grow: 1;
	}

	footer {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 10px 30px;

		.footer-text {
			display: flex;
			align-items: center;
			gap: 10px;
		}
	}

	#spey-setup-bg {
		position: fixed;
		top: -5%;
		left: -5%;
		width: 110%;
		height: 110%;
		z-index: 0;

		background-image: url('$lib/assets/images/spey-coe-setup.jpg');
		background-repeat: no-repeat;
		background-size: cover;
		background-position: center;

		filter: blur(5px) brightness(50%);
	}
</style>
