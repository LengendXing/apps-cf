<template>
  <MainLayout>
    <div>
      <h1 class="text-[28px] font-semibold tracking-tight mb-6" style="color:var(--fg)">{{ t('systemConfig.title') }}</h1>
      <div class="apple-card p-6 max-w-xl">
        <label class="text-[13px] font-medium block mb-3" style="color:var(--fg)">{{ t('systemConfig.menuLayout') }}</label>
        <div class="flex gap-3">
          <button @click="save('left')" class="flex-1 p-4 rounded-apple-lg transition-all" :style="layout==='left'?'background:var(--accent);color:#fff;box-shadow:0 4px 12px rgba(0,125,255,0.3)':'background:var(--bg-sidebar-hover);color:var(--fg-secondary)'">
            <div class="flex gap-2 mb-2 justify-center">
              <div class="w-6 h-12 rounded opacity-40" :style="layout==='left'?'background:rgba(255,255,255,0.3)':'background:var(--divider)'" />
              <div class="flex-1 space-y-1.5"><div class="h-1.5 rounded" :style="layout==='left'?'background:rgba(255,255,255,0.3)':'background:var(--divider)'" /><div class="h-1.5 rounded w-3/4" :style="layout==='left'?'background:rgba(255,255,255,0.3)':'background:var(--divider)'" /><div class="h-1.5 rounded w-1/2" :style="layout==='left'?'background:rgba(255,255,255,0.2)':'background:var(--divider)'" /></div>
            </div>
            <span class="text-xs font-medium">{{ t('systemConfig.layoutLeft') }}</span>
          </button>
          <button @click="save('top')" class="flex-1 p-4 rounded-apple-lg transition-all" :style="layout==='top'?'background:var(--accent);color:#fff;box-shadow:0 4px 12px rgba(0,125,255,0.3)':'background:var(--bg-sidebar-hover);color:var(--fg-secondary)'">
            <div class="space-y-2 mb-2">
              <div class="flex gap-1 justify-center"><div class="h-1.5 w-8 rounded" :style="layout==='top'?'background:rgba(255,255,255,0.3)':'background:var(--divider)'" /><div class="h-1.5 w-8 rounded" :style="layout==='top'?'background:rgba(255,255,255,0.3)':'background:var(--divider)'" /><div class="h-1.5 w-8 rounded" :style="layout==='top'?'background:rgba(255,255,255,0.3)':'background:var(--divider)'" /></div>
              <div class="space-y-1.5"><div class="h-1.5 rounded" :style="layout==='top'?'background:rgba(255,255,255,0.3)':'background:var(--divider)'" /><div class="h-1.5 rounded w-3/4" :style="layout==='top'?'background:rgba(255,255,255,0.2)':'background:var(--divider)'" /></div>
            </div>
            <span class="text-xs font-medium">{{ t('systemConfig.layoutTop') }}</span>
          </button>
        </div>
        <p v-if="msg" :class="ok?'text-apple-green':'text-apple-red'" class="text-xs mt-3">{{ msg }}</p>
      </div>
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

onMounted(async () => { try { const res = await systemConfigApi.get(); layout.value = res.data?.layout || 'left' } catch {} })

async function save(val) {
  layout.value = val; msg.value = ''
  try { await systemConfigApi.update({ layout: val }); msg.value = t('systemConfig.saved'); ok.value = true; localStorage.setItem('system_layout', val) } catch (e) { msg.value = e.response?.data?.message || t('systemConfig.saveFailed'); ok.value = false }
}
</script>
