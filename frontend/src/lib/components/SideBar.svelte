<script lang="ts">
	import { goto } from '$app/navigation';
	import { LOCALES } from '$lib/i18n';
	import { _, locale, locales } from 'svelte-i18n';
	import CloseIcon from '$lib/images/close.svg';
	import { openSideBar } from '../../stores';

	const handleLink = (href: string) => {
		$openSideBar = false;
		goto(href);
	};
</script>

<aside class:open={$openSideBar}>
	<button class="close-btn" on:click={() => ($openSideBar = false)}>
		<img src={CloseIcon} alt="close-sidebar" />
	</button>
	<nav>
		<button on:click={() => handleLink('/#listings')}>{$_('listings')}</button>
		<button on:click={() => handleLink('/#buy')}>{$_('buy')}</button>
		<button on:click={() => handleLink('/#sell')}>{$_('sell')}</button>
		<button on:click={() => handleLink('/#reviews')}>{$_('testimonials')}</button>
		<button on:click={() => handleLink('/#media')}>{$_('media')}</button>
		<button on:click={() => handleLink('/#contact')}>{$_('contact')}</button>
		<select bind:value={$locale}>
			{#each $locales as loc}
				<option value={loc}>{LOCALES[loc] ?? loc}</option>
			{/each}
		</select>
	</nav>
</aside>

<style lang="scss">
	aside {
		display: flex;
		flex-direction: column;
		align-items: flex-end;
		align-items: right;
		right: -100%;
		width: 100vw;
		height: 100vh;
		position: fixed;
		padding: 3rem;
		color: var(--color-text);
		transition: right 0.3s ease-in-out;
		background: var(--color-bg-0);
		z-index: 4;
	}

	nav {
		display: flex;
		flex-direction: column;
		align-items: flex-end;
	}

	button.close-btn {
		padding: 0;
		margin-bottom: 1.5rem;
		& > img {
			width: 3rem;
			height: 3rem;
			color: var(--color-text);
		}
	}

	button,
	select {
		display: flex;
		color: var(--color-text);
		flex-direction: row;
		width: fit-content;
		align-items: center;
		padding: 0.7rem 0rem;
		margin: 0;
		font-weight: bold;
		font-size: 2rem;
		text-decoration: none;
		scroll-behavior: smooth;
		background: none;
		border: none;
	}

	select {
		text-align: end;
		text-decoration: underline;
		-webkit-appearance: none;
		-moz-appearance: none;
		appearance: none;
	}

	.open {
		right: 0;
	}
</style>
