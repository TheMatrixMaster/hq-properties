<script lang="ts">
	import ArrowRight from '$lib/images/arrow-right.svg';

	type Action = {
		label: string;
		onPress: () => void;
	};

	export let title = 'Some Title';
	export let action: Action | undefined = undefined;
</script>

<div>
	<h1>{title}</h1>
	{#if action}
		<button class="secondary" on:click={action.onPress}>
			{action.label}
			<img alt="icon" id="icon" src={ArrowRight} />
		</button>
	{/if}
</div>

<style lang="scss">
	div {
		display: flex;
		grid-column: 1/13;
		flex-direction: row;
		align-items: center;
		justify-content: space-between;
	}
	div h1 {
		font-weight: bolder;
		margin: 1.5rem 0 2rem 0;
	}
</style>
