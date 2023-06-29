import { writable } from 'svelte/store';

export type Alert = {
	msg: string;
	mode: 'success' | 'danger';
	ms?: number;
};

export const alert = writable<Alert>();
