import { createMemoryHistory, createRouter } from "vue-router";

import WelcomeView from "../views/welcome/welcome-view.vue";
import LoginView from "../views/auth/login-view.vue";

const routes = [{ path: "/", component: WelcomeView, name:"welcome" }, {
  path:"/login", component:LoginView, name:"login"
}];

const router = createRouter({
  history: createMemoryHistory(),
  routes,
});

export default router;
