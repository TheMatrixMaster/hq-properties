<script>
	import { goto } from '$app/navigation';
	import ReviewCard from '$lib/components/ReviewCard.svelte';
	import TitleBar from '$lib/components/TitleBar.svelte';
	import Carousel from '$lib/components/Carousel.svelte';
</script>

<section id="reviews">
	<TitleBar
		title="Testimonials"
		action={{ label: 'Write a testimonial', onPress: () => goto('/reviews/new') }}
	/>
	<Carousel selector="reviews-siema">
		<ReviewCard mode="carousel" />
		<ReviewCard mode="carousel" />
		<ReviewCard mode="carousel" />
		<ReviewCard mode="carousel" />
	</Carousel>
</section>

<style>
	#reviews {
		display: flex;
		flex-direction: column;
		justify-content: space-between;
		background: var(--color-bg-1);
	}
</style>
