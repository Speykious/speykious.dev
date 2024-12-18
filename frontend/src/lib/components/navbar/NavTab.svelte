<script lang="ts">
	let className: string = '';
	export { className as class };

	export let href: string;
	$: active = href === $page.url.pathname;

	export let icon: string;
	export let title: string;
	export let disabled: boolean = false;

	import { page } from '$app/stores';
</script>

<li>
	<a
		{href}
		aria-current={!disabled && active}
		class:disabled
		class={`navtab kinda-button centered-content nav-block ${className}`}
	>
		<div class="navtab-content">
			<img class="tab-icon" src={icon} alt={title} />
			<div>{title}</div>
		</div>
		<div class="navtab-bottom-bar"></div>
	</a>
</li>
