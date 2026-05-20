import { createRouter, createWebHistory } from 'vue-router'
import { useAuthStore } from '../stores/auth'

const routes = [
  { path: '/', component: () => import('../views/Home.vue') },
  {
    path: '/admin/login',
    component: () => import('../views/Login.vue'),
    meta: { guest: true },
  },
  {
    path: '/admin',
    component: () => import('../views/Dashboard.vue'),
    meta: { auth: true },
  },
  {
    path: '/admin/tools',
    component: () => import('../views/AdminTools.vue'),
    meta: { auth: true },
  },
  {
    path: '/admin/configs',
    component: () => import('../views/AdminConfigs.vue'),
    meta: { auth: true },
  },
  {
    path: '/admin/system',
    component: () => import('../views/SystemConfig.vue'),
    meta: { auth: true },
  },
  {
    path: '/admin/users',
    component: () => import('../views/AdminUsers.vue'),
    meta: { auth: true },
  },
  {
    path: '/admin/audit',
    component: () => import('../views/AdminAudit.vue'),
    meta: { auth: true },
  },
]

const router = createRouter({
  history: createWebHistory(),
  routes,
})

router.beforeEach((to) => {
  const auth = useAuthStore()
  if (to.meta.auth && !auth.isLoggedIn) {
    return '/admin/login'
  }
  if (to.meta.guest && auth.isLoggedIn) {
    return '/admin'
  }
})

export default router
