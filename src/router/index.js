import { createRouter, createWebHistory} from 'vue-router'
import HomeView from '../views/HomeView.vue'

const routes = [
  {
    path: '/',
    name: 'home',
    component: HomeView
  },
  {
    path: '/config',
    name: 'config',
    // route level code-splitting
    // this generates a separate chunk (about.[hash].js) for this route
    // which is lazy-loaded when the route is visited.
    component: () => import('../views/ConfigView.vue')
  
  },
  {
    path: '/pricing',
    name: 'pricing',
    // route level code-splitting
    // this generates a separate chunk (about.[hash].js) for this route
    // which is lazy-loaded when the route is visited.
    component: () => import(/* webpackChunkName: "about" */ '../views/PricingView.vue')
  },
  {
    path: '/hotkeys',
    name: 'hotkeys',
    component: () => import(/* webpackChunkName: "about" */ '../views/HotkeyView.vue')
  },
  {
    path: '/stats',
    name: 'stats',
    // route level code-splitting
    // this generates a separate chunk (about.[hash].js) for this route
    // which is lazy-loaded when the route is visited.
    component: () => import(/* webpackChunkName: "about" */ '../views/StatsView.vue')
  },
]

const router = createRouter({
  history: createWebHistory(process.env.BASE_URL),
  // history: createMemoryHistory(),
  routes
})

export default router
