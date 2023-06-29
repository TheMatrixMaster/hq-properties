<script>
	import './styles.scss';
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
				Welcome to the world of real estate excellence with Hong Qu, your dedicated professional
				broker at Royal Lepage. With a seamless transition from a successful 20-year ownership of a
				uniform supply business, Hong's outstanding communication and negotiation abilities have
				already garnered her remarkable success as an agent. A proud West Island resident for over
				15 years, Hong combines her local expertise with a fresh approach to deliver exceptional
				service for your next home transition.
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

<style lang="scss">
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

		& > div {
			display: flex;
			flex-direction: column;
			max-width: 30rem;
		}
		& h1 {
			font-size: 4rem;
			font-weight: bolder;
			margin: 0 0 0.5rem 0;
			text-transform: uppercase;
		}
		& h2 {
			font-weight: 700;
			margin: 0 0 1.5rem 0;
		}
		& p {
			margin: 0;
		}
		& .links {
			display: flex;
			margin-top: 1.5rem;
			flex-direction: column;
		}
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
		gap: 0.5rem;
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
