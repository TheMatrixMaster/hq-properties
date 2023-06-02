<script>
	import Header from '../Header.svelte';
	import TitleBar from '$lib/components/TitleBar.svelte';
	import { dummy_listings } from '$lib/dummy';
	import ListingCard from '$lib/components/ListingCard.svelte';

	let cards = dummy_listings;
</script>

<svelte:head>
	<title>Listings</title>
	<meta name="listings" content="All my listings" />
</svelte:head>

<Header isTransparent={false} />
<section>
	<TitleBar title="My Listings" />
	<div class="filter-container" />
	<div class="card-container">
		{#each cards as card}
			<ListingCard data={card} mode="list" />
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
		gap: 2rem;
		row-gap: 1.5rem;
		grid-template-columns: repeat(var(--carousel-num-items), auto);
	}
</style>
