import { defineStore } from 'pinia'
import { ref, computed } from 'vue'

export const useAuthStore = defineStore('auth', () => {
  const token = ref(localStorage.getItem('token') || '')
  const user = ref(null)
  const isLoggedIn = computed(() => !!token.value)

  function setToken(t) {
    token.value = t
    localStorage.setItem('token', t)
  }

  function setUser(u) {
    user.value = u
  }

  function logout() {
    token.value = ''
    user.value = null
    localStorage.removeItem('token')
  }

  return { token, user, isLoggedIn, setToken, setUser, logout }
})
