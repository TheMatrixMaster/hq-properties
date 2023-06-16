<script>
	import './styles.scss';
	import TitleBar from '$lib/components/TitleBar.svelte';
</script>

<section id="media">
	<TitleBar title="Videos" />
	<TitleBar title="Posts" />
</section>

<style lang="scss">
	#media {
		background: var(--color-bg-0);
	}
</style>
