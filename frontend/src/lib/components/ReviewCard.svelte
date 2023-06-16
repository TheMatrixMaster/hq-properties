<script lang="ts">
	import { goto } from '$app/navigation';
	import { dummy_reviews } from '$lib/dummy';
	import type { Review } from '$lib/dummy';

	export let mode: 'carousel' | 'list' = 'carousel';
	export let data: Review = dummy_reviews[0];

	const getInitials = (): string => {
		if (data.anonymous) return 'A';
		else return `${data.first_name[0].toUpperCase()}${data.last_name[0].toUpperCase()}`;
	};

	$: displayTime = new Date(data.created_at).toLocaleDateString();
</script>

{#if mode === 'carousel'}
	<div class="base-card carousel-card">
		<div class="avatar-container">
			<div class="avatar">
				<h1>{getInitials()}</h1>
			</div>
			<h2>{data.anonymous ? 'Anonymous' : data.first_name}</h2>
		</div>
		<div class="body-container">
			<p>{data.body}</p>
		</div>
		<button class="secondary" on:click={() => goto('/reviews/1')}>Read More</button>
	</div>
{:else}
	<div class="base-card list-card">
		<div class="list-header">
			<h2>{data.anonymous ? 'Anonymous' : `${data.first_name} ${data.last_name}`}</h2>
			<h3>{displayTime}</h3>
		</div>
		<p>{data.body}</p>
	</div>
{/if}

<style lang="scss">
	div.base-card {
		height: 100%;
		max-width: 100%;
		backdrop-filter: blur(10px);
		background: rgba(255, 255, 255, 0.05);
	}
	div.carousel-card {
		display: grid;
		margin: 0 1rem;
		padding: 2rem 4rem;
		/* max-height: 500px; */
		grid-template-rows: fit-content(100%) auto fit-content(100%);
	}
	div.list-card {
		display: flex;
		padding: 1rem 2rem;
		flex-direction: column;
		height: min-content;

		& > p {
			font-size: 1.1rem;
			margin: 0;
		}
	}
	div.list-header {
		display: flex;
		margin-bottom: 0.5rem;
		flex-direction: row;
		justify-content: space-between;
		align-items: center;

		& > h2,
		h3 {
			margin: 0;
		}
		& > h2 {
			font-size: 1.6rem;
		}
	}

	.carousel-card button {
		justify-self: center;
	}
	div.body-container {
		display: flex;
		flex-direction: column;
		justify-content: center;
	}
	.body-container p {
		overflow-wrap: anywhere;
		hyphens: manual;
		text-overflow: ellipsis;
		overflow: hidden;
		display: -webkit-box;
		-webkit-box-orient: vertical;
		-webkit-line-clamp: 7;
	}
	div.avatar-container {
		display: flex;
		flex-direction: column;
		align-items: center;
	}
	.avatar-container h2 {
		font-weight: bold;
		margin: 0.5rem 0 0 0;
	}
	div.avatar {
		width: 5rem;
		height: 5rem;
		display: flex;
		align-items: center;
		justify-content: center;
		background: var(--color-bg-2);
		box-shadow: 0px 4px 4px rgba(0, 0, 0, 0.25);
		border-radius: 200px;
	}
</style>
