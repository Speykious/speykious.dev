<script lang="ts">
	import '$lib/assets/scss/spey.scss';
	import '$lib/assets/scss/debug.scss';

	import speyPng from '$lib/assets/images/spey.png';
	import roundChevronLeft from '$lib/assets/svgs/round-chevron-left.svg';
	import roundChevronRight from '$lib/assets/svgs/round-chevron-right.svg';

	import archBtw from '$lib/assets/svgs/icon-arch.svg';
	import { onMount } from 'svelte';

	let speySetup: HTMLElement;

	let pageWidth: number;
	let pageHeight: number;

	// parallax

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

	// age calculation

	let speyAge: HTMLElement;

	function secsSinceEpoch(date: Date): number {
		return date.getTime() / 1000.0;
	}

	function speyDecimalAge() {
		const speyBirthYear = 2001;

		const todayDate = new Date();
		const today = secsSinceEpoch(todayDate);
		const thisYear = todayDate.getFullYear();

		const birthdayThisYear = secsSinceEpoch(new Date(`Oct 03 ${thisYear} 00:00:00 UTC+2`));

		if (today >= birthdayThisYear) {
			// birthday coming up next year!

			const birthdayNextYear = secsSinceEpoch(new Date(`Oct 03 ${thisYear + 1} 00:00:00 UTC+2`));

			const ageYear = thisYear - speyBirthYear;
			const ageDecimals = (today - birthdayThisYear) / (birthdayNextYear - birthdayThisYear);

			return ageYear + ageDecimals;
		} else {
			// birthday coming up this year!

			const birthdayPrevYear = secsSinceEpoch(new Date(`Oct 03 ${thisYear - 1} 00:00:00 UTC+2`));

			const ageYear = thisYear - 1 - speyBirthYear;
			const ageDecimals = (today - birthdayPrevYear) / (birthdayThisYear - birthdayPrevYear);

			return ageYear + ageDecimals;
		}
	}

	onMount(() => {
		let frame: number;

		function updateOnFrame() {
			speyAge.innerText = speyDecimalAge().toLocaleString('en-US', {
				maximumFractionDigits: 7,
				minimumFractionDigits: 7
			});

			requestAnimationFrame(updateOnFrame);
		}

		updateOnFrame();

		return () => cancelAnimationFrame(frame);
	});
</script>

<svelte:window
	on:mousemove={onMouseMove}
	on:touchmove={onTouchMove}
	bind:innerWidth={pageWidth}
	bind:innerHeight={pageHeight}
/>

<figure id="spey-setup-bg" bind:this={speySetup} />
<div class="center-panel centered-content">
	<figure class="chevronned-spey">
		<img class="chevron" src={roundChevronLeft} alt="left chevron" />
		<img class="spey-pfp" src={speyPng} alt="yours truly" />
		<img class="chevron" src={roundChevronRight} alt="right chevron" />
	</figure>
	<h1>Hello :3</h1>
	<div class="spey-description">
		I'm <strong>Speykious</strong>, a
		<span id="spey-age" bind:this={speyAge}>??</span>
		year old software developer, huge weeb and rhythm games enthusiast.
	</div>
</div>
<footer>
	<div class="footer-text" id="not-sponsored-by-the-rust-foundation">
		This website is not sponsored by the Rust Foundation™
	</div>
	<div class="footer-text" id="arch-btw">
		<div>I use Arch btw</div>
		<img src={archBtw} alt="Arch btw" />
	</div>
</footer>

<style lang="scss">
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

		background-image: url('https://fs.speykious.dev/spey-coe-setup-2024-hwysi.jpg');
		background-repeat: no-repeat;
		background-size: cover;
		background-position: center;

		filter: blur(10px) brightness(50%);
	}
</style>
