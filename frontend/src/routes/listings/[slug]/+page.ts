import { PUBLIC_SERVER_URL } from '$env/static/public';

const LIMIT: number = 6;

export const load = async ({ fetch, params }) => {
	let size: number = 0;
	let listings: any = [];
	let error: string = '';
	let fetching: boolean = true;

	const page: number = parseInt(params.slug) ?? 1;
	const offset: number = (page - 1) * LIMIT;

	await fetch(`${PUBLIC_SERVER_URL}/listings?limit=${LIMIT}&offset=${offset}`)
		.then((resp) => resp.json())
		.then((r) => {
			size = r.size;
			listings = r.data;
		})
		.catch((err) => {
			console.error(err);
			error = err;
		})
		.then(() => (fetching = false));

	return {
		slug: params.slug,
		listings,
		size,
		error,
		fetching,
		LIMIT
	};
};
