import { defineConfig } from 'vite';
import vue from '@vitejs/plugin-vue';

export default defineConfig({
  root: '.', // Корень проекта — frontend/
  plugins: [vue()],
  build: {
    outDir: '../static',
    assetsDir: '',
    publicDir: 'public', // Указываем папку public
  },
  server: {
    port: 3001,
  },
});