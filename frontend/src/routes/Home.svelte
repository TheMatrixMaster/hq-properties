<script lang="ts">
	import './styles.scss';
	import { _ } from 'svelte-i18n';
	import { goto } from '$app/navigation';
	import ArrowRight from '$lib/images/arrow-right.svg';
	import RLLogo from '$lib/images/royal-lepage.svg';
</script>

<section id="intro" class="container-center">
	<div class="left-col">
		<h6>{$_('welcome_msg')}</h6>
		<h1>{$_('brand')}</h1>
		<h6>Royal Lepage du Quartier</h6>
	</div>
	<div class="right-col">
		<h6>{$_('who_am_i')}</h6>
		<h2>{$_('service_areas')}</h2>
		<button class="secondary" on:click={() => goto('/reviews/1')}>
			{$_('review_btn')}
			<img id="icon" alt="icon" src={ArrowRight} />
		</button>
	</div>
	<div class="footer">
		<img alt="royal-lepage" src={RLLogo} />
	</div>
</section>

<style lang="scss">
	#intro {
		min-height: calc(100vh - var(--header-height));
		grid-template-rows: auto fit-content(100%);
		scroll-margin-top: 0;
		@media (orientation: portrait) {
			padding: 0 3rem;
			row-gap: 1rem;
			grid-template-rows: auto auto fit-content(100%);
		}
	}
	#intro .left-col {
		grid-column: 1/7;
		@media (orientation: portrait) {
			align-self: end;
		}
	}
	#intro .right-col {
		grid-column: 7/13;
		display: flex;
		flex-direction: column;
		align-items: flex-end;
		@media (orientation: portrait) {
			align-self: start;
		}
		& > * {
			text-align: right;
		}
	}
	#intro .left-col,
	#intro .right-col {
		@media (orientation: portrait) {
			grid-column: 1/13;
		}
	}
	#intro h6 {
		font-size: 1.8rem;
		font-weight: normal;
		margin: 1rem 0;
	}
	#intro h1 {
		font-size: 4rem;
		margin: 0 0 1.5rem 0;
		letter-spacing: 1.5px;
	}
	#intro h2 {
		font-size: 2.3rem;
		white-space: pre-line;
		margin: 0;
	}
	#intro button {
		gap: 1.5rem;
		margin-top: 2rem;
		font-size: 1.4rem;
		text-align: end;
		white-space: pre-line;
		& > img {
			height: 1.8rem;
		}
	}
	.footer {
		grid-column: 1/13;
		padding-bottom: 1.5rem;
	}
	.footer img {
		width: auto;
		height: 4rem;
	}
</style>
