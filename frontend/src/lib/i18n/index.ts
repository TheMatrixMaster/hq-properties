import { browser } from '$app/environment';
import { register, init } from 'svelte-i18n';

export const LOCALES: any = {
	en: 'English',
	fr: 'Français'
	// "zh": "中文"
};

const defaultLocale = 'en';

register('en', () => import('./locales/en.json'));
register('fr', () => import('./locales/fr.json'));
// register('zh', () => import('./locales/zh.json'));

const getInitialLocale = () => {
	const supported = ['en', 'fr', 'zh'];
	let res = defaultLocale;

	if (browser) {
		const lang = window.navigator.language;
		supported.forEach((sup) => {
			if (lang.startsWith(sup)) {
				res = sup;
				return res;
			}
		});
	}

	return res;
};

init({
	fallbackLocale: 'en',
	initialLocale: getInitialLocale() ?? defaultLocale
});
