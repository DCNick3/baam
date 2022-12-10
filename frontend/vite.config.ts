import { sveltekit } from '@sveltejs/kit/vite';
import type { UserConfig } from 'vite';
import basicSsl from '@vitejs/plugin-basic-ssl';
import Icons from 'unplugin-icons/vite';
import { FileSystemIconLoader } from 'unplugin-icons/loaders';

const BACKEND_URL = process.env['BACKEND_URL'] ?? 'http://localhost:8080';

const config: UserConfig = {
  plugins: [
    sveltekit(),
    basicSsl(),
    Icons({
      compiler: 'svelte',
      customCollections: {
        baam: FileSystemIconLoader('src/lib/assets/icons')
      }
    })
  ],
  server: {
    proxy: {
      '/api': BACKEND_URL
    }
  },
  build: {
    sourcemap: true
  }
};
export default config;
