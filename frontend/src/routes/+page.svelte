<script lang="ts">
	import '$lib/assets/scss/spey.scss';
	import '$lib/assets/scss/debug.scss';

	import speyPng from '$lib/assets/images/spey.png';
	import roundChevronLeft from '$lib/assets/svgs/round-chevron-left.svg';
	import roundChevronRight from '$lib/assets/svgs/round-chevron-right.svg';

	import logoGithub from '$lib/assets/svgs/logo-github.svg';
	import logoYoutube from '$lib/assets/svgs/logo-youtube.svg';
	import logoBluesky from '$lib/assets/svgs/logo-bluesky.svg';
	import logoVtsocial from '$lib/assets/svgs/logo-vtsocial.svg';
	import logoAnilist from '$lib/assets/svgs/logo-anilist.svg';
	import logoOsu from '$lib/assets/svgs/logo-osu.svg';

	import archBtw from '$lib/assets/svgs/icon-arch.svg';
	import { onMount } from 'svelte';

	let speySetup: HTMLElement;

	let pageWidth: number;
	let pageHeight: number;
	let animationFrameHandle: number;

	// parallax

	function clamp(x: number, a: number, b: number): number {
		return Math.min(b, Math.max(a, x));
	}

	function updateParallax(pageX: number, pageY: number) {
		const parallaxX = clamp(pageX / pageWidth, 0, 1);
		const parallaxY = clamp(pageY / pageHeight, 0, 1);

		speySetup.style.top = `${-2 * (1 - parallaxY)}%`;
		speySetup.style.left = `${-2 * (1 - parallaxX)}%`;
	}

	function onMouseMove(e: MouseEvent) {
		// Throttling the event handler because otherwise it can't
		// handle updating the parallax every frame for some reason >:(
		if (animationFrameHandle) cancelAnimationFrame(animationFrameHandle);
		animationFrameHandle = requestAnimationFrame(() => updateParallax(e.pageX, e.pageY));
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
			if (speyAge) {
				speyAge.innerText = speyDecimalAge().toLocaleString('en-US', {
					maximumFractionDigits: 7,
					minimumFractionDigits: 7
				});
			}

			frame = requestAnimationFrame(updateOnFrame);
		}

		updateOnFrame();

		return () => cancelAnimationFrame(frame);
	});
</script>

<svelte:head>
	<title>Speykious's website :3</title>

	<meta property="og:type" content="website" />
	<meta property="og:title" content="speykious.dev" />
	<meta property="og:description" content="Behold, my personal website :3" />
	<meta
		property="og:image"
		content="https://fs.speykious.dev/spey-coe-setup-2024-hwysi-bannercrop.jpg"
	/>
	<meta property="og:image:alt" content="My gamer setup at Cavoe's osu! Event 2024." />
	<meta name="twitter:card" content="summary_large_image" />
	<meta name="theme-color" content="#FF4B77" data-react-helmet="true" />
</svelte:head>

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
	<div class="spey-socials-bar centered-content">
		<a class="spey-social github" rel="me" href="https://github.com/Speykious">
			<img src={logoGithub} alt="Github" />
			<div class="tooltip">Github</div>
		</a>
		<a class="spey-social youtube" rel="me" href="https://youtube.com/@Speykious">
			<img src={logoYoutube} alt="YouTube" />
			<div class="tooltip">YouTube</div>
		</a>
		<a class="spey-social bluesky" rel="me" href="https://bsky.app/profile/speykious.dev">
			<img src={logoBluesky} alt="Bluesky" />
			<div class="tooltip">Bluesky</div>
		</a>
		<a class="spey-social vtsocial" rel="me" href="https://vt.social/@Speykious">
			<img src={logoVtsocial} alt="vt.social" />
			<div class="tooltip">vt.social <small>(Mastodon)</small></div>
		</a>
		<div class="spey-socials-sep"></div>
		<a class="spey-social anilist" rel="me" href="https://anilist.co/user/Speykious">
			<img src={logoAnilist} alt="Anilist" />
			<div class="tooltip">Anilist</div>
		</a>
		<a class="spey-social osu" rel="me" href="https://osu.ppy.sh/users/19553508">
			<img src={logoOsu} alt="osu!" />
			<div class="tooltip">osu!</div>
		</a>
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
		top: -2%;
		left: -2%;
		width: 104%;
		height: 104%;
		z-index: 0;

		background-image: url('https://fs.speykious.dev/spey-coe-setup-2024-hwysi.jpg');
		background-repeat: no-repeat;
		background-size: cover;
		background-position: center;

		transition: cubic-bezier(0.23, 1, 0.32, 1) 1s;

		filter: blur(10px) brightness(50%);
	}
</style>
