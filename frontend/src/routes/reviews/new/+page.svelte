<script>
	import TitleBar from '$lib/components/TitleBar.svelte';
	import Header from '../../Header.svelte';
</script>

<svelte:head>
	<title>Testimonials — New</title>
	<meta name="reviews" content="new review" />
</svelte:head>

<Header isTransparent={false} />
<section>
	<TitleBar title="Write a Testimonial" />
	<form>
		<label class="half">
			First name
			<input type="text" placeholder="ex: John" />
		</label>
		<label class="half">
			Last name
			<input type="text" placeholder="ex: Smith" />
		</label>
		<label class="full textbox">
			Review
			<textarea rows="8" placeholder="Write a review..." />
		</label>
		<div class="full checkbox-container">
			<input type="checkbox" name="anonymous" id="anonymous" />
			<div class="checkbox-label">
				<label for="anonymous"><strong>Anonymous</strong></label>
				Check this if you'd like to hide your name on the website
			</div>
		</div>
		<button class="primary" type="submit">Send</button>
	</form>
</section>

<style>
	section {
		flex-grow: 1;
		min-height: unset;
		padding-bottom: 3rem;
		height: calc(100vh - var(--header-height));
		grid-template-rows: fit-content(100%) auto;
	}
	form {
		display: grid;
		grid-column: 1/13;
		grid-template-rows: fit-content(100%) auto fit-content(100%) fit-content(100%);
		@media (orientation: portrait) {
			grid-template-rows: repeat(5, fit-content(100%));
		}
	}
	textarea {
		height: 90%;
	}
	div.checkbox-label {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}
</style>
