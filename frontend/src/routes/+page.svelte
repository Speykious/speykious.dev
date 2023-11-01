<script lang="ts">
	import '$lib/assets/scss/spey.scss';
	import '$lib/assets/scss/debug.scss';

	import archBtw from '$lib/assets/svgs/icon-arch.svg';

	import Navbar from '$lib/components/navbar/Navbar.svelte';

	let speySetup: HTMLElement;

	let pageWidth: number;
	let pageHeight: number;

	function clamp(x: number, a: number, b: number): number {
		return Math.min(b, Math.max(a, x));
	}

	function updateParallax(pageX: number, pageY: number) {
		const parallaxX = clamp(pageX / pageWidth, 0, 1);
		const parallaxY = clamp(pageY / pageHeight, 0, 1);

		speySetup.style.top = `${-10 * (1 - parallaxY)}%`;
		speySetup.style.left = `${-10 * (1 - parallaxX)}%`;
	}

	function onMouseMove(e: MouseEvent) {
		updateParallax(e.pageX, e.pageY);
	}

	function onTouchMove(e: TouchEvent) {
		let avgPageX = 0;
		let avgPageY = 0;

		for (const touch of e.touches) {
			avgPageX += touch.pageX / e.touches.length;
			avgPageY += touch.pageY / e.touches.length;
		}

		updateParallax(avgPageX, avgPageY);
	}
</script>

<svelte:window
	on:mousemove={onMouseMove}
	on:touchmove={onTouchMove}
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
			<div class="footer-text" id="not-sponsored-by-the-rust-foundation">This website is not sponsored by the Rust Foundation</div>
			<div class="footer-text" id="arch-btw">
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

	@media (max-width: 620px) {
		footer {
			justify-content: center;
		}

		#not-sponsored-by-the-rust-foundation {
			display: none;
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
