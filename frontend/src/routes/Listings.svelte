<script>
	import { goto } from '$app/navigation';
	import ListingCard from '$lib/components/ListingCard.svelte';
	import TitleBar from '$lib/components/TitleBar.svelte';
	import Carousel from '$lib/components/Carousel.svelte';
</script>

<section id="listings">
	<TitleBar title="My Listings" action={{ label: 'See all', onPress: () => goto('/listings') }} />
	<Carousel selector="listings-siema">
		<ListingCard mode="carousel" />
		<ListingCard mode="carousel" />
		<ListingCard mode="carousel" />
		<ListingCard mode="carousel" />
	</Carousel>
</section>

<style>
	#listings {
		display: flex;
		flex-direction: column;
		justify-content: space-between;
		background: var(--color-bg-0);
	}
</style>
