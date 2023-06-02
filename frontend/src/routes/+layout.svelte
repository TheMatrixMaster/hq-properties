<script>
	import './styles.css';
	import Footer from './Footer.svelte';
</script>

<div class="app">
	<main>
		<slot />
	</main>
	<Footer />
</div>

<style>
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
