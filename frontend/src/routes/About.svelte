<script>
	import './styles.css';
	import profile from '$lib/images/ma.jpeg';
	import PhoneIcon from '$lib/images/phone.svg';
	import EmailIcon from '$lib/images/email.svg';
</script>

<section id="aboutme" class="container-center">
	<div class="left-col">
		<img alt="profile-pic" src={profile} />
	</div>
	<div class="right-col">
		<div>
			<h1>Hong Qu</h1>
			<h2>Certified Real Estate Broker</h2>
			<p>
				Hong has been a consistent top performer with Royal LePage since 1988. She has a wealth of
				experience, and a special empathy and understanding working with older clients who may be
				making the transition from house to retirement living. Her involvement in the local
				community and on a national level through Royal LePage has helped Hong develop an.
			</p>
			<div class="links">
				<div class="social-container">
					<img alt="phone" src={PhoneIcon} />
					<a href="tel::514-651-2536" target="_blank">514-651-2536</a>
				</div>
				<div class="social-container">
					<img alt="mail" src={EmailIcon} />
					<a href="mailto::hong.qu@royallepage.ca" target="_blank">hong.qu@royallepage.ca</a>
				</div>
			</div>
		</div>
	</div>
</section>

<style>
	#aboutme {
		gap: 4rem;
		background: var(--color-bg-0);
	}
	.left-col {
		grid-column: 1/7;
		display: flex;
		justify-content: flex-end;
	}
	.left-col img {
		max-height: 30rem;
		width: auto;
	}
	.right-col {
		display: flex;
		flex-direction: column;
		grid-column: 7/13;
	}
	.right-col div {
		display: flex;
		flex-direction: column;
		max-width: 25rem;
	}
	.right-col h1 {
		font-size: 4rem;
		font-weight: bolder;
		margin: 0 0 0.5rem 0;
		text-transform: uppercase;
	}
	.right-col h2 {
		font-weight: 700;
		margin: 0 0 1.5rem 0;
	}
	.right-col .links {
		display: flex;
		margin-top: 1rem;
		flex-direction: column;
	}
	@media (orientation: portrait) {
		#aboutme {
			gap: unset;
			row-gap: 1.5rem;
			grid-template-rows: auto auto;
		}
		.left-col,
		.right-col {
			grid-column: 1/13;
			justify-content: center;
			align-items: center;
		}
		.left-col img {
			max-width: 30rem;
			height: auto;
			width: 100%;
			padding-top: 2rem;
			max-height: unset;
		}
		.right-col div {
			max-width: 30rem;
		}
	}
	div.links {
		display: flex;
		flex-direction: column;
		gap: .5rem;
	}
	div.social-container {
		display: flex;
		flex-direction: row;
		align-items: center;
		gap: 1rem;
		& > a {
			margin: 0;
		}
		& > img {
			height: 1.5rem;
			width: 1.5rem;
		}
	}
</style>
