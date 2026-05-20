import axios from 'axios'
import { useAuthStore } from '../stores/auth'
import router from '../router'

const api = axios.create({
  baseURL: '/api',
  timeout: 15000,
})

api.interceptors.request.use((config) => {
  const token = localStorage.getItem('token')
  if (token) {
    config.headers.Authorization = `Bearer ${token}`
  }
  return config
})

api.interceptors.response.use(
  (res) => res,
  (err) => {
    if (err.response?.data?.code === 1001) {
      localStorage.removeItem('token')
      router.push('/admin/login')
    }
    return Promise.reject(err)
  },
)

// --- Auth ---
export const authApi = {
  login: (data) => api.post('/auth/login', data),
  me: () => api.get('/auth/me'),
}

// --- Categories ---
export const categoryApi = {
  list: () => api.get('/categories'),
  create: (data) => api.post('/categories', data),
  update: (id, data) => api.put(`/categories/${id}`, data),
  delete: (id) => api.delete(`/categories/${id}`),
}

// --- Tools ---
export const toolApi = {
  list: (params) => api.get('/tools', { params }),
  get: (id) => api.get(`/tools/${id}`),
  create: (data) => api.post('/tools', data),
  update: (id, data) => api.put(`/tools/${id}`, data),
  delete: (id) => api.delete(`/tools/${id}`),
}

// --- Users ---
export const userApi = {
  list: (params) => api.get('/users', { params }),
  update: (id, data) => api.put(`/users/${id}`, data),
  delete: (id) => api.delete(`/users/${id}`),
}

// --- Audit Logs ---
export const auditApi = {
  list: (params) => api.get('/audit-logs', { params }),
}

// --- Scripts ---
export const scriptApi = {
  list: (params) => api.get('/scripts', { params }),
  get: (id) => api.get(`/scripts/${id}`),
  create: (data) => api.post('/scripts', data),
  update: (id, data) => api.put(`/scripts/${id}`, data),
  delete: (id) => api.delete(`/scripts/${id}`),
}

// --- Configs ---
export const configApi = {
  list: (params) => api.get('/configs', { params }),
  get: (id) => api.get(`/configs/${id}`),
  create: (data) => api.post('/configs', data),
  update: (id, data) => api.put(`/configs/${id}`, data),
  delete: (id) => api.delete(`/configs/${id}`),
  copy: (id) => api.post(`/configs/${id}/copy`),
}

// --- System Config ---
export const systemConfigApi = {
  get: () => api.get('/settings/system-config'),
  update: (data) => api.put('/settings/system-config', data),
}

// --- Settings ---
export const settingsApi = {
  getAccessPassword: () => api.get('/settings/access-password'),
  updateAccessPassword: (data) => api.put('/settings/access-password', data),
  verifyAccessPassword: (data) => api.post('/settings/access-password/verify', data),
}

// --- Stats ---
export const statsApi = {
  get: () => api.get('/stats'),
}
