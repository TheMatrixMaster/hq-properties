<script lang="ts">
	import './styles.scss';
	import { _ } from 'svelte-i18n';
	import Alert from '$lib/components/Alert.svelte';
	import Footer from './Footer.svelte';
	import { PUBLIC_ANALYTICS_ID } from '$env/static/public';
</script>

<svelte:head>
	<title>{$_('brand')}</title>

	<script async src={`https://www.googletagmanager.com/gtag/js?id=${PUBLIC_ANALYTICS_ID}`}></script>
	<script lang="ts">
		window.dataLayer = window.dataLayer || [];
		function gtag() {
			dataLayer.push(arguments);
		}
		gtag('js', new Date());
		gtag('config', PUBLIC_ANALYTICS_ID);
	</script>
</svelte:head>

<div class="app">
	<main>
		<Alert />
		<slot />
	</main>
	<Footer />
</div>

<style lang="scss">
	.app {
		display: flex;
		flex-direction: column;
		min-height: 100vh;
	}

	main {
		flex: 1;
		display: flex;
		flex-direction: column;
		box-sizing: border-box;
		height: auto;
	}
</style>
