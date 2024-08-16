import { createMemoryHistory, createRouter } from "vue-router";

import WelcomeView from "../views/welcome/welcome-view.vue";

const routes = [{ path: "/", component: WelcomeView }];

const router = createRouter({
  history: createMemoryHistory(),
  routes,
});

export default router;
