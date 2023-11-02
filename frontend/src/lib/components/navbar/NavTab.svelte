<script lang="ts">
	let className: string = '';
	export { className as class };

	let hrefPath: string;
	export { hrefPath as href };

	export let icon: string;
	export let title: string;
	export let disabled: boolean = false;

	import { page } from '$app/stores';
</script>

<li>
	<a
		href={hrefPath}
		aria-current={!disabled && $page.url.pathname === hrefPath}
		class={`navtab kinda-button centered-content nav-block${
			disabled ? ' disabled' : ''
		} ${className}`}
	>
		<img class="tab-icon" src={icon} alt={title} />
		<div>{title}</div>
	</a>
</li>
