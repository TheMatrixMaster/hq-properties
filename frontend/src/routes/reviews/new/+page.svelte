<script lang="ts">
	import Header from '../../Header.svelte';
	import Alert from '$lib/components/Alert.svelte';
	import TitleBar from '$lib/components/TitleBar.svelte';
	import { PUBLIC_SERVER_URL } from '$env/static/public';

	const isRequiredFieldValid = (val: any) => val != null && val !== '';
	const parseFormData = (key: string, val: FormDataEntryValue) => {
		if (key === 'anonymous') return Boolean(val);
		else return val;
	};

	const SUCCESS_MSG: string =
		'Thank you very much! We have received your testimonial and will be publishing it shortly.';
	const ERROR_MSG: string = 'Form input is invalid, please try again.';
	let alert: { msg: string; mode: 'success' | 'danger' } | null = null;

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
			alert = { msg: ERROR_MSG, mode: 'danger' };
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
			.then((_) => (alert = { msg: SUCCESS_MSG, mode: 'success' }))
			.catch((err) => {
				console.error(err);
				alert = { msg: err, mode: 'danger' };
			});
	};
</script>

<svelte:head>
	<title>Testimonials — New</title>
	<meta name="reviews" content="new review" />
</svelte:head>

<Header isTransparent={false} />
<section>
	<Alert {...alert} visible={alert !== null} />
	<TitleBar title="Write a Testimonial" />
	<form on:submit|preventDefault={onSubmit}>
		<label class="half">
			First name
			<input type="text" id="first_name" name="first_name" placeholder="ex: John" value="" />
		</label>
		<label class="half">
			Last name
			<input type="text" id="last_name" name="last_name" placeholder="ex: Smith" value="" />
		</label>
		<label class="full textbox">
			Review
			<textarea id="body" name="body" rows="8" value="" placeholder="Write a review..." />
		</label>
		<div class="full checkbox-container">
			<input type="checkbox" name="anonymous" id="anonymous" value={true} />
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
