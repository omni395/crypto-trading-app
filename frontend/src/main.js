// Импортируем полифилл для process
import process from './shims/process-shim';

import { createApp } from 'vue';
import App from './App.vue';
import './style.css';

createApp(App).mount('#app');