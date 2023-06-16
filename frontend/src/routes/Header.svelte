<script lang="ts">
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
				<a href="/#listings">My Listings</a>
			</li>
			<li>
				<a href="/#buy">Buy</a>
			</li>
			<li>
				<a href="/#sell">Sell</a>
			</li>
			<li>
				<a href="/#reviews">Testimonials</a>
			</li>
			<li>
				<a href="#contact">Contact</a>
			</li>
			<!-- <li>
				<a href="/#media">Media</a>
			</li>
			<li>
				<a href="/">FR | 中文</a>
			</li> -->
		</ul>
	</nav>
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

		@media (orientation: portrait) {
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
	}
	nav a {
		display: flex;
		height: 100%;
		align-items: center;
		padding: 0 0 0 7vw;
		color: var(--color-text);
		font-weight: bold;
		font-size: 1rem;
		text-decoration: none;
		scroll-behavior: smooth;
	}
	a:hover {
		text-decoration: underline;
	}
</style>
