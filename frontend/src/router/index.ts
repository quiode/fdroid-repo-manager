import { createRouter, createWebHistory } from 'vue-router'
import HomeView from '../views/HomeView.vue'
import LoginView from '@/views/LoginView.vue';
import { useAuthStore } from '@/stores/auth';


const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: HomeView
    },
    {
      path: '/login',
      name: 'login',
      component: LoginView,
    },
    { path: '/:pathMatch(.*)*', redirect: '/' },
  ]
});

// auth guard
router.beforeEach((to, from) => {
  const auth = useAuthStore();
  if (auth.authenticated) {
    if (to.name == 'login') {
      return false;
    } else {
      return true;
    }
  } else {
    if (to.name == 'login') {
      return true;
    } else {
      router.push({name: 'login'});
      return false;
    }
  }
});

export default router
