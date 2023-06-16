<script lang="ts">
	import Siema from 'siema';
	import { onMount } from 'svelte';
	import ArrowRight from '$lib/images/arrow-right.svg';
	import ArrowLeft from '$lib/images/arrow-left.svg';

	export let maxItems: number;
	export let selector = 'siema';
	let slider: Siema, prev: () => void, next: () => void;
	let select = 0;

	onMount(() => {
		const numItems: number = Math.min(
			maxItems,
			parseInt(getComputedStyle(document.documentElement).getPropertyValue('--carousel-num-items'))
		);

		slider = new Siema({
			selector: `.${selector}`,
			duration: 200,
			easing: 'ease-in-out',
			perPage: numItems ?? 2,
			startIndex: 0,
			draggable: true,
			multipleDrag: true,
			threshold: 20,
			loop: true,
			rtl: false,
			onInit: () => null,
			onChange: () => null
		});

		prev = () => {
			slider.prev();
			if (select > 0) {
				select--;
			}
		};

		next = () => {
			slider.next();
			if (select >= 0) {
				select++;
			}
		};
	});
</script>

<div class="{selector} layout">
	<slot />
</div>

<div class="siema-control">
	<button class="secondary icon-only" on:click={prev}>
		<img alt="prev" id="icon" src={ArrowLeft} />
	</button>
	<button class="secondary icon-only" on:click={next}>
		<img alt="next" id="icon" src={ArrowRight} />
	</button>
</div>

<style lang="scss">
	.layout {
		display: grid;
		flex-grow: 1;
		margin: 0 -1rem;
	}
	.siema-control {
		display: flex;
		flex-direction: row;
		margin: 1.5rem 0;
		gap: 1rem;
	}
</style>
