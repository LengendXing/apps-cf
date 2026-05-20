<template>
  <MainLayout>
    <div>
      <h2 class="text-xl font-bold mb-6">{{ t('adminConfigs.title') }}</h2>
      <div class="flex items-center justify-between mb-4">
        <input v-model="search" :placeholder="t('adminConfigs.search')" class="px-3 py-2 text-sm rounded-lg border border-border bg-background w-64 focus:outline-none focus:ring-2 focus:ring-primary/30" />
        <button @click="editConfig(null)" class="px-4 py-2 text-sm bg-primary text-primary-foreground rounded-lg hover:opacity-90 transition">{{ t('adminConfigs.addConfig') }}</button>
      </div>

      <div class="bg-card rounded-xl border border-border overflow-hidden">
        <table class="w-full text-sm">
          <thead class="bg-muted/50 text-muted-foreground">
            <tr>
              <th class="px-4 py-3 text-left">ID</th>
              <th class="px-4 py-3 text-left">{{ t('adminConfigs.configName') }}</th>
              <th class="px-4 py-3 text-left">{{ t('adminConfigs.configFormat') }}</th>
              <th class="px-4 py-3 text-left">{{ t('adminConfigs.copyCount') }}</th>
              <th class="px-4 py-3 text-right">{{ t('adminTools.delete') }}</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="c in filtered" :key="c.id" class="border-t border-border hover:bg-muted/30">
              <td class="px-4 py-3 text-muted-foreground">{{ c.id }}</td>
              <td class="px-4 py-3 font-medium">{{ c.name }}</td>
              <td class="px-4 py-3"><span class="px-2 py-0.5 text-xs rounded bg-primary/10 text-primary uppercase font-mono">{{ c.format }}</span></td>
              <td class="px-4 py-3">{{ c.copy_count || 0 }}</td>
              <td class="px-4 py-3 text-right">
                <button @click="editConfig(c)" class="px-2 py-1 text-xs rounded bg-muted hover:bg-primary hover:text-primary-foreground transition mr-1">{{ t('adminTools.edit') }}</button>
                <button @click="deleteConfig(c)" class="px-2 py-1 text-xs rounded bg-muted text-red-500 hover:bg-red-500 hover:text-white transition">{{ t('adminTools.delete') }}</button>
              </td>
            </tr>
            <tr v-if="!filtered.length">
              <td colspan="5" class="px-4 py-8 text-center text-muted-foreground">{{ t('home.noConfigs') }}</td>
            </tr>
          </tbody>
        </table>
      </div>

      <!-- Modal -->
      <div v-if="showModal" class="fixed inset-0 z-50 flex items-center justify-center" @click.self="showModal = false">
        <div class="absolute inset-0 bg-black/40" />
        <div class="relative bg-card rounded-2xl border border-border shadow-2xl w-full max-w-2xl max-h-[85vh] flex flex-col z-10">
          <div class="flex items-center justify-between p-4 border-b border-border">
            <h3 class="font-bold">{{ editing ? t('adminConfigs.editConfig') : t('adminConfigs.addConfig') }}</h3>
            <button @click="showModal = false" class="w-6 h-6 rounded-full bg-muted flex items-center justify-center text-xs hover:bg-primary hover:text-primary-foreground">✕</button>
          </div>
          <div class="flex-1 overflow-y-auto p-4 space-y-4">
            <div>
              <label class="text-xs text-muted-foreground mb-1 block">{{ t('adminConfigs.configName') }}</label>
              <input v-model="form.name" class="w-full px-3 py-2 text-sm rounded-lg border border-border bg-background focus:outline-none focus:ring-2 focus:ring-primary/30" />
            </div>
            <div>
              <label class="text-xs text-muted-foreground mb-1 block">{{ t('adminConfigs.configFormat') }}</label>
              <select v-model="form.format" class="w-full px-3 py-2 text-sm rounded-lg border border-border bg-background focus:outline-none focus:ring-2 focus:ring-primary/30">
                <option value="properties">properties</option>
                <option value="yaml">yaml</option>
                <option value="toml">toml</option>
                <option value="json">json</option>
                <option value="text">text</option>
              </select>
            </div>
            <div>
              <label class="text-xs text-muted-foreground mb-1 block">{{ t('adminConfigs.configContent') }}</label>
              <textarea v-model="form.content" rows="14" class="w-full px-3 py-2 text-sm rounded-lg border border-border bg-background font-mono focus:outline-none focus:ring-2 focus:ring-primary/30 leading-relaxed" />
            </div>
          </div>
          <div class="flex justify-end gap-2 p-4 border-t border-border">
            <button @click="showModal = false" class="px-4 py-2 text-sm rounded-lg border border-border hover:bg-muted transition">{{ t('adminConfigs.cancel') }}</button>
            <button @click="saveConfig" class="px-4 py-2 text-sm bg-primary text-primary-foreground rounded-lg hover:opacity-90 transition">{{ t('adminConfigs.save') }}</button>
          </div>
        </div>
      </div>
    </div>
  </MainLayout>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import MainLayout from '../components/MainLayout.vue'
import { configApi } from '../api'

const { t } = useI18n()
const configs = ref([])
const search = ref('')
const showModal = ref(false)
const editing = ref(null)
const form = ref({ name: '', format: 'text', content: '' })

const filtered = computed(() => {
  if (!search.value) return configs.value
  const q = search.value.toLowerCase()
  return configs.value.filter(c => c.name.toLowerCase().includes(q))
})

onMounted(fetch)

async function fetch() {
  try {
    const res = await configApi.list()
    configs.value = res.data?.items || []
  } catch {}
}

function editConfig(c) {
  editing.value = c || null
  form.value = c ? { name: c.name, format: c.format, content: c.content } : { name: '', format: 'text', content: '' }
  showModal.value = true
}

async function saveConfig() {
  try {
    if (editing.value) await configApi.update(editing.value.id, form.value)
    else await configApi.create(form.value)
    showModal.value = false
    fetch()
  } catch {}
}

async function deleteConfig(c) {
  if (!confirm(t('adminConfigs.confirmDelete'))) return
  try { await configApi.delete(c.id); fetch() } catch {}
}
</script>
