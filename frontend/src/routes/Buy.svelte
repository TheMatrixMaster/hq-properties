<script lang="ts">
	import './styles.scss';
	import { _ } from 'svelte-i18n';
	import { alert } from '../stores';
	import { download, resetForm } from '../util';
	import { PUBLIC_SERVER_URL } from '$env/static/public';

	import BuyImg from '$lib/images/buy.jpeg';
	import TitleBar from '$lib/components/TitleBar.svelte';

	const SUCCESS_MSG = $_('thx_download', { values: { user: "buyer's" } });

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
			.then(() => download(`${PUBLIC_SERVER_URL}/files/buy.pdf`))
			.then(() => resetForm('buy-form'))
			.catch((err) => {
				console.error(err);
				$alert = { msg: err, mode: 'danger' };
			});
	};
</script>

<section id="buy" class="buy-sell container-center">
	<TitleBar title={$_('buyer_title')} />
	<div class="left-col">
		<img alt="buy-home" src={BuyImg} />
		<p>{$_('buyer_readme')}</p>
	</div>
	<div class="right-col">
		<form on:submit|preventDefault={onSubmit} id="buy-form">
			<label class="half">
				{$_('first_name')}
				<input required id="first_name" name="first_name" type="text" placeholder="John" />
			</label>
			<label class="half">
				{$_('last_name')}
				<input required id="last_name" name="last_name" type="text" placeholder="Smith" />
			</label>
			<label class="half">
				{$_('email')}
				<input required id="email" name="email" type="email" placeholder="john.smith@email.com" />
			</label>
			<label class="half">
				{$_('phone')}
				<input id="phone" name="phone" type="tel" />
			</label>
			<div class="full">
				<label for="buy_period">{$_('when_to_buy')}</label>
				<select required id="buy_period" name="buy_period" value={undefined}>
					<option value="Immediately">{$_('when_immediately')}</option>
					<option value="Next month">{$_('when_next_month')}</option>
					<option value="Within the next three months">{$_('when_within_3')}</option>
					<option value="Within the next six months">{$_('when_within_6')}</option>
					<option value="Next year">{$_('when_next_year')}</option>
					<option value="Undecided">{$_('undecided')}</option>
				</select>
			</div>
			<div class="half">
				<label for="home_type">{$_('home_type')}</label>
				<select required id="home_type" name="home_type" value={undefined}>
					<option value="House">{$_('home_type_house')}</option>
					<option value="Condo">{$_('home_type_condo')}</option>
					<option value="Building">{$_('home_type_building')}</option>
					<option value="Rental">{$_('home_type_rental')}</option>
					<option value="Undecided">{$_('undecided')}</option>
				</select>
			</div>
			<div class="half">
				<label for="bedrooms">{$_('bedrooms')}</label>
				<select required id="bedrooms" name="bedrooms" value={undefined}>
					<option value="1">1</option>
					<option value="2">2</option>
					<option value="3">3</option>
					<option value="4">4</option>
					<option value="5+">5+</option>
				</select>
			</div>
			<div class="full">
				<label for="location">{$_('location')}</label>
				<select required id="location" name="location" value={undefined}>
					<option value="Montreal - West Island">{$_('location_west_island')}</option>
					<option value="Montreal - Downtown">{$_('location_downtown')}</option>
					<option value="Laval">{$_('location_laval')}</option>
					<option value="Other">{$_('other')}</option>
					<option value="Undecided">{$_('undecided')}</option>
				</select>
			</div>
			<label class="full">
				{$_('anything_else')}
				<textarea id="other" name="other" rows="3" />
			</label>
			<button class="primary" type="submit">{$_('send_btn')}</button>
		</form>
	</div>
</section>
