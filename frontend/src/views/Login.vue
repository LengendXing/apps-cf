<template>
  <div :class="bgClass" class="h-screen flex items-center justify-center transition-colors duration-700">
    <div class="w-full max-w-sm mx-auto p-8">
      <div class="bg-card rounded-2xl border border-border p-10 shadow-sm text-center">
        <div class="w-16 h-16 mx-auto mb-6 rounded-2xl bg-muted flex items-center justify-center">
          <svg class="w-8 h-8 text-foreground" viewBox="0 0 24 24" fill="currentColor">
            <path d="M12 2C6.477 2 2 6.477 2 12c0 4.42 2.865 8.166 6.839 9.489.5.092.682-.217.682-.482 0-.237-.009-.866-.013-1.7-2.782.603-3.369-1.342-3.369-1.342-.454-1.155-1.11-1.462-1.11-1.462-.908-.62.069-.608.069-.608 1.003.07 1.531 1.03 1.531 1.03.892 1.529 2.341 1.087 2.91.831.092-.646.35-1.086.636-1.336-2.22-.253-4.555-1.11-4.555-4.943 0-1.091.39-1.984 1.029-2.683-.103-.253-.446-1.27.098-2.647 0 0 .84-.269 2.75 1.025A9.578 9.578 0 0112 6.836c.85.004 1.705.114 2.504.336 1.909-1.294 2.747-1.025 2.747-1.025.546 1.377.203 2.394.1 2.647.64.699 1.028 1.592 1.028 2.683 0 3.842-2.339 4.687-4.566 4.935.359.309.678.919.678 1.852 0 1.336-.012 2.415-.012 2.743 0 .267.18.578.688.48C19.138 20.161 22 16.416 22 12c0-5.523-4.477-10-10-10z"/>
          </svg>
        </div>
        <h2 class="text-2xl font-bold mb-2">Apps Admin</h2>
        <p class="text-sm text-muted-foreground mb-8">{{ t('login.hint') }}</p>
        <a href="/api/auth/github" class="block w-full py-3 bg-foreground text-background rounded-xl hover:opacity-90 transition font-medium text-sm">
          {{ t('login.github') }}
        </a>
      </div>
    </div>
  </div>
</template>

<script setup>
import { computed, onMounted, onUnmounted, ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { useRoute, useRouter } from 'vue-router'
import { useAuthStore } from '../stores/auth'

const { t } = useI18n()
const route = useRoute()
const router = useRouter()
const auth = useAuthStore()

const isDaytime = ref(true)
let timer = null

function checkBeijingTime() {
  const now = new Date()
  const utcHours = now.getUTCHours()
  const bjHour = (utcHours + 8) % 24
  isDaytime.value = bjHour >= 6 && bjHour < 18
}

const bgClass = computed(() => {
  if (isDaytime.value) {
    return 'bg-white text-black'
  }
  return 'bg-black text-white dark'
})

onMounted(() => {
  checkBeijingTime()
  timer = setInterval(checkBeijingTime, 60000)

  if (route.query.token) {
    auth.setToken(route.query.token)
    router.replace('/admin')
  }

  if (auth.isLoggedIn) {
    router.replace('/admin')
  }
})

onUnmounted(() => {
  if (timer) clearInterval(timer)
})
</script>
