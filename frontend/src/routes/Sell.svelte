<script lang="ts">
	import './styles.css';
	import SellImg from '$lib/images/sell.jpeg';
	import TitleBar from '$lib/components/TitleBar.svelte';

	import { PUBLIC_SERVER_URL } from '$env/static/public';

	const SUCCESS_MSG: string =
		"Thank you for downloading our seller's guide! We have sent it to the email address you provided.";
	// const ERROR_MSG: string = 'Form input is invalid, please try again.';

	let alert: { msg: string; mode: 'success' | 'danger' } | null = null;

	const onSubmit = (e: any) => {
		const formData = new FormData(e.target);
		const body: any = {};

		for (let field of formData) {
			const [key, value] = field;
			body[key] = value;
		}

		return fetch(`${PUBLIC_SERVER_URL}/sellers`, {
			method: 'POST',
			headers: {
				Accept: 'application/json',
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(body)
		})
			.then((resp) => resp.json())
			.then((_) => (alert = { msg: SUCCESS_MSG, mode: 'success' }))
			.catch((err) => {
				console.error(err);
				alert = { msg: err, mode: 'danger' };
			});
	};
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
		<form on:submit|preventDefault={onSubmit}>
			<label class="half">
				First name
				<input required id="first_name" name="first_name" type="text" placeholder="John" />
			</label>
			<label class="half">
				Last name
				<input required id="last_name" name="last_name" type="text" placeholder="Smith" />
			</label>
			<label class="half">
				Email
				<input required id="email" name="email" type="email" placeholder="john.smith@email.com" />
			</label>
			<label class="half">
				Phone
				<input required id="phone" name="phone" type="tel" />
			</label>
			<label class="full">
				Home Address
				<input required id="address" name="address" type="text" autocomplete="street-address" />
			</label>
			<label class="half">
				Postal Code (Optional)
				<input id="postal_code" name="postal_code" type="text" autocomplete="postal-code" />
			</label>
			<label class="half">
				City
				<input required id="city" name="city" type="text" autocomplete="address-level2" />
			</label>
			<div class="full">
				<label for="sell_period">When are you planning to sell?</label>
				<select required id="sell_period" name="sell_period" value={undefined}>
					<option value="Immediately">Immediately</option>
					<option value="Next month">Next Month</option>
					<option value="Within the next three months"> Within the next three months </option>
					<option value="Within the next six months"> Within the next six months </option>
					<option value="Next year">Next year</option>
					<option value="Undecided">Undecided</option>
				</select>
			</div>
			<label class="full">
				Anything else we should know?
				<textarea id="other" name="other" rows="2" />
			</label>
			<button class="primary" type="submit">Send</button>
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
