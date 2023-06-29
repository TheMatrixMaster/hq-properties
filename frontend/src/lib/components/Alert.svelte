<script lang="ts">
	import { onDestroy } from 'svelte';
	import { alert } from '../../stores';
	import type { Alert } from '../../stores';

	export let ms: number = 3000;
	let timeout: any;
	let visible: boolean;

	let msg = '';
	let mode: 'success' | 'danger' = 'success';

	const onMessageChange = (alert: Alert, ms: number) => {
		clearTimeout(timeout);
		if (!alert) {
			visible = false;
		} else {
			visible = true;
			msg = alert.msg;
			mode = alert.mode;
			let time = alert?.ms ?? ms;
			if (ms > 0) {
				timeout = setTimeout(() => (visible = false), time);
			}
		}
	};

	$: onMessageChange($alert, ms);
	$: display = visible ? 'unset' : 'hidden';

	onDestroy(() => clearTimeout(timeout));
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<div class={`main-container ${mode} ${display}`} on:click={() => (visible = false)}>
	<p><strong>{mode} alert!</strong> {msg}</p>
</div>

<style lang="scss">
	div.main-container {
		z-index: 3;
		position: fixed;
		top: calc(var(--header-mini-height) + 1rem);
		right: 3rem;

		display: flex;
		border-radius: 6px;
		padding: 0.5rem 1rem;
		flex-direction: row;
		background-color: rgb(31 41 55);

		@media (orientation: portrait) {
			right: unset;
			border-radius: 0;
			top: var(--header-mini-height);
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
