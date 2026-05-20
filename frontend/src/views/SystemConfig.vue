<template>
  <MainLayout>
    <h2 class="text-xl font-bold mb-6">{{ t('systemConfig.title') }}</h2>
    <div class="bg-card rounded-xl border border-border p-6 max-w-xl">
      <div class="mb-6">
        <label class="text-sm font-medium block mb-2">{{ t('systemConfig.menuLayout') }}</label>
        <div class="flex gap-3">
          <button @click="save('left')" :class="layout === 'left' ? 'border-primary bg-primary/5 ring-1 ring-primary' : 'border-border hover:bg-muted'" class="flex-1 p-4 rounded-xl border transition text-center">
            <div class="flex gap-2 mb-2 justify-center">
              <div class="w-8 h-16 bg-muted rounded" />
              <div class="flex-1 space-y-1">
                <div class="h-2 bg-muted rounded w-full" />
                <div class="h-2 bg-muted rounded w-3/4" />
                <div class="h-2 bg-muted rounded w-1/2" />
              </div>
            </div>
            <span class="text-xs text-muted-foreground">{{ t('systemConfig.layoutLeft') }}</span>
          </button>
          <button @click="save('top')" :class="layout === 'top' ? 'border-primary bg-primary/5 ring-1 ring-primary' : 'border-border hover:bg-muted'" class="flex-1 p-4 rounded-xl border transition text-center">
            <div class="space-y-2 mb-2">
              <div class="flex gap-1 justify-center">
                <div class="h-2 w-10 bg-muted rounded" />
                <div class="h-2 w-10 bg-muted rounded" />
                <div class="h-2 w-10 bg-muted rounded" />
              </div>
              <div class="space-y-1">
                <div class="h-2 bg-muted rounded w-full" />
                <div class="h-2 bg-muted rounded w-3/4" />
              </div>
            </div>
            <span class="text-xs text-muted-foreground">{{ t('systemConfig.layoutTop') }}</span>
          </button>
        </div>
      </div>
      <p v-if="msg" :class="ok ? 'text-green-500' : 'text-red-500'" class="text-xs">{{ msg }}</p>
    </div>
  </MainLayout>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import MainLayout from '../components/MainLayout.vue'
import { systemConfigApi } from '../api'

const { t } = useI18n()
const layout = ref('left')
const msg = ref('')
const ok = ref(false)

onMounted(async () => {
  try {
    const res = await systemConfigApi.get()
    layout.value = res.data?.layout || 'left'
  } catch {}
})

async function save(val) {
  layout.value = val
  msg.value = ''
  try {
    await systemConfigApi.update({ layout: val })
    msg.value = t('systemConfig.saved')
    ok.value = true
    localStorage.setItem('system_layout', val)
  } catch (e) {
    msg.value = e.response?.data?.message || t('systemConfig.saveFailed')
    ok.value = false
  }
}
</script>
