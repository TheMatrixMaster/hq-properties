<script lang="ts">
	import { _ } from 'svelte-i18n';
	import { goto } from '$app/navigation';
	import Header from '../../Header.svelte';
	import TitleBar from '$lib/components/TitleBar.svelte';
	import ListingCard from '$lib/components/ListingCard.svelte';
	import Pagination from '$lib/components/Pagination.svelte';

	export let data;

	$: page = parseInt(data.slug) ?? 1;
	$: offset = (page - 1) * data.LIMIT;

	const changePage = (page: number) => goto(`/listings/${page}`);
</script>

<svelte:head>
	<title>{$_('listings')}</title>
	<meta name="listings" content="All my listings" />
</svelte:head>

<Header isTransparent={false} />
<section>
	<TitleBar title={$_('listings')} />
	<div class="filter-container" />
	<div class="card-container">
		{#if data.fetching}
			<p>{$_('loading')}</p>
		{:else if data.error}
			<p>{data.error}</p>
		{:else if data.listings.length === 0}
			<p>{$_('no_listings_list')}</p>
		{:else}
			{#each data.listings as listing}
				<ListingCard data={listing} mode="list" />
			{/each}
		{/if}
	</div>
	<Pagination size={data.size} limit={data.LIMIT} {offset} onPress={changePage} />
</section>

<style lang="scss">
	section {
		grid-template-rows: fit-content(100%) fit-content(100%) auto fit-content(100%);
		grid-template-columns: auto;
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
		grid-template-columns: repeat(var(--carousel-num-items), 1fr);
	}
</style>
