<script lang="ts">
	import { onMount } from 'svelte';
	import { PUBLIC_SERVER_URL } from '$env/static/public';
	import { goto } from '$app/navigation';
	import Header from '../../Header.svelte';
	import TitleBar from '$lib/components/TitleBar.svelte';
	import ReviewCard from '$lib/components/ReviewCard.svelte';
	import Pagination from '$lib/components/Pagination.svelte';

	const LIMIT: number = 8;
	export let data;

	let size: number;
	let reviews: any;
	let error: string;
	let fetching: boolean = true;

	$: page = parseInt(data.slug) ?? 1;
	$: offset = (page - 1) * LIMIT;

	onMount(() => {
		fetch(`${PUBLIC_SERVER_URL}/reviews?limit=${LIMIT}&offset=${offset}&published=false`)
			.then((resp) => resp.json())
			.then((r) => {
				console.log(r);
				size = r.size;
				reviews = r.data;
			})
			.catch((err) => (error = err))
			.then(() => (fetching = false));
	});

	$: numPages = Math.ceil(size / LIMIT);
	$: console.log(numPages);

	const changePage = (page: number) => goto(`/reviews/${page}`);
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
		{#if fetching}
			<p>Loading</p>
		{:else if error}
			<p>{error}</p>
		{:else if reviews.length === 0}
			<p>It's a little empty in here. Why don't you write the first review!</p>
		{:else}
			{#each reviews as card}
				<ReviewCard data={card} mode="list" />
			{/each}
		{/if}
	</div>
	<Pagination {size} limit={LIMIT} {offset} onPress={changePage} />
</section>

<style>
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
