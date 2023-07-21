<script lang="ts">
	import { LOCALES } from '$lib/i18n';
	import { openSideBar } from '../stores';
	import { _, locale, locales } from 'svelte-i18n';
	import HamburgerIcon from '$lib/images/menu.svg';

	export let isTransparent = true;
	export let duration = '300ms';
	export let offset = 50;
	export let tolerance = 5;

	let useMini: boolean = false;
	let y: number = 0;
	let lastY: number = 0;

	function deriveHeader(y: number, dy: number) {
		if (y < offset) return false;
		if (Math.abs(dy) <= tolerance) return useMini;
		// if (dy < 0) return false;
		return true;
	}

	function updateHeader(y: number) {
		const dy = lastY - y;
		lastY = y;
		return deriveHeader(y, dy);
	}

	function setTransitionDuration(node: any) {
		node.style.transitionDuration = duration;
	}

	$: useMini = updateHeader(y);
	$: headerClass =
		useMini && isTransparent
			? 'mini transparent'
			: useMini
			? 'mini'
			: isTransparent
			? 'transparent'
			: '';
</script>

<svelte:window bind:scrollY={y} />

<header id="header" class={headerClass} use:setTransitionDuration>
	<a class="logo" href="/">
		{#if useMini}
			<h1>HQ</h1>
		{:else}
			<h1>Hong Qu</h1>
			<h3>Properties</h3>
		{/if}
	</a>

	<nav>
		<ul>
			<li>
				<a href="/#listings">{$_('listings')}</a>
			</li>
			<li>
				<a href="/#buy">{$_('buy')}</a>
			</li>
			<li>
				<a href="/#sell">{$_('sell')}</a>
			</li>
			<li>
				<a href="/#reviews">{$_('testimonials')}</a>
			</li>
			<li>
				<a href="/#media">{$_('media')}</a>
			</li>
			<li>
				<a href="/#contact">{$_('contact')}</a>
			</li>
			<li>
				<select bind:value={$locale}>
					{#each $locales as loc}
						<option value={loc}>{LOCALES[loc] ?? loc}</option>
					{/each}
				</select>
			</li>
		</ul>
	</nav>

	<button class="sidebar-btn" on:click={() => ($openSideBar = true)}>
		<img src={HamburgerIcon} alt="sidebar-btn" />
	</button>
</header>

<style lang="scss">
	header {
		top: 0;
		display: flex;
		padding: 0 3rem;
		position: sticky;
		align-content: center;
		width: -webkit-fill-available;
		justify-content: space-between;
		z-index: 2;
		background-color: var(--color-bg-0);

		height: var(--header-height);
	}
	.transparent {
		background-color: transparent;
	}
	.mini {
		height: var(--header-mini-height);
		background-color: var(--color-bg-0);
	}
	a.logo:hover {
		color: unset;
		text-decoration: unset;
	}
	nav {
		display: flex;
		justify-content: center;
		--background: rgba(255, 255, 255, 0.7);

		@media (orientation: portrait), (orientation: landscape) and (aspect-ratio < 1.55) {
			display: none;
		}
	}
	ul {
		position: relative;
		padding: 0;
		margin: 0;
		display: flex;
		justify-content: center;
		align-items: center;
		list-style: none;
	}
	li {
		scroll-behavior: smooth;
		position: relative;
		height: 100%;

		& > select {
			border: none;
		}
	}
	nav a,
	nav select {
		display: flex;
		height: 100%;
		align-items: center;
		padding: 0 0 0 6vw;
		color: var(--color-text);
		font-weight: bold;
		font-size: 1rem;
		text-decoration: none;
		scroll-behavior: smooth;
	}
	button.sidebar-btn {
		all: unset;
		& > img {
			width: 2.5rem;
			height: 2.5rem;
		}
		@media (orientation: landscape) and (aspect-ratio >= 1.55) {
			display: none;
		}
	}
	a:hover {
		text-decoration: underline;
	}
</style>
