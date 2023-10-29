<script lang="ts">
    let className: string = "";
	let hrefName: string;
	export let icon: string;
	export let title: string;
	export let active: boolean = false;

	export { className as class };
	export { hrefName as href };
</script>

<a href={hrefName} class={`tab kinda-button${active ? " active" : ""} centered-content ${className}`}>
    <img class="tab-icon" src={icon} alt={title} />
    <div>{title}</div>
</a>