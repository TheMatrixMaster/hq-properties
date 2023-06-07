<script lang="ts">
	import { onMount } from 'svelte';
	import { PUBLIC_SERVER_URL } from '$env/static/public';
	import { goto } from '$app/navigation';
	import ReviewCard from '$lib/components/ReviewCard.svelte';
	import TitleBar from '$lib/components/TitleBar.svelte';
	import Carousel from '$lib/components/Carousel.svelte';

	let error: string | undefined = undefined;
	let fetching: boolean = true;
	let reviews: any = [];

	onMount(async () => {
		fetch(PUBLIC_SERVER_URL + '/reviews?limit=6&offset=0&published=false', {
			method: 'GET',
			headers: { Accept: 'application/json' }
		})
			.then((resp) => resp.json())
			.then((r) => (reviews = r.data))
			.catch((err) => {
				console.error(err);
				error = err;
			})
			.then(() => {
				fetching = false;
			});
	});
</script>

<section id="reviews">
	<TitleBar
		title="Testimonials"
		action={{ label: 'Write a testimonial', onPress: () => goto('/reviews/new') }}
	/>
	{#if fetching}
		<p>Loading</p>
	{:else if error}
		<p>{error}</p>
	{:else}
		<Carousel maxItems={reviews.length} selector="reviews-siema">
			{#each reviews as review}
				<ReviewCard data={review} mode="carousel" />
			{/each}
		</Carousel>
	{/if}
</section>

<style>
	#reviews {
		display: flex;
		flex-direction: column;
		justify-content: space-between;
		background: var(--color-bg-1);
	}
</style>
