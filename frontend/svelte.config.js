import adapter from '@sveltejs/adapter-node'; // FIXME: use appropriate adapter (e.g. static if not using SSR)
import preprocess from 'svelte-preprocess';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	// Consult https://github.com/sveltejs/svelte-preprocess
	// for more information about preprocessors
	preprocess: preprocess({
		postcss: true
	}),

	kit: {
		adapter: adapter()
	}
};

export default config;
