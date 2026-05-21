<template>
  <MainLayout>
    <div>
      <h1 class="text-[28px] font-semibold tracking-tight mb-8" style="color:var(--fg)">{{ t('dashboard.title') }}</h1>

      <div v-if="stats" class="grid grid-cols-3 gap-4 mb-8">
        <div class="apple-card p-5">
          <p class="text-xs font-medium uppercase tracking-wider mb-1" style="color:var(--fg-secondary)">{{ t('dashboard.statUsers') }}</p>
          <p class="text-[32px] font-semibold tracking-tight" style="color:var(--fg)">{{ stats.users }}</p>
          <p class="text-xs mt-1" style="color:var(--fg-tertiary)">{{ stats.active_users }} {{ t('adminUsers.active') }}</p>
        </div>
        <div class="apple-card p-5">
          <p class="text-xs font-medium uppercase tracking-wider mb-1" style="color:var(--fg-secondary)">{{ t('dashboard.statCategories') }}</p>
          <p class="text-[32px] font-semibold tracking-tight" style="color:var(--fg)">{{ stats.categories }}</p>
        </div>
        <div class="apple-card p-5">
          <p class="text-xs font-medium uppercase tracking-wider mb-1" style="color:var(--fg-secondary)">{{ t('dashboard.statTools') }}</p>
          <p class="text-[32px] font-semibold tracking-tight" style="color:var(--fg)">{{ stats.tools }}</p>
          <p class="text-xs mt-1" style="color:var(--fg-tertiary)">{{ stats.featured_tools }} {{ t('adminTools.featured') }}</p>
        </div>
      </div>

      <div class="grid grid-cols-2 gap-4 mb-6">
        <router-link to="/admin/tools" class="apple-card p-6 flex items-start gap-4 hover:shadow-apple-md transition-shadow cursor-pointer group">
          <div class="w-10 h-10 rounded-apple flex items-center justify-center flex-shrink-0" style="background:rgba(0,125,255,0.1)">
            <span class="text-lg">🔧</span>
          </div>
          <div>
            <h3 class="text-[15px] font-semibold mb-0.5" style="color:var(--fg)">{{ t('dashboard.tools') }}</h3>
            <p class="text-xs" style="color:var(--fg-secondary)">{{ t('dashboard.toolsDesc') }}</p>
          </div>
        </router-link>
        <router-link to="/admin/users" class="apple-card p-6 flex items-start gap-4 hover:shadow-apple-md transition-shadow cursor-pointer">
          <div class="w-10 h-10 rounded-apple flex items-center justify-center flex-shrink-0" style="background:rgba(52,199,89,0.1)">
            <span class="text-lg">👥</span>
          </div>
          <div>
            <h3 class="text-[15px] font-semibold mb-0.5" style="color:var(--fg)">{{ t('dashboard.users') }}</h3>
            <p class="text-xs" style="color:var(--fg-secondary)">{{ t('dashboard.usersDesc') }}</p>
          </div>
        </router-link>
      </div>

      <div class="grid grid-cols-2 gap-4 mb-6">
        <router-link to="/admin/audit" class="apple-card p-6 flex items-start gap-4 hover:shadow-apple-md transition-shadow cursor-pointer">
          <div class="w-10 h-10 rounded-apple flex items-center justify-center flex-shrink-0" style="background:rgba(175,82,222,0.1)">
            <span class="text-lg">📋</span>
          </div>
          <div>
            <h3 class="text-[15px] font-semibold mb-0.5" style="color:var(--fg)">{{ t('dashboard.audit') }}</h3>
            <p class="text-xs" style="color:var(--fg-secondary)">{{ t('dashboard.auditDesc') }}</p>
          </div>
        </router-link>
        <div class="apple-card p-6 flex items-start gap-4">
          <div class="w-10 h-10 rounded-apple flex items-center justify-center flex-shrink-0" style="background:rgba(255,149,0,0.1)">
            <span class="text-lg">🔑</span>
          </div>
          <div class="flex-1 min-w-0">
            <h3 class="text-[15px] font-semibold mb-2" style="color:var(--fg)">{{ t('dashboard.accessPassword') }}</h3>
            <div class="flex gap-2">
              <input v-model="newPwd" type="password" :placeholder="t('dashboard.accessPasswordPlaceholder')" class="apple-input flex-1" />
              <button @click="updatePassword" class="apple-btn text-xs">{{ t('dashboard.update') }}</button>
            </div>
            <p v-if="pwdMsg" :class="pwdOk ? 'text-apple-green' : 'text-apple-red'" class="text-xs mt-1.5">{{ pwdMsg }}</p>
          </div>
        </div>
      </div>
    </div>
  </MainLayout>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import MainLayout from '../components/MainLayout.vue'
import { statsApi, settingsApi } from '../api'

const { t } = useI18n()
const stats = ref(null)
const newPwd = ref('')
const pwdMsg = ref('')
const pwdOk = ref(false)

onMounted(async () => {
  try { const res = await statsApi.get(); stats.value = res.data } catch {}
})

async function updatePassword() {
  pwdMsg.value = ''
  if (!newPwd.value) return
  try {
    await settingsApi.updateAccessPassword({ password: newPwd.value })
    pwdMsg.value = t('dashboard.passwordUpdated')
    pwdOk.value = true
    newPwd.value = ''
  } catch (e) {
    pwdMsg.value = e.response?.data?.detail || t('dashboard.updateFailed')
    pwdOk.value = false
  }
}
</script>
