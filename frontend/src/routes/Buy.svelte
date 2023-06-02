<script>
	import './styles.css';
	import BuyImg from '$lib/images/buy.jpeg';
	import TitleBar from '$lib/components/TitleBar.svelte';
</script>

<section id="buy" class="buy-sell container-center">
	<TitleBar title="Download our Buyer's Guide" />
	<div class="left-col">
		<img alt="buy-home" src={BuyImg} />
		<p>
			Purchasing a new property is no small feat, and we are here to help. We put together this
			information together to help guide you through the various steps and stages of purchasing a
			property in Quebec. Our objective is to provide you with relevant information and sound advice
			from beginning to end.
		</p>
	</div>
	<div class="right-col">
		<form>
			<label class="half">
				First name
				<input type="text" placeholder="John" />
			</label>
			<label class="half">
				Last name
				<input type="text" placeholder="Smith" />
			</label>
			<label class="half">
				Email
				<input type="email" placeholder="john.smith@email.com" />
			</label>
			<label class="half">
				Phone
				<input type="tel" />
			</label>
			<div class="full">
				<label for="time">When are you planning to buy?</label>
				<input name="time" id="time" />
			</div>
			<div class="half">
				<label for="home-type">Home type</label>
				<input name="home-type" id="home-type" />
			</div>
			<div class="half">
				<label for="bedrooms">Bedrooms</label>
				<input name="bedroooms" id="bedrooms" />
			</div>
			<div class="full">
				<label for="location">Location</label>
				<input name="location" id="location" />
			</div>
			<label class="full">
				Anything else we should know?
				<textarea rows="3" />
			</label>
			<button class="primary" type="submit">Send</button>
		</form>
	</div>
</section>
