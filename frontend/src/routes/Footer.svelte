<script lang="ts">
	import { _ } from 'svelte-i18n';
	import Logo from '$lib/images/sl-icon.png';
	import RLLogo from '$lib/images/royal-lepage.svg';

	import PhoneIcon from '$lib/images/phone.svg';
	import EmailIcon from '$lib/images/email.svg';
	import ClockIcon from '$lib/images/clock.svg';
	import MapIcon from '$lib/images/map.svg';

	import FbIcon from '$lib/images/fb.svg';
	import IgIcon from '$lib/images/instagram.svg';
	import LinkedinIcon from '$lib/images/linkedin.svg';
	import WeChatIcon from '$lib/images/wechat.svg';

	import { alert } from '../stores';
	import { resetForm } from '../util';
	import { PUBLIC_SERVER_URL } from '$env/static/public';

	const SUCCESS_MSG: string = $_('thx_message');
	const ERROR_MSG: string = $_('invalid_form');

	const isRequiredFieldValid = (val: any) => val != null && val !== '';

	const onSubmit = (e: any) => {
		const formData = new FormData(e.target);

		let isValid = true;
		const body: any = {};

		for (let field of formData) {
			const [key, value] = field;
			body[key] = value;
			isValid = isValid && isRequiredFieldValid(value);
		}

		if (!isValid) {
			$alert = { msg: ERROR_MSG, mode: 'danger' };
			console.error(ERROR_MSG);
			return;
		}

		return fetch(`${PUBLIC_SERVER_URL}/contact`, {
			method: 'POST',
			headers: {
				Accept: 'application/json',
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(body)
		})
			.then((resp) => resp.json())
			.then((_) => ($alert = { msg: SUCCESS_MSG, mode: 'success' }))
			.then(() => resetForm('contact-form'))
			.catch((err) => {
				console.error(err);
				$alert = { msg: err, mode: 'danger' };
			});
	};
</script>

<footer id="contact">
	<div class="contact-card">
		<div class="left-col">
			<div class="logo-container">
				<div class="logo">
					<h1>Hong Qu</h1>
					<h3>Properties</h3>
				</div>
				<div class="sep" />
				<img alt="royal-lepage" src={RLLogo} />
			</div>
			<div class="contact-container">
				<div class="contact-row">
					<img alt="phone-icon" src={PhoneIcon} />
					<div class="contact-col">
						<h2>{$_('phone_number')}</h2>
						<p>(514) 651-2536</p>
					</div>
				</div>
				<div class="contact-row">
					<img alt="email-icon" src={EmailIcon} />
					<div class="contact-col">
						<h2>{$_('email')}</h2>
						<p>irisqu@royallepage.ca</p>
					</div>
				</div>
				<div class="contact-row">
					<img alt="clock-icon" src={ClockIcon} />
					<div class="contact-col">
						<h2>{$_('open_hours')}</h2>
						<p>24 / 7 / 365</p>
					</div>
				</div>
				<div class="contact-row">
					<img alt="map-icon" src={MapIcon} />
					<div class="contact-col">
						<h2>{$_('address')}</h2>
						<p>6971 ch Côte-de-Liesse, St-Laurent, Quebec H4T1Z3</p>
					</div>
				</div>
			</div>
			<div class="social-container">
				<a href="https://www.facebook.com/irisqu72115" target="_blank">
					<img alt="fb-icon" src={FbIcon} />
				</a>
				<a href="https://www.instagram.com/iris72115" target="_blank">
					<img alt="ig-icon" src={IgIcon} />
				</a>
				<a href="https://www.linkedin.com/in/hong-qu-b68427230" target="_blank">
					<img alt="linkedin-icon" src={LinkedinIcon} />
				</a>
				<a
					href="https://msng.link/o?https://u.wechat.com/kJsIpu0fUkNyHP8vRenVm_8=wc"
					target="_blank"
				>
					<img alt="wechat-icon" src={WeChatIcon} />
				</a>
			</div>
		</div>
		<div class="flat-sep" />
		<div class="right-col">
			<h2>{$_('send_message')}</h2>
			<form on:submit|preventDefault={onSubmit} id="contact-form">
				<label class="full">
					{$_('name')}
					<input required id="name" name="name" type="text" placeholder="ex: John Smith" value="" />
				</label>
				<label class="full">
					{$_('email')}
					<input
						required
						id="email"
						name="email"
						type="email"
						placeholder="ex: john.smith@email.ca"
						value=""
					/>
				</label>
				<label class="full">
					{$_('phone')}
					<input required id="phone" name="phone" type="tel" value="" />
				</label>
				<label class="full">
					{$_('message')}
					<textarea
						required
						id="body"
						name="body"
						rows="5"
						placeholder={$_('write_message')}
						value=""
					/>
				</label>
				<button class="primary" type="submit">{$_('submit_btn')}</button>
			</form>
		</div>
	</div>
	<div class="epilogue">
		<p>
			Made with &#x2665 by <a target="_blank" href="https://matrixmaster.me/">@thematrixmaster</a>
		</p>
		<img alt="sl-logo" src={Logo} />
		<p>Copyright © 2022 | <a href="/">Privacy Policy</a></p>
	</div>
</footer>

<style lang="scss">
	footer {
		display: grid;
		min-height: 100vh;
		padding: 3rem 4rem 1.5rem;
		grid-template-rows: auto fit-content(100%);
		background: var(--color-bg-0);

		@media (orientation: portrait) {
			grid-template-rows: fit-content(100%) auto;
		}
	}
	div.sep {
		height: 100%;
		width: 0.08rem;
		background-color: var(--color-white);
	}
	div.contact-card {
		display: grid;
		grid-template-columns: 50% 50%;

		@media (orientation: portrait) {
			gap: 4rem;
			padding-top: 2rem;
			grid-template-columns: 100%;
		}

		& > div.left-col {
			display: flex;
			flex-direction: column;
			padding: 0 2rem;
			gap: 3rem;
			@media (orientation: portrait) {
				padding: 0;
			}

			& > div.logo-container {
				gap: 1.5rem;
				display: flex;
				flex-direction: row;
				align-items: center;

				& > img {
					width: clamp(10rem, 18vw, 15rem);
					height: auto;
				}
			}
		}
		& > div.flat-sep {
			width: 100%;
			height: 0.06rem;
			background-color: var(--color-white);
			@media (orientation: landscape) {
				display: none;
			}
		}
		& > div.right-col {
			display: flex;
			flex-direction: column;
			padding: 1rem 2rem 0;
			@media (orientation: portrait) {
				padding: 0;
			}

			& > h2 {
				font-size: 2rem;
				margin-bottom: 2rem;
			}
		}
	}
	div.contact-container {
		display: flex;
		flex-direction: column;
		gap: 1rem;

		& img {
			width: 1.8rem;
			height: 1.8rem;
		}
		& h2 {
			text-transform: uppercase;
			margin-bottom: 0.5rem;
		}
		& p {
			font-size: 1.5rem;
			line-height: 1;
			margin: 0;
		}
		& > div.contact-row {
			gap: 1.5rem;
			display: flex;
			flex-direction: row;
			align-items: flex-start;
		}
	}
	div.social-container {
		display: flex;
		flex-direction: row;
		align-items: center;
		gap: 1rem;
		& img {
			height: 2rem;
			width: 2rem;
		}
	}
	div.epilogue {
		display: flex;
		flex-direction: row;
		align-items: flex-end;
		justify-content: space-between;

		& img {
			width: auto;
			height: 4.5rem;
			aspect-ratio: 1;
		}
		& p {
			margin: 0;
			font-size: clamp(0.5rem, 1.5vw, 1.2rem);
		}
		& a {
			font-size: inherit;
			font-weight: bold;
		}
	}
</style>
