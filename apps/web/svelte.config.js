import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
const config = {
  preprocess: vitePreprocess(),
  kit: {
    // Static output: served by Tauri on desktop and by any static host on the web.
    adapter: adapter({ fallback: 'index.html' }),
  },
};

export default config;
