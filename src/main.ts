import { createApp } from "vue";
import App from "./App.vue";


import router from "./router";

//styles
import "./tailwind.css";

//plugins

createApp(App).use(router).mount("#app");
