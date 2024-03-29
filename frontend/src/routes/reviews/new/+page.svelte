<script lang="ts">
	import { _ } from 'svelte-i18n';
	import { alert } from '../../../stores';
	import { resetForm } from '../../../util';
	import { PUBLIC_SERVER_URL } from '$env/static/public';

	import Header from '../../Header.svelte';
	import TitleBar from '$lib/components/TitleBar.svelte';

	const isRequiredFieldValid = (val: any) => val != null && val !== '';
	const parseFormData = (key: string, val: File | string) => {
		if (key === 'anonymous') return Boolean(val);
		else return val;
	};

	const SUCCESS_MSG: string = $_('thx_writing');
	const ERROR_MSG: string = $_('invalid_form');

	const onSubmit = (e: any) => {
		const formData = new FormData(e.target);

		let isValid = true;
		const body: any = { anonymous: false };

		for (let field of formData) {
			const [key, value] = field;
			body[key] = parseFormData(key, value);
			isValid = isValid && isRequiredFieldValid(value);
		}

		if (!isValid) {
			$alert = { msg: ERROR_MSG, mode: 'danger' };
			console.error(ERROR_MSG);
			return;
		}

		return fetch(`${PUBLIC_SERVER_URL}/reviews`, {
			method: 'POST',
			headers: {
				Accept: 'application/json',
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(body)
		})
			.then((resp) => resp.json())
			.then((_) => ($alert = { msg: SUCCESS_MSG, mode: 'success' }))
			.then(() => resetForm('new-review-form'))
			.catch((err) => {
				console.error(err);
				$alert = { msg: err, mode: 'danger' };
			});
	};
</script>

<svelte:head>
	<title>{$_('write_testimonial')}</title>
	<meta name="reviews" content="new review" />
</svelte:head>

<Header isTransparent={false} />
<section>
	<TitleBar title={$_('write_testimonial')} />
	<form on:submit|preventDefault={onSubmit} id="new-review-form">
		<label class="half">
			{$_('first_name')}
			<input
				required
				type="text"
				id="first_name"
				name="first_name"
				placeholder="ex: John"
				value=""
			/>
		</label>
		<label class="half">
			{$_('last_name')}
			<input
				required
				type="text"
				id="last_name"
				name="last_name"
				placeholder="ex: Smith"
				value=""
			/>
		</label>
		<label class="full textbox">
			{$_('review')}
			<textarea required id="body" name="body" rows="8" value="" placeholder={$_('write_review')} />
		</label>
		<div class="full checkbox-container">
			<input type="checkbox" name="anonymous" id="anonymous" value={true} />
			<div class="checkbox-label">
				<label for="anonymous"><strong>{$_('anonymous')}</strong></label>
				{$_('anonymous_desc')}
			</div>
		</div>
		<button class="primary" type="submit">{$_('send_btn')}</button>
	</form>
</section>

<style lang="scss">
	section {
		flex-grow: 1;
		min-height: unset;
		padding-bottom: 3rem;
		height: calc(100vh - var(--header-height));
		grid-template-rows: fit-content(100%) fit-content(100%) auto;
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
