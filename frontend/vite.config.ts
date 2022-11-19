import { sveltekit } from '@sveltejs/kit/vite';
import type { UserConfig } from 'vite';
import basicSsl from '@vitejs/plugin-basic-ssl';

const config: UserConfig = {
  plugins: [sveltekit(), basicSsl()]
};

export default config;
