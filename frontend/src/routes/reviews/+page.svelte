<script>
	import { goto } from '$app/navigation';
	import Header from '../Header.svelte';
	import TitleBar from '$lib/components/TitleBar.svelte';
	import { dummy_reviews } from '$lib/dummy';
	import ReviewCard from '$lib/components/ReviewCard.svelte';

	let cards = dummy_reviews;
</script>

<svelte:head>
	<title>Testimonials</title>
	<meta name="reviews" content="All reviews" />
</svelte:head>

<Header isTransparent={false} />
<section>
	<TitleBar
		title="Testimonials"
		action={{ label: 'Write a testimonial', onPress: () => goto('/reviews/new') }}
	/>
	<div class="filter-container" />
	<div class="card-container">
		{#each cards as card}
			<ReviewCard data={card} mode="list" />
		{/each}
	</div>
</section>

<style>
	section {
		grid-template-rows: fit-content(100%) fit-content(100%) auto;
	}
	div.filter-container {
		gap: 1rem;
		display: flex;
		flex-direction: row;
		align-items: center;
	}
	div.card-container {
		display: grid;
		grid-column: 1/13;
		row-gap: 1.5rem;
		grid-template-columns: auto;
	}
</style>
