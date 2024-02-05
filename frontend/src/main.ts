import './assets/styles.scss';
import 'bootstrap-icons/font/bootstrap-icons.scss';
// Import all of Bootstrap's JS
import * as bootstrap from 'bootstrap';

import { createApp } from 'vue';
import { createPinia } from 'pinia';

import App from './App.vue';
import router from './router';

const app = createApp(App);

app.use(createPinia());
app.use(router);

app.mount('#app');
