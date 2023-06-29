<script lang="ts">
	import './styles.scss';
	import { alert } from '../stores';
	import { download, resetForm } from '../util';
	import { PUBLIC_SERVER_URL } from '$env/static/public';

	import SellImg from '$lib/images/sell.jpeg';
	import TitleBar from '$lib/components/TitleBar.svelte';

	const SUCCESS_MSG: string = "Thank you for downloading our seller's guide!";

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
			.then((_) => ($alert = { msg: SUCCESS_MSG, mode: 'success' }))
			.then(() => download(`${PUBLIC_SERVER_URL}/sell.pdf`))
			.then(() => resetForm('sell-form'))
			.catch((err) => {
				console.error(err);
				$alert = { msg: err, mode: 'danger' };
			});
	};
</script>

<section id="sell" class="buy-sell container-center">
	<TitleBar title="Download our Seller's Guide" />
	<div class="left-col">
		<img alt="sell-home" src={SellImg} />
		<p>
			As your dedicated real estate broker, I prioritize maximizing your sale price while minimizing
			your effort. To achieve this, it is crucial to begin with a comprehensive understanding of
			your neighborhood's real estate market. By assessing your home's unique position within the
			market, I can provide valuable insights and guidance. I encourage you to involve me early in
			the process and share your goals, allowing us to collaborate and develop a winning strategy
			tailored to your specific needs.
		</p>
	</div>
	<div class="right-col">
		<form on:submit|preventDefault={onSubmit} id="sell-form">
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

<style lang="scss">
	.left-col > img {
		width: auto;
		max-height: 18rem;
		object-fit: cover;
		object-position: 50% 90%;
	}
</style>
