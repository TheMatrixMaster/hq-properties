export type Listing = {
	id: number;
	city: string;
	address: string;
	listing_url: string;
	bedrooms: number;
	bathrooms: number;
	area: number;
	price: number;
	market_st: 'Sale' | 'Sold' | 'Rent' | 'Expired';
	created_at: string;
	updated_at: string;
};
export type ListingImg = {
	id: number;
	listing_id: number;
	url: string;
	priority: number;
	tag: string;
	created_at: string;
};
export type FullListing = {
	listing: Listing;
	imgs: ListingImg[];
};

const full_listing: FullListing = {
	listing: {
		id: -1,
		city: 'Montreal, QC',
		address: '4847 Bixby Creek Road, Carmel, CA, 93940',
		listing_url: 'https://passerelle.centris.ca/redirect.aspx?CodeDest=ROYALLEPAGE&NoMLS=14918395',
		bedrooms: 4,
		bathrooms: 3,
		area: 8491,
		price: 18950000,
		market_st: 'Sale',
		created_at: '100',
		updated_at: '100'
	},
	imgs: [
		{
			id: -1,
			listing_id: -1,
			url: 'https://ts1.cn.mm.bing.net/th/id/R-C.652b794060c61d86c1f9c94efbc7b6ee?rik=8rmgkkPC6irH1w&pid=ImgRaw&r=0',
			priority: 1,
			tag: 'front',
			created_at: '100'
		}
	]
};

export const dummy_listings: FullListing[] = [
	full_listing,
	full_listing,
	full_listing,
	full_listing,
	full_listing
];

export type Review = {
	id: number;
	first_name: string;
	last_name: string;
	body: string;
	anonymous: boolean;
	published: boolean;
	created_at: string;
};

const review: Review = {
	id: -1,
	first_name: 'John',
	last_name: 'Smith',
	body: 'From start to finish this was a well run operation - \
          much more than just selling the house. Your entire team was incredibly efficient, \
          effective and resourceful! You’ve got a great team of professionals that were responsive, \
          and provided superb guidance during every step of the process. \
          I thoroughly enjoyed working with you and your team. \
          From start to finish this was a well run operation - \
          much more than just selling the house. Your entire team was incredibly efficient, \
          effective and resourceful! You’ve got a great team of professionals that were responsive, \
          and provided superb guidance during every step of the process. \
          I thoroughly enjoyed working with you and your team.',
	anonymous: false,
	published: true,
	created_at: '100'
};

export const dummy_reviews: Review[] = [review, review, review, review, review];
