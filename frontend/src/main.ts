import './assets/styles.scss';

// Vue
import { createApp } from 'vue';

// Vuetify
import 'vuetify/styles';
import { createVuetify } from 'vuetify';

// Pinia
import { createPinia } from 'pinia';

// Custom
import App from './App.vue';
import router from './router';

const app = createApp(App);

const vuetify = createVuetify();

app.use(createPinia());
app.use(router);
app.use(vuetify);

app.mount('#app');
