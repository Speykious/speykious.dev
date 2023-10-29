<script lang="ts">
	import '$lib/assets/scss/spey.scss';
	import '$lib/assets/scss/debug.scss';
	import '$lib/assets/scss/navbar.scss';

	import iconMenu from '$lib/assets/svgs/icon-menu.svg';

    export let open = false;

    function toggleOpen() {
        open = !open;
    }

	let className: string = '';
	export { className as class };
</script>

<button on:click={toggleOpen} class={`nav-edge right nav-block kinda-button centered-content ${className}`} role="button" aria-pressed="false" tabindex="0">
	<img class="tab-icon" src={iconMenu} alt="Menu" />
</button>
