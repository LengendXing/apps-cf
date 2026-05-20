<template>
  <MainLayout>
    <div>
      <h2 class="text-2xl font-bold mb-6">{{ t('adminAudit.title') }}</h2>
      <div class="flex gap-3 mb-4">
        <select v-model="actionFilter" @change="page = 1; fetch()" class="px-3 py-1.5 text-sm rounded-md border border-border bg-background">
          <option value="">{{ t('adminAudit.filterAction') }}: {{ t('adminAudit.all') }}</option>
          <option value="create">create</option>
          <option value="update">update</option>
          <option value="delete">delete</option>
        </select>
        <select v-model="typeFilter" @change="page = 1; fetch()" class="px-3 py-1.5 text-sm rounded-md border border-border bg-background">
          <option value="">{{ t('adminAudit.filterType') }}: {{ t('adminAudit.all') }}</option>
          <option value="category">category</option>
          <option value="tool">tool</option>
        </select>
      </div>
      <div class="bg-card rounded-lg border border-border overflow-hidden">
        <table class="w-full text-sm">
          <thead class="border-b border-border">
            <tr>
              <th class="px-4 py-3 text-left font-medium text-muted-foreground">ID</th>
              <th class="px-4 py-3 text-left font-medium text-muted-foreground">{{ t('adminAudit.userId') }}</th>
              <th class="px-4 py-3 text-left font-medium text-muted-foreground">{{ t('adminAudit.action') }}</th>
              <th class="px-4 py-3 text-left font-medium text-muted-foreground">{{ t('adminAudit.targetType') }}</th>
              <th class="px-4 py-3 text-left font-medium text-muted-foreground">{{ t('adminAudit.targetId') }}</th>
              <th class="px-4 py-3 text-left font-medium text-muted-foreground">{{ t('adminAudit.details') }}</th>
              <th class="px-4 py-3 text-left font-medium text-muted-foreground">{{ t('adminAudit.time') }}</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="log in logs" :key="log.id" class="border-b border-border last:border-0">
              <td class="px-4 py-3 text-muted-foreground">{{ log.id }}</td>
              <td class="px-4 py-3">{{ log.user_id }}</td>
              <td class="px-4 py-3">
                <span :class="actionColor(log.action)">{{ log.action }}</span>
              </td>
              <td class="px-4 py-3 text-muted-foreground">{{ log.target_type }}</td>
              <td class="px-4 py-3 text-muted-foreground">{{ log.target_id || '-' }}</td>
              <td class="px-4 py-3 text-muted-foreground max-w-xs truncate">{{ parseDetails(log.details) }}</td>
              <td class="px-4 py-3 text-muted-foreground whitespace-nowrap">{{ log.created_at }}</td>
            </tr>
          </tbody>
        </table>
      </div>
      <div v-if="totalPages > 1" class="flex items-center justify-center gap-2 mt-4">
        <button v-for="p in totalPages" :key="p" @click="page = p; fetch()" :class="p === page ? 'bg-primary text-primary-foreground' : 'border border-border'" class="px-3 py-1 text-sm rounded-md">{{ p }}</button>
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

function actionColor(action) {
  if (action === 'delete') return 'text-red-400'
  if (action === 'create') return 'text-green-500'
  if (action === 'update') return 'text-blue-400'
  return 'text-muted-foreground'
}

function parseDetails(d) {
  try {
    const obj = JSON.parse(d)
    return Object.entries(obj).map(([k, v]) => `${k}: ${v}`).join(', ')
  } catch { return d }
}
</script>
