import { createApp } from 'vue'
import App from './App.vue'
import router from './router'
import {createPinia} from 'pinia'

const pinia = createPinia();

const app = createApp(App)
    .use(router)
    .use(pinia)
    .mount('#app');

// app.component('Home', Home)