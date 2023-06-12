<script lang="ts">
	import { onMount } from 'svelte';
	import { PUBLIC_SERVER_URL } from '$env/static/public';
	import { goto } from '$app/navigation';
	import ListingCard from '$lib/components/ListingCard.svelte';
	import TitleBar from '$lib/components/TitleBar.svelte';
	import Carousel from '$lib/components/Carousel.svelte';

	let error: string | undefined = undefined;
	let fetching: boolean = true;
	let listings: any = [];

	onMount(async () => {
		fetch(PUBLIC_SERVER_URL + '/listings?limit=6&offset=0&market_st=sale', {
			method: 'GET',
			headers: { Accept: 'application/json' }
		})
			.then((resp) => resp.json())
			.then((r) => (listings = r.data))
			.catch((err) => {
				console.error(err);
				error = err;
			})
			.then(() => {
				fetching = false;
			});
	});
</script>

<section id="listings">
	<TitleBar title="My Listings" action={{ label: 'See all', onPress: () => goto('/listings/1') }} />
	{#if fetching}
		<p>Loading</p>
	{:else if listings.length == 0}
		<p>There are currently no listings to show. Please come back later.</p>
	{:else if error}
		<p>{error}</p>
	{:else}
		<Carousel maxItems={listings.length} selector="listings-siema">
			{#each listings as listing}
				<ListingCard data={listing} mode="carousel" />
			{/each}
		</Carousel>
	{/if}
</section>

<style>
	#listings {
		display: flex;
		flex-direction: column;
		justify-content: space-between;
		background: var(--color-bg-0);
	}
</style>
