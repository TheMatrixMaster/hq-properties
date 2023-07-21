<script lang="ts">
	import './styles.scss';
	import { _ } from 'svelte-i18n';
	import { alert } from '../stores';
	import { download, resetForm } from '../util';
	import { PUBLIC_SERVER_URL } from '$env/static/public';

	import SellImg from '$lib/images/sell.jpeg';
	import TitleBar from '$lib/components/TitleBar.svelte';

	const SUCCESS_MSG: string = $_('thx_download', { values: { user: "seller's" } });

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
			.then(() => download(`${PUBLIC_SERVER_URL}/files/sell.pdf`))
			.then(() => resetForm('sell-form'))
			.catch((err) => {
				console.error(err);
				$alert = { msg: err, mode: 'danger' };
			});
	};
</script>

<section id="sell" class="buy-sell container-center">
	<TitleBar title={$_('seller_title')} />
	<div class="left-col">
		<img alt="sell-home" src={SellImg} />
		<p>{$_('seller_readme')}</p>
	</div>
	<div class="right-col">
		<form on:submit|preventDefault={onSubmit} id="sell-form">
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
				<input required id="phone" name="phone" type="tel" />
			</label>
			<label class="full">
				{$_('home_address')}
				<input required id="address" name="address" type="text" autocomplete="street-address" />
			</label>
			<label class="half">
				{$_('postal_code')} ({$_('optional')})
				<input id="postal_code" name="postal_code" type="text" autocomplete="postal-code" />
			</label>
			<label class="half">
				{$_('city')}
				<input required id="city" name="city" type="text" autocomplete="address-level2" />
			</label>
			<div class="full">
				<label for="sell_period">{$_('when_to_sell')}</label>
				<select required id="sell_period" name="sell_period" value={undefined}>
					<option value="Immediately">{$_('when_immediately')}</option>
					<option value="Next month">{$_('when_next_month')}</option>
					<option value="Within the next three months">{$_('when_within_3')}</option>
					<option value="Within the next six months">{$_('when_within_6')}</option>
					<option value="Next year">{$_('when_next_year')}</option>
					<option value="Undecided">{$_('undecided')}</option>
				</select>
			</div>
			<label class="full">
				{$_('anything_else')}
				<textarea id="other" name="other" rows="2" />
			</label>
			<button class="primary" type="submit">{$_('send_btn')}</button>
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
