import { createApp } from "vue";
import App from "./App.vue";

//styles
import "./styles/tailwind.css";

//plugins
import router from "./router";

createApp(App).use(router).mount("#app");
