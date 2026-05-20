<template>
  <MainLayout>
    <div>
      <h2 class="text-2xl font-bold mb-8">{{ t('dashboard.title') }}</h2>

      <div v-if="stats" class="grid grid-cols-2 md:grid-cols-3 gap-4 mb-8">
        <div class="bg-card rounded-xl border border-border p-5">
          <p class="text-sm text-muted-foreground">{{ t('dashboard.statUsers') }}</p>
          <p class="text-2xl font-bold mt-1">{{ stats.users }}</p>
          <p class="text-xs text-muted-foreground mt-1">{{ stats.active_users }} {{ t('adminUsers.active') }}</p>
        </div>
        <div class="bg-card rounded-xl border border-border p-5">
          <p class="text-sm text-muted-foreground">{{ t('dashboard.statCategories') }}</p>
          <p class="text-2xl font-bold mt-1">{{ stats.categories }}</p>
        </div>
        <div class="bg-card rounded-xl border border-border p-5">
          <p class="text-sm text-muted-foreground">{{ t('dashboard.statTools') }}</p>
          <p class="text-2xl font-bold mt-1">{{ stats.tools }}</p>
          <p class="text-xs text-muted-foreground mt-1">{{ stats.featured_tools }} {{ t('adminTools.featured') }}</p>
        </div>
      </div>

      <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-8">
        <router-link to="/admin/tools" class="group bg-card rounded-xl border border-border p-6 hover:bg-muted/50 transition-all cursor-pointer">
          <h3 class="text-lg font-semibold">{{ t('dashboard.tools') }}</h3>
          <p class="text-sm text-muted-foreground mt-1">{{ t('dashboard.toolsDesc') }}</p>
        </router-link>
        <div class="bg-card rounded-xl border border-border p-6 hover:bg-muted/50 transition-all">
          <h3 class="text-lg font-semibold">{{ t('dashboard.settings') }}</h3>
          <p class="text-sm text-muted-foreground mt-1">{{ t('dashboard.settingsDesc') }}</p>
          <div class="mt-4">
            <label class="text-xs text-muted-foreground block mb-1">{{ t('dashboard.accessPassword') }}</label>
            <div class="flex gap-2">
              <input v-model="newPwd" type="password" :placeholder="t('dashboard.accessPasswordPlaceholder')" class="flex-1 px-3 py-1.5 text-sm rounded-md border border-border bg-background focus:outline-none focus:ring-1 focus:ring-primary" />
              <button @click="updatePassword" class="px-3 py-1.5 text-sm bg-primary text-primary-foreground rounded-md hover:opacity-90 transition">{{ t('dashboard.update') }}</button>
            </div>
            <p v-if="pwdMsg" :class="pwdOk ? 'text-green-500' : 'text-red-500'" class="text-xs mt-1">{{ pwdMsg }}</p>
          </div>
        </div>
      </div>

      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <router-link to="/admin/users" class="bg-card rounded-xl border border-border p-6 hover:bg-muted/50 transition-all cursor-pointer">
          <h3 class="text-lg font-semibold">{{ t('dashboard.users') }}</h3>
          <p class="text-sm text-muted-foreground mt-1">{{ t('dashboard.usersDesc') }}</p>
        </router-link>
        <router-link to="/admin/audit" class="bg-card rounded-xl border border-border p-6 hover:bg-muted/50 transition-all cursor-pointer">
          <h3 class="text-lg font-semibold">{{ t('dashboard.audit') }}</h3>
          <p class="text-sm text-muted-foreground mt-1">{{ t('dashboard.auditDesc') }}</p>
        </router-link>
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
  try {
    const res = await statsApi.get()
    stats.value = res.data
  } catch {}
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
