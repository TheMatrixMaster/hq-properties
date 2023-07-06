<script lang="ts">
	import { goto } from '$app/navigation';
	import Header from '../../Header.svelte';
	import TitleBar from '$lib/components/TitleBar.svelte';
	import PostCard from '$lib/components/PostCard.svelte';
	import Pagination from '$lib/components/Pagination.svelte';

	export let data;

	$: page = parseInt(data.slug) ?? 1;
	$: offset = (page - 1) * data.LIMIT;

	const changePage = (page: number) => goto(`/posts/${page}`);

	const splitIntoChunks = (list: any[], numChunks: number) => {
		if (numChunks <= 0) return list;

		let chunkSize = Math.ceil(list.length / numChunks);
		let numLargeChunks = list.length % numChunks;
		if (numLargeChunks === 0) numLargeChunks = list.length;

		let curr = 0;
		const res = [];

		for (let i = 0; i < numLargeChunks; i++) {
			res.push(list.slice(curr, curr + chunkSize));
			curr += chunkSize;
		}

		chunkSize -= 1;

		for (; curr < list.length; curr += chunkSize) res.push(list.slice(curr, curr + chunkSize));

		while (res.length < numChunks) res.push([]);
		return res;
	};

	$: sections = splitIntoChunks(data.posts, 3);
</script>

<svelte:head>
	<title>Posts</title>
	<meta name="posts" content="All posts" />
</svelte:head>

<Header isTransparent={false} />
<section>
	<TitleBar title="Posts" />
	<div class="filter-container" />
	<div class="card-container">
		{#if data.fetching}
			<p>Loading</p>
		{:else if data.error}
			<p>{data.error}</p>
		{:else if data.posts.length === 0}
			<p>No posts were found.</p>
		{:else}
			<div class="column">
				{#each sections[0] as post}
					<PostCard data={post} mode="list" />
				{/each}
			</div>
			<div class="column">
				{#each sections[1] as post}
					<PostCard data={post} mode="list" />
				{/each}
			</div>
			<div class="column">
				{#each sections[2] as post}
					<PostCard data={post} mode="list" />
				{/each}
			</div>
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
		display: flex;
		width: 100%;
		flex-direction: row;
		gap: 1.5rem;
		grid-column: span 12;

		& > div.column {
			flex: 1;
			display: flex;
			flex-direction: column;
			gap: 1.5rem;
		}

		@media (orientation: portrait) and (aspect-ratio <= 11/18) {
			flex-direction: column;
		}
	}
</style>
