<script lang="ts">
	import { dummy_posts } from '$lib/dummy';
	import type { Post } from '$lib/dummy';

	export let mode: 'carousel' | 'list' = 'carousel';
	export let data: Post = dummy_posts[0];

	$: displayTime = new Date(data.created_at).toLocaleDateString();
</script>

<div class={`container ${mode}`}>
	<img src={data.img_url} alt={data.title} />
	<div class="overlay">
		<h2>{data.title}</h2>
		<h5>{displayTime}</h5>
		<p>{data.body}</p>
	</div>
</div>

<style lang="scss">
	.container {
		position: relative;
		cursor: pointer;
		overflow: hidden;
		border-radius: 5px;
	}

	.list {
		max-height: unset;
	}

	.carousel {
		display: flex;
		max-height: 35vh;
		@media (orientation: portrait) and (aspect-ratio <= 11/18) {
			max-height: 40vh;
		}
	}

	.container img {
		width: 100%;
		height: 100%;
		object-fit: cover;
		object-position: center;
	}

	.overlay {
		width: 100%;
		height: 100%;
		position: absolute;
		background: rgba(57, 57, 57, 0.7);
		top: 0;
		left: 0;
		transform: scale(0);
		transition: all 0.2s 0.1s ease-in-out;
		color: var(--color-text);
		/* center overlay content */
		display: flex;
		flex-direction: column;
		padding: 1.5rem 2rem;
		overflow: scroll;

		& > h2 {
			font-size: 1.2rem;
			margin-bottom: 0.6rem;
		}
		& > h5 {
			margin-top: 0;
			margin-bottom: 0.8rem;
		}
		& > p {
			font-size: 1rem;
			margin: 0;
		}
	}

	.container:hover .overlay {
		transform: scale(1);
	}
</style>
