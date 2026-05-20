<template>
  <div class="min-h-screen bg-background text-foreground">
    <template v-if="layout === 'top'">
      <header class="sticky top-0 z-50 bg-background/80 backdrop-blur-md border-b border-border">
        <div class="max-w-7xl mx-auto px-6 h-14 flex items-center justify-between">
          <router-link to="/admin" class="text-lg font-bold tracking-tight">Apps Admin</router-link>
          <nav class="flex items-center gap-1">
            <router-link to="/admin" class="px-3 py-1.5 text-sm rounded-md hover:bg-muted transition-colors">{{ t('nav.dashboard') }}</router-link>
            <router-link to="/admin/tools" class="px-3 py-1.5 text-sm rounded-md hover:bg-muted transition-colors">{{ t('nav.tools') }}</router-link>
            <router-link to="/admin/configs" class="px-3 py-1.5 text-sm rounded-md hover:bg-muted transition-colors">{{ t('nav.configs') }}</router-link>
            <router-link to="/admin/system" class="px-3 py-1.5 text-sm rounded-md hover:bg-muted transition-colors">{{ t('nav.system') }}</router-link>
            <router-link to="/admin/users" class="px-3 py-1.5 text-sm rounded-md hover:bg-muted transition-colors">{{ t('nav.users') }}</router-link>
            <router-link to="/admin/audit" class="px-3 py-1.5 text-sm rounded-md hover:bg-muted transition-colors">{{ t('nav.audit') }}</router-link>
          </nav>
          <div class="flex items-center gap-2">
            <button @click="toggleLocale" class="px-2 py-1 text-xs rounded border border-border hover:bg-muted transition-colors">{{ locale === 'zh' ? 'EN' : '中' }}</button>
            <button @click="toggleDark" class="px-2 py-1 text-xs rounded border border-border hover:bg-muted transition-colors">{{ isDark ? '☀' : '☾' }}</button>
            <button @click="doLogout" class="px-2 py-1 text-xs rounded border border-border hover:bg-muted transition-colors">{{ t('nav.logout') }}</button>
          </div>
        </div>
      </header>
      <main class="max-w-7xl mx-auto px-6 py-8">
        <slot />
      </main>
    </template>

    <template v-else>
      <div class="flex h-screen">
        <aside class="flex-shrink-0 w-52 border-r border-border bg-card flex flex-col">
          <div class="p-4 border-b border-border">
            <router-link to="/admin" class="text-lg font-bold tracking-tight">Apps Admin</router-link>
          </div>
          <nav class="flex-1 p-2 space-y-0.5">
            <router-link to="/admin" class="block px-3 py-2 text-sm rounded-md hover:bg-muted transition-colors">{{ t('nav.dashboard') }}</router-link>
            <router-link to="/admin/tools" class="block px-3 py-2 text-sm rounded-md hover:bg-muted transition-colors">{{ t('nav.tools') }}</router-link>
            <router-link to="/admin/configs" class="block px-3 py-2 text-sm rounded-md hover:bg-muted transition-colors">{{ t('nav.configs') }}</router-link>
            <router-link to="/admin/system" class="block px-3 py-2 text-sm rounded-md hover:bg-muted transition-colors">{{ t('nav.system') }}</router-link>
            <router-link to="/admin/users" class="block px-3 py-2 text-sm rounded-md hover:bg-muted transition-colors">{{ t('nav.users') }}</router-link>
            <router-link to="/admin/audit" class="block px-3 py-2 text-sm rounded-md hover:bg-muted transition-colors">{{ t('nav.audit') }}</router-link>
          </nav>
          <div class="p-3 border-t border-border space-y-1">
            <button @click="toggleLocale" class="w-full px-3 py-1.5 text-xs rounded border border-border hover:bg-muted transition-colors text-left">{{ locale === 'zh' ? '中文' : 'English' }}</button>
            <button @click="toggleDark" class="w-full px-3 py-1.5 text-xs rounded border border-border hover:bg-muted transition-colors text-left">{{ isDark ? '☀ Light' : '☾ Dark' }}</button>
            <button @click="doLogout" class="w-full px-3 py-1.5 text-xs rounded border border-border hover:bg-muted transition-colors text-left">{{ t('nav.logout') }}</button>
          </div>
        </aside>
        <main class="flex-1 overflow-y-auto p-8">
          <slot />
        </main>
      </div>
    </template>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { useRouter } from 'vue-router'
import { useAuthStore } from '../stores/auth'
import { systemConfigApi } from '../api'

const { t, locale } = useI18n()
const router = useRouter()
const auth = useAuthStore()
const isDark = ref(localStorage.getItem('dark') === 'true')
const layout = ref(localStorage.getItem('system_layout') || 'left')

onMounted(async () => {
  if (isDark.value) document.documentElement.classList.add('dark')
  try {
    const res = await systemConfigApi.get()
    layout.value = res.data?.layout || 'left'
    localStorage.setItem('system_layout', layout.value)
  } catch {}
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
