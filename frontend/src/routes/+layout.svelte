<script>
	import './styles.scss';
	import Alert from '$lib/components/Alert.svelte';
	import Footer from './Footer.svelte';
</script>

<svelte:head>
	<title>HQ Properties</title>
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
