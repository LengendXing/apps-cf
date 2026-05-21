<template>
  <div class="flex h-screen" style="background:var(--bg)">
    <!-- Sidebar -->
    <aside class="flex-shrink-0 w-[220px] flex flex-col h-full" style="background:var(--bg-sidebar);backdrop-filter:blur(40px);-webkit-backdrop-filter:blur(40px);border-right:1px solid var(--divider)">
      <div class="px-5 pt-5 pb-3">
        <div class="flex items-center gap-2.5">
          <div class="w-8 h-8 rounded-apple flex items-center justify-center" style="background:var(--accent)">
            <svg class="w-4 h-4 text-white" viewBox="0 0 24 24" fill="currentColor"><path d="M3 4h18v2H3V4zm0 7h12v2H3v-2zm0 7h18v2H3v-2z"/></svg>
          </div>
          <span class="text-sm font-semibold tracking-tight" style="color:var(--fg)">Apps Admin</span>
        </div>
      </div>

      <nav class="flex-1 px-3 py-2 space-y-0.5">
        <router-link v-for="item in navItems" :key="item.to" :to="item.to"
          class="flex items-center gap-2.5 px-3 py-[7px] rounded-apple text-[13px] font-medium transition-colors"
          :class="isActive(item.to) ? '' : ''"
          :style="isActive(item.to) ? 'background:var(--bg-sidebar-active);color:var(--accent)' : 'color:var(--fg-secondary)'"
        >
          <span class="w-[18px] text-center" v-html="item.icon"></span>
          <span>{{ item.label }}</span>
        </router-link>
      </nav>

      <div class="px-3 pb-3 space-y-0.5">
        <div class="my-2" style="border-top:1px solid var(--divider)"></div>
        <button @click="toggleLocale" class="flex items-center gap-2.5 w-full px-3 py-[7px] rounded-apple text-[13px] font-medium transition-colors hover:opacity-70" style="color:var(--fg-secondary)">
          <span class="w-[18px] text-center">🌐</span>
          <span>{{ locale === 'zh' ? 'English' : '中文' }}</span>
        </button>
        <button @click="toggleDark" class="flex items-center gap-2.5 w-full px-3 py-[7px] rounded-apple text-[13px] font-medium transition-colors hover:opacity-70" style="color:var(--fg-secondary)">
          <span class="w-[18px] text-center">{{ isDark ? '☀️' : '🌙' }}</span>
          <span>{{ isDark ? 'Light' : 'Dark' }}</span>
        </button>
        <button @click="doLogout" class="flex items-center gap-2.5 w-full px-3 py-[7px] rounded-apple text-[13px] font-medium transition-colors hover:opacity-70" style="color:#FF3B30">
          <span class="w-[18px] text-center">⏻</span>
          <span>{{ t('nav.logout') }}</span>
        </button>
      </div>
    </aside>

    <!-- Content -->
    <main class="flex-1 overflow-y-auto">
      <div class="max-w-4xl mx-auto px-8 py-10">
        <slot />
      </div>
    </main>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { useRouter, useRoute } from 'vue-router'
import { useAuthStore } from '../stores/auth'
import { systemConfigApi } from '../api'

const { t, locale } = useI18n()
const router = useRouter()
const route = useRoute()
const auth = useAuthStore()
const isDark = ref(localStorage.getItem('dark') === 'true')

const navItems = computed(() => [
  { to: '/admin', label: t('nav.dashboard'), icon: '📊' },
  { to: '/admin/tools', label: t('nav.tools'), icon: '🔧' },
  { to: '/admin/configs', label: t('nav.configs'), icon: '📝' },
  { to: '/admin/system', label: t('nav.system'), icon: '⚙️' },
  { to: '/admin/users', label: t('nav.users'), icon: '👥' },
  { to: '/admin/audit', label: t('nav.audit'), icon: '📋' },
])

function isActive(path) {
  if (path === '/admin') return route.path === '/admin'
  return route.path.startsWith(path)
}

onMounted(() => {
  if (isDark.value) document.documentElement.classList.add('dark')
})

function toggleDark() {
  isDark.value = !isDark.value
  localStorage.setItem('dark', String(isDark.value))
  document.documentElement.classList.toggle('dark')
}

function toggleLocale() {
  const next = locale.value === 'zh' ? 'en' : 'zh'
  locale.value = next
  localStorage.setItem('locale', next)
}

function doLogout() {
  auth.logout()
  router.push('/admin/login')
}
</script>
