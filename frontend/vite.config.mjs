import { defineConfig } from 'vite';
import vue from '@vitejs/plugin-vue';
import { createHtmlPlugin } from 'vite-plugin-html';
import vueDevTools from 'vite-plugin-vue-devtools';
import path from 'path'; // Импортируем модуль path

export default defineConfig({
  plugins: [
    vue(),
    vueDevTools(),
    createHtmlPlugin({}),
  ],
  server: {
    host: "127.0.0.1",
    port: 8080,
  },
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'), // Настраиваем алиас @ для src
    },
  },
});