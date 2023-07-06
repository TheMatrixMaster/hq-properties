import { PUBLIC_SERVER_URL } from '$env/static/public';

const LIMIT: number = 8;

export const load = async ({ fetch, params }) => {
	let fetching: boolean = true;
	let error: string = '';
	let size: number = 0;
	let reviews: any = [];

	const page: number = parseInt(params.slug) ?? 1;
	const offset: number = (page - 1) * LIMIT;

	await fetch(`${PUBLIC_SERVER_URL}/reviews?limit=${LIMIT}&offset=${offset}&published=false`)
		.then((resp) => resp.json())
		.then((r) => {
			size = r.size;
			reviews = r.data;
		})
		.catch((err) => {
			console.error(err);
			error = err;
		})
		.then(() => (fetching = false));

	return {
		slug: params.slug,
		reviews,
		size,
		error,
		fetching,
		LIMIT
	};
};
