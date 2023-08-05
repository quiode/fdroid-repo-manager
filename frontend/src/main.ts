import './assets/styles.scss';

// Vue
import { createApp } from 'vue';

// Vuetify
import { createVuetify } from 'vuetify';
import '@mdi/font/css/materialdesignicons.css';

// Pinia
import { createPinia } from 'pinia';

// Custom
import App from './App.vue';
import router from './router';

const app = createApp(App);

const vuetify = createVuetify({
  theme: {
    defaultTheme: 'dark'
  }
});

app.use(createPinia());
app.use(router);
app.use(vuetify);

app.mount('#app');
