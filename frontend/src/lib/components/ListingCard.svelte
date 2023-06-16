<script lang="ts">
	import BedIcon from '$lib/images/bed.svg';
	import BathIcon from '$lib/images/bath.svg';
	import { dummy_listings } from '$lib/dummy';
	import type { FullListing } from '$lib/dummy';

	export let mode: 'carousel' | 'list' = 'carousel';
	export let data: FullListing = dummy_listings[0];

	$: formattedPrice = `$${data.listing.price.toLocaleString()}`;
	$: marketStatus = (() => {
		switch (data.listing.market_st) {
			case 'Sold':
				return 'Sold';
			default:
				return `for ${data.listing.market_st}`;
		}
	})();
</script>

<a class={`card-container ${mode}`} href={data.listing.listing_url} target="_blank">
	<div class="img-container">
		<div class={`market-state ${data.listing.market_st.toLowerCase()}`}>
			{marketStatus}
		</div>
		<img alt="property-img" src={data.imgs[0].url} />
	</div>
	<div class="info-container">
		<h2>{data.listing.city}</h2>
		<p>{data.listing.address}</p>
		<div class="specs-container">
			{#if data.listing.bedrooms >= 0}
				<div class="spec">
					<div>
						<h2>{data.listing.bedrooms}</h2>
						<img alt="bed-icon" src={BedIcon} />
					</div>
					<p>Bedrooms</p>
				</div>
			{/if}
			{#if data.listing.bathrooms >= 0}
				<div class="spec">
					<div>
						<h2>{data.listing.bathrooms}</h2>
						<img alt="bath-icon" src={BathIcon} />
					</div>
					<p>Bathrooms</p>
				</div>
			{/if}
			<div class="spec">
				<div>
					<h2>{data.listing.area}</h2>
				</div>
				<p>SQ. FT.</p>
			</div>
		</div>
		<h2>{formattedPrice}</h2>
	</div>
</a>

<style lang="scss">
	a {
		color: var(--color-text);
		text-decoration: none;
	}
	a:hover {
		& > div.img-container {
			opacity: 0.6;
		}
	}
	.card-container {
		display: grid;
		max-width: 100%;
		height: 100%;
		min-height: 30rem;
		grid-template-rows: auto fit-content(100%);
		background: rgba(255, 255, 255, 0.1);
	}
	.carousel {
		margin: 0 1rem;
	}
	.info-container {
		display: flex;
		overflow: hidden;
		padding: 1rem 1rem;
		flex-direction: column;
		justify-content: space-between;
		& > h2 {
			overflow: hidden;
			white-space: nowrap;
			text-overflow: ellipsis;
		}
		& > p {
			margin: 0.2rem 0 0 0;
		}
	}
	.info-container > div {
		display: flex;
		flex-direction: row;
		flex-grow: 1;
	}
	.info-container h2 {
		margin: 0;
		font-size: 1.6rem;
		font-weight: bold;
	}
	.specs-container {
		display: grid;
		gap: 2rem;
		grid-template-columns: auto auto auto;
		margin: 1rem 0;
	}
	div.spec {
		flex: 0 0 auto;
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		& > p {
			margin: 0;
			font-size: 0.9rem;
		}
		& > div {
			display: flex;
			flex-grow: 1;
			flex-direction: row;
			align-items: center;
			gap: 0.5rem;
		}
		& img {
			height: auto;
			width: 2rem;
			color: var(--color-white);
		}
	}
	.img-container {
		display: flex;
		flex-direction: column;
		position: relative;
		height: 100%;
		overflow: hidden;
	}
	.img-container > img {
		width: 100%;
		min-height: 100%;
		position: absolute;
		object-fit: cover;
	}
	div.market-state {
		top: 0;
		right: 0;
		z-index: 1;
		position: absolute;
		font-size: 1.5rem;
		padding: 0.3rem 2.5rem;
		font-weight: bolder;
		font-family: inherit;
		text-transform: capitalize;
		color: var(--color-text);
	}
	div.sale {
		background: var(--color-theme-2);
	}
	div.sold {
		background: var(--color-black);
	}
	div.rent {
		background: var(--color-theme-1);
	}
</style>
