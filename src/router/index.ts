import { createMemoryHistory, createRouter } from "vue-router";

import WelcomeView from "../views/welcome/welcome-view.vue";
import LoginView from "../views/auth/login-view.vue";
import SignupView from "../views/auth/sign-up-view.vue";

const routes = [
  { path: "/", component: WelcomeView },
  {
    path: "/login",
    component: LoginView,
    name: "login",
  },
  {
    path: "/sign-up",
    component: SignupView,
    name: "sign-up",
  },
];

const router = createRouter({
  history: createMemoryHistory(),
  routes,
});

export default router;
