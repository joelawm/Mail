import { createApp } from 'vue'
import './index.css'
import App from './App.vue'
import router from './router/index.ts'
import VueClickAway from "vue3-click-away";

// Vue.js
const app = createApp(App);
app.use(VueClickAway)
app.use(router)
app.mount('#app')