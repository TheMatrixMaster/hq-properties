<script lang="ts">
	export let size: number;
	export let limit: number;
	export let offset: number;
	export let onPress: (page: number) => void;

	$: numPages = Math.ceil(size / limit);
	$: currPage = Math.floor(offset / limit) + 1;
	$: start = offset + 1;
	$: end = Math.min(offset + 1 + limit, size);
</script>

<div class="main-container">
	<p>Showing <strong>{start}</strong> to <strong>{end}</strong> of {size} entries</p>
	<div class="btn-container">
		<button class="secondary" disabled={currPage === 1} on:click={() => onPress(currPage - 1)}>
			Previous
		</button>
		<button
			class="secondary"
			disabled={currPage === numPages}
			on:click={() => onPress(currPage + 1)}
		>
			Next
		</button>
	</div>
</div>

<style>
	div.main-container {
		display: flex;
		gap: 0.5rem;
		margin: 2rem 0;
		align-items: center;
		flex-direction: column;
		grid-column: span 12;
		& > p {
			margin: 0;
			white-space: nowrap;
		}
	}
	div.btn-container {
		display: flex;
		gap: 0.5rem;
		flex-direction: row;
	}
</style>
