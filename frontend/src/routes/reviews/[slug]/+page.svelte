<script lang="ts">
	import { _ } from 'svelte-i18n';
	import { goto } from '$app/navigation';
	import Header from '../../Header.svelte';
	import TitleBar from '$lib/components/TitleBar.svelte';
	import ReviewCard from '$lib/components/ReviewCard.svelte';
	import Pagination from '$lib/components/Pagination.svelte';

	export let data;

	$: page = parseInt(data.slug) ?? 1;
	$: offset = (page - 1) * data.LIMIT;

	const changePage = (page: number) => goto(`/reviews/${page}`);
</script>

<svelte:head>
	<title>{$_('testimonials')}</title>
	<meta name="reviews" content="All reviews" />
</svelte:head>

<Header isTransparent={false} />
<section>
	<TitleBar
		title={$_('testimonials')}
		action={{ label: $_('write_testimonial'), onPress: () => goto('/reviews/new') }}
	/>
	<div class="filter-container" />
	<div class="card-container">
		{#if data.fetching}
			<p>{$_('loading')}</p>
		{:else if data.error}
			<p>{data.error}</p>
		{:else if data.reviews.length === 0}
			<p>{$_('no_reviews_list')}</p>
		{:else}
			{#each data.reviews as card}
				<ReviewCard data={card} mode="list" />
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
		row-gap: 1.5rem;
		grid-template-columns: auto;
	}
</style>
