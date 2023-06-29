<script lang="ts">
	import './styles.scss';
	import { alert } from '../stores';
	import { download, resetForm } from '../util';
	import { PUBLIC_SERVER_URL } from '$env/static/public';

	import BuyImg from '$lib/images/buy.jpeg';
	import TitleBar from '$lib/components/TitleBar.svelte';

	const SUCCESS_MSG = "Thank you for downloading our buyer's guide!";

	const onSubmit = (e: any) => {
		const formData = new FormData(e.target);
		const body: any = {};

		for (let field of formData) {
			const [key, value] = field;
			body[key] = value;
		}

		return fetch(`${PUBLIC_SERVER_URL}/buyers`, {
			method: 'POST',
			headers: {
				Accept: 'application/json',
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(body)
		})
			.then((resp) => resp.json())
			.then((_) => ($alert = { msg: SUCCESS_MSG, mode: 'success' }))
			.then(() => download(`${PUBLIC_SERVER_URL}/buy.pdf`))
			.then(() => resetForm('buy-form'))
			.catch((err) => {
				console.error(err);
				$alert = { msg: err, mode: 'danger' };
			});
	};
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
		<form on:submit|preventDefault={onSubmit} id="buy-form">
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
				<input id="phone" name="phone" type="tel" />
			</label>
			<div class="full">
				<label for="buy_period">When are you planning to buy?</label>
				<select required id="buy_period" name="buy_period" value={undefined}>
					<option value="Immediately">Immediately</option>
					<option value="Next month">Next Month</option>
					<option value="Within the next three months"> Within the next three months </option>
					<option value="Within the next six months"> Within the next six months </option>
					<option value="Next year">Next year</option>
					<option value="Undecided">Undecided</option>
				</select>
			</div>
			<div class="half">
				<label for="home_type">Home type</label>
				<select required id="home_type" name="home_type" value={undefined}>
					<option value="House">House</option>
					<option value="Condo">Condo</option>
					<option value="Building">Building</option>
					<option value="Rental">Rental</option>
					<option value="Undecided">Undecided</option>
				</select>
			</div>
			<div class="half">
				<label for="bedrooms">Bedrooms</label>
				<select required id="bedrooms" name="bedrooms" value={undefined}>
					<option value="1">1</option>
					<option value="2">2</option>
					<option value="3">3</option>
					<option value="4">4</option>
					<option value="5+">5+</option>
				</select>
			</div>
			<div class="full">
				<label for="location">Location</label>
				<select required id="location" name="location" value={undefined}>
					<option value="Montreal - West Island">Montreal - West Island</option>
					<option value="Montreal - Downtown">Montreal - Downtown</option>
					<option value="Other">Other</option>
					<option value="Undecided">Undecided</option>
				</select>
			</div>
			<label class="full">
				Anything else we should know?
				<textarea id="other" name="other" rows="3" />
			</label>
			<button class="primary" type="submit">Send</button>
		</form>
	</div>
</section>
