<template>
  <MainLayout>
    <div>
      <h1 class="text-[28px] font-semibold tracking-tight mb-6" style="color:var(--fg)">{{ t('adminAudit.title') }}</h1>
      <div class="flex gap-3 mb-4">
        <select v-model="actionFilter" @change="page=1;fetch()" class="apple-select">
          <option value="">{{ t('adminAudit.filterAction') }}: {{ t('adminAudit.all') }}</option>
          <option value="create">create</option><option value="update">update</option><option value="delete">delete</option>
        </select>
        <select v-model="typeFilter" @change="page=1;fetch()" class="apple-select">
          <option value="">{{ t('adminAudit.filterType') }}: {{ t('adminAudit.all') }}</option>
          <option value="category">category</option><option value="tool">tool</option>
        </select>
      </div>
      <div class="apple-card overflow-hidden">
        <table class="apple-table">
          <thead>
            <tr><th>ID</th><th>{{ t('adminAudit.userId') }}</th><th>{{ t('adminAudit.action') }}</th><th>{{ t('adminAudit.targetType') }}</th><th>{{ t('adminAudit.targetId') }}</th><th>{{ t('adminAudit.details') }}</th><th>{{ t('adminAudit.time') }}</th></tr>
          </thead>
          <tbody>
            <tr v-for="log in logs" :key="log.id">
              <td style="color:var(--fg-tertiary)">{{ log.id }}</td>
              <td style="color:var(--fg)">{{ log.user_id }}</td>
              <td><span class="inline-flex items-center px-2 py-0.5 rounded-full text-xs font-medium" :style="actionStyle(log.action)">{{ log.action }}</span></td>
              <td style="color:var(--fg-secondary)">{{ log.target_type }}</td>
              <td style="color:var(--fg-tertiary)">{{ log.target_id || '-' }}</td>
              <td style="color:var(--fg-secondary)" class="max-w-xs truncate">{{ parseDetails(log.details) }}</td>
              <td style="color:var(--fg-tertiary)" class="whitespace-nowrap">{{ log.created_at }}</td>
            </tr>
            <tr v-if="!logs.length"><td colspan="7" class="text-center py-12 text-sm" style="color:var(--fg-tertiary)">No logs</td></tr>
          </tbody>
        </table>
      </div>
      <div v-if="totalPages>1" class="flex items-center justify-center gap-1.5 mt-6">
        <button v-for="p in totalPages" :key="p" @click="page=p;fetch()" class="w-8 h-8 rounded-lg text-xs font-medium transition-colors" :style="p===page?'background:var(--accent);color:#fff':'color:var(--fg-secondary)'">{{ p }}</button>
      </div>
    </div>
  </MainLayout>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import MainLayout from '../components/MainLayout.vue'
import { auditApi } from '../api'

const { t } = useI18n()
const logs = ref([])
const page = ref(1)
const total = ref(0)
const actionFilter = ref('')
const typeFilter = ref('')
const pageSize = 20
const totalPages = computed(() => Math.ceil(total.value / pageSize))

onMounted(fetch)

async function fetch() {
  try {
    const params = { page: page.value, page_size: pageSize }
    if (actionFilter.value) params.action = actionFilter.value
    if (typeFilter.value) params.target_type = typeFilter.value
    const res = await auditApi.list(params)
    logs.value = res.data.items || []
    total.value = res.data.total || 0
  } catch {}
}

function actionStyle(action) {
  if (action === 'delete') return 'background:rgba(255,59,48,0.1);color:#FF3B30'
  if (action === 'create') return 'background:rgba(52,199,89,0.1);color:#34C759'
  if (action === 'update') return 'background:rgba(0,125,255,0.1);color:var(--accent)'
  return 'background:var(--bg-sidebar-hover);color:var(--fg-secondary)'
}

function parseDetails(d) { try { return Object.entries(JSON.parse(d)).map(([k,v])=>`${k}: ${v}`).join(', ') } catch { return d } }
</script>
