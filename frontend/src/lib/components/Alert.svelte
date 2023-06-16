<script lang="ts">
	export let msg = '';
	export let mode: 'success' | 'danger' = 'success';
	export let visible = false;
	$: display = visible ? 'unset' : 'hidden';
</script>

<div class={`main-container ${mode} ${display}`}>
	<p><strong>{mode} alert!</strong> {msg}</p>
</div>

<style>
	div.main-container {
		position: absolute;
		right: 3rem;

		display: flex;
		border-radius: 6px;
		padding: 0.5rem 1rem;
		flex-direction: row;
		background-color: rgb(31 41 55);

		@media (orientation: portrait) {
			right: unset;
			position: relative;
			width: -webkit-fill-available;
			grid-column: span 12;
			margin: 0;
		}
		@media (orientation: landscape) {
			max-width: 45vw;
		}

		& strong {
			text-transform: capitalize;
		}
		& > p {
			margin: 0;
			font-size: 1rem;
		}
	}

	div.success {
		color: rgb(49 196 141);
	}
	div.danger {
		color: rgb(249 128 128);
	}
	div.hidden {
		display: none;
	}
</style>
