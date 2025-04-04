import { defineConfig } from 'vite';
import vue from '@vitejs/plugin-vue';

export default defineConfig({
  root: '.',
  plugins: [vue()],
  build: {
    outDir: '../static',
    assetsDir: '',
    publicDir: false,
  },
  server: {
    port: 3001,
  },
});