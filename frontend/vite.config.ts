import { sveltekit } from '@sveltejs/kit/vite';
import type { UserConfig } from 'vite';
import basicSsl from '@vitejs/plugin-basic-ssl';
import Icons from 'unplugin-icons/vite';
import { FileSystemIconLoader } from 'unplugin-icons/loaders';
import childProcess from 'child_process';

const BACKEND_URL = process.env['BACKEND_URL'] ?? 'http://localhost:8080';

const getGitSHA = (ifShortSHA: boolean) => {
  const { exec } = childProcess;
  const sh = ifShortSHA ? 'git rev-parse --short HEAD' : 'git rev-parse HEAD';

  return new Promise((resolve, reject) => {
    exec(sh, (error, stdout) => {
      if (error) {
        reject(error);
      } else {
        const output = stdout.toString()?.replace('\n', '');
        resolve(output);
      }
    });
  });
};

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
  define: {
    __APP_VERSION__: JSON.stringify(await getGitSHA(false))
  },
  build: {
    sourcemap: true
  },
  test: {
    include: ['src/**/*.{test,spec}.{js,ts}']
  }
};
export default config;
