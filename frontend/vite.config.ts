import { sveltekit } from '@sveltejs/kit/vite';
import type { UserConfig } from 'vite';
import basicSsl from '@vitejs/plugin-basic-ssl';
import Icons from 'unplugin-icons/vite';
import { FileSystemIconLoader } from 'unplugin-icons/loaders';

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
  ]
};
export default config;
