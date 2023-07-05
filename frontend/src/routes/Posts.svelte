<script lang="ts">
	import './styles.scss';
	import { onMount } from 'svelte';
	import { PUBLIC_SERVER_URL } from '$env/static/public';
	import { goto } from '$app/navigation';

	import TitleBar from '$lib/components/TitleBar.svelte';
	import PostCard from '$lib/components/PostCard.svelte';

	let error: string | undefined = undefined;
	let fetching: boolean = false;
	let posts: any = [];

	onMount(async () => {
		fetch(PUBLIC_SERVER_URL + '/posts?limit=3&offset=0', {
			method: 'GET',
			headers: { Accept: 'application/json' }
		})
			.then((resp) => resp.json())
			.then((r) => (posts = r.data))
			.catch((err) => {
				console.error(err);
				error = err;
			})
			.then(() => {
				fetching = false;
			});
	});
</script>

<section id="posts">
	<TitleBar title="Posts" action={{ label: 'See all', onPress: () => goto('/posts/1') }} />
	{#if fetching}
		<p>Loading</p>
	{:else if posts.length == 0}
		<p>No posts have been written yet.</p>
	{:else if error}
		<p>{error}</p>
	{:else}
		<div class="post-container">
			{#each posts as post}
				<PostCard data={post} mode="carousel" />
			{/each}
		</div>
	{/if}
</section>

<style lang="scss">
	#posts {
		display: flex;
		flex-direction: column;
		justify-content: space-between;
		background: var(--color-bg-0);
		min-height: unset;
	}
	div.post-container {
		gap: 1rem;
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		@media (orientation: portrait) and (aspect-ratio <= 11/18) {
			grid-template-columns: auto;
		}
		@media (orientation: portrait) and (aspect-ratio > 11/18) {
			grid-template-columns: 1fr 1fr;
		}
	}
</style>
