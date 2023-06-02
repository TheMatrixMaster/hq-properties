<script>
	import './styles.css';
	import SellImg from '$lib/images/sell.jpeg';
	import TitleBar from '$lib/components/TitleBar.svelte';
</script>

<section id="sell" class="buy-sell container-center">
	<TitleBar title="Download our Seller's Guide" />
	<div class="left-col">
		<img alt="sell-home" src={SellImg} />
		<p>
			You want to sell at the highest possible price, with the least amount of effort. So, where to
			start? It takes a broker with an in-depth understanding of your neighbourhood’s real estate
			market to recognise your unique home's place within it. We encourage you to seek our advice
			early in the process, and share your goals with us, so that we can develop a winning strategy
			together.
		</p>
	</div>
	<div class="right-col">
		<form>
			<label class="half">
				First name
				<input type="text" />
			</label>
			<label class="half">
				Last name
				<input type="text" />
			</label>
			<label class="half">
				Email
				<input type="email" />
			</label>
			<label class="half">
				Phone
				<input type="tel" />
			</label>
			<label class="full">
				Home Address
				<input type="tel" />
			</label>
			<div class="full">
				<label for="time">When are you planning to sell?</label>
				<input name="time" id="time" />
			</div>
			<label class="full">
				Anything else we should know?
				<textarea rows="3" />
			</label>
			<button type="submit">Send</button>
		</form>
	</div>
</section>

<style>
	.left-col > img {
		width: auto;
		max-height: 18rem;
		object-fit: cover;
		object-position: 50% 90%;
	}
</style>
