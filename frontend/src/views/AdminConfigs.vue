<template>
  <MainLayout>
    <div>
      <div class="flex items-center justify-between mb-6">
        <h1 class="text-[28px] font-semibold tracking-tight" style="color:var(--fg)">{{ t('adminConfigs.title') }}</h1>
        <div class="flex gap-2">
          <button @click="showImportInfo=true" class="apple-btn-warn text-xs">{{ t('adminConfigs.importBtn') }}</button>
          <button @click="editConfig(null)" class="apple-btn text-xs">{{ t('adminConfigs.addConfig') }}</button>
        </div>
      </div>

      <div class="flex items-center gap-2 mb-4">
        <input v-model="search" :placeholder="t('adminConfigs.search')" class="apple-input h-[34px]" style="width:224px" />
        <div class="flex-1" />
        <button @click="doExport" class="apple-btn-secondary text-xs h-[34px]">{{ t('adminConfigs.exportBtn') }}</button>
      </div>

      <div class="apple-card overflow-hidden">
        <table class="apple-table">
          <thead>
            <tr><th>ID</th><th>{{ t('adminConfigs.configName') }}</th><th>{{ t('adminConfigs.configFormat') }}</th><th>{{ t('adminConfigs.copyCount') }}</th><th class="text-right">{{ t('adminTools.delete') }}</th></tr>
          </thead>
          <tbody>
            <tr v-for="c in filtered" :key="c.id">
              <td style="color:var(--fg-tertiary)">{{ c.id }}</td>
              <td><span class="font-medium" style="color:var(--fg)">{{ c.name }}</span></td>
              <td><span class="inline-flex items-center px-2 py-0.5 rounded-md text-xs font-mono font-medium" style="background:rgba(0,125,255,0.1);color:var(--accent)">{{ c.format }}</span></td>
              <td style="color:var(--fg-secondary)">{{ c.copy_count || 0 }}</td>
              <td class="text-right">
                <button @click="editConfig(c)" class="apple-btn-secondary text-xs py-1 px-2 mr-1">{{ t('adminTools.edit') }}</button>
                <button @click="confirmDeleteConfig(c)" class="apple-btn-danger text-xs py-1 px-2">{{ t('adminTools.delete') }}</button>
              </td>
            </tr>
            <tr v-if="!filtered.length"><td colspan="5" class="text-center py-12 text-sm" style="color:var(--fg-tertiary)">{{ t('home.noConfigs') }}</td></tr>
          </tbody>
        </table>
      </div>
      <div v-if="totalPages>1" class="flex items-center justify-center gap-1.5 mt-4">
        <button v-for="p in totalPages" :key="p" @click="page=p;fetch()" class="w-8 h-8 rounded-lg text-xs font-medium transition-colors" :style="p===page?'background:var(--accent);color:#fff':'color:var(--fg-secondary)'">{{ p }}</button>
      </div>

      <!-- Config Modal -->
      <Transition name="modal">
        <div v-if="showModal" class="fixed inset-0 z-50 flex items-center justify-center">
          <div class="absolute inset-0 bg-black/30 backdrop-blur-sm" @click="showModal=false" />
          <div class="relative w-full max-w-2xl max-h-[85vh] flex flex-col rounded-apple-xl overflow-hidden" style="background:var(--bg-card);box-shadow:0 24px 80px rgba(0,0,0,0.25)">
            <div class="flex items-center justify-between px-6 py-4" style="border-bottom:1px solid var(--divider)">
              <h3 class="text-[15px] font-semibold" style="color:var(--fg)">{{ editing ? t('adminConfigs.editConfig') : t('adminConfigs.addConfig') }}</h3>
              <button @click="showModal=false" class="w-6 h-6 rounded-full flex items-center justify-center text-xs transition-colors hover:opacity-60" style="background:var(--bg-sidebar-hover);color:var(--fg-secondary)">✕</button>
            </div>
            <div class="flex-1 overflow-y-auto p-6 space-y-4">
              <div><label class="text-xs block mb-1" style="color:var(--fg-secondary)">{{ t('adminConfigs.configName') }}</label><input v-model="form.name" class="apple-input" /></div>
              <div><label class="text-xs block mb-1" style="color:var(--fg-secondary)">{{ t('adminConfigs.configFormat') }}</label><select v-model="form.format" class="apple-select w-full"><option value="properties">properties</option><option value="yaml">yaml</option><option value="toml">toml</option><option value="json">json</option><option value="text">text</option></select></div>
              <div><label class="text-xs block mb-1" style="color:var(--fg-secondary)">{{ t('adminConfigs.configContent') }}</label><textarea v-model="form.content" rows="14" class="apple-input font-mono leading-relaxed" /></div>
            </div>
            <div class="flex justify-end gap-2 px-6 py-4" style="border-top:1px solid var(--divider)">
              <button @click="showModal=false" class="apple-btn-secondary">{{ t('adminConfigs.cancel') }}</button>
              <button @click="saveConfig" class="apple-btn">{{ t('adminConfigs.save') }}</button>
            </div>
          </div>
        </div>
      </Transition>

      <!-- Import Info Modal -->
      <Transition name="modal">
        <div v-if="showImportInfo" class="fixed inset-0 z-50 flex items-center justify-center">
          <div class="absolute inset-0 bg-black/30 backdrop-blur-sm" @click="showImportInfo=false" />
          <div class="relative w-full max-w-lg rounded-apple-xl overflow-hidden" style="background:var(--bg-card);box-shadow:0 24px 80px rgba(0,0,0,0.25)">
            <div class="px-6 py-4" style="border-bottom:1px solid var(--divider)">
              <h3 class="text-[15px] font-semibold" style="color:var(--fg)">{{ t('adminConfigs.importBtn') }}</h3>
            </div>
            <div class="p-6">
              <p class="text-sm mb-3" style="color:var(--fg-secondary)">{{ t('adminConfigs.importDesc') }}</p>
              <pre class="text-xs p-3 rounded-lg overflow-x-auto font-mono leading-relaxed" style="background:var(--bg-sidebar-hover);color:var(--fg)">{{ importFormatExample }}</pre>
            </div>
            <div class="flex justify-end gap-2 px-6 py-4" style="border-top:1px solid var(--divider)">
              <button @click="showImportInfo=false" class="apple-btn-secondary">{{ t('adminConfigs.cancel') }}</button>
              <label class="apple-btn text-xs cursor-pointer">{{ t('adminConfigs.chooseFile') }}
                <input type="file" accept=".json" class="hidden" @change="doImport" />
              </label>
            </div>
          </div>
        </div>
      </Transition>

      <!-- macOS Confirm Dialog -->
      <Transition name="modal">
        <div v-if="macDialog.show" class="mac-dialog-overlay">
          <div class="mac-dialog-backdrop" @click="macDialog.show=false;macDialog.resolve(false)" />
          <div class="mac-dialog">
            <h4>{{ macDialog.title }}</h4>
            <p>{{ macDialog.message }}</p>
            <div class="mac-dialog-actions">
              <button class="mac-dialog-cancel" @click="macDialog.show=false;macDialog.resolve(false)">{{ t('adminConfigs.cancel') }}</button>
              <button class="mac-dialog-ok" style="background:#FF3B30" @click="macDialog.show=false;macDialog.resolve(true)">{{ t('adminTools.delete') }}</button>
            </div>
          </div>
        </div>
      </Transition>
    </div>
  </MainLayout>
</template>

<script setup>
import { ref, computed, reactive, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import MainLayout from '../components/MainLayout.vue'
import { configApi, importExportApi } from '../api'

const { t } = useI18n()
const configs = ref([])
const search = ref('')
const showModal = ref(false)
const editing = ref(null)
const form = ref({ name: '', format: 'text', content: '' })
const page = ref(1)
const total = ref(0)
const pageSize = 20
const totalPages = computed(() => Math.ceil(total.value / pageSize))
const showImportInfo = ref(false)

const importFormatExample = JSON.stringify({
  categories: [{ name: "Category", icon: "📦", sort_order: 0 }],
  tools: [{ name: "Tool", url: "https://...", category_id: 1, icon: "🔧", description: "desc", tags: ["tag1"], platforms: ["mac"], versions: [{ version: "1.0", url: "https://..." }] }],
  configs: [{ name: "Config", format: "json", content: "..." }]
}, null, 2)

const macDialog = reactive({ show: false, title: '', message: '', resolve: () => {} })
function macConfirm(title, message) {
  return new Promise(resolve => { Object.assign(macDialog, { show: true, title, message, resolve }) })
}

const filtered = computed(() => { if (!search.value) return configs.value; const q = search.value.toLowerCase(); return configs.value.filter(c => c.name.toLowerCase().includes(q)) })

onMounted(fetch)

async function fetch() { try { const res = await configApi.list({ page: page.value, page_size: pageSize }); configs.value = res.data?.items || []; total.value = res.data?.total || 0 } catch {} }

function editConfig(c) { editing.value = c || null; form.value = c ? { name: c.name, format: c.format, content: c.content } : { name: '', format: 'text', content: '' }; showModal.value = true }

async function saveConfig() { try { if (editing.value) await configApi.update(editing.value.id, form.value); else await configApi.create(form.value); showModal.value = false; fetch() } catch {} }

async function confirmDeleteConfig(c) {
  const ok = await macConfirm(t('adminConfigs.confirmDelete'), t('adminConfigs.confirmDelete'))
  if (!ok) return
  try { await configApi.delete(c.id); fetch() } catch {}
}

async function doExport() {
  try {
    const res = await importExportApi.exportAll()
    const blob = new Blob([JSON.stringify(res.data, null, 2)], { type: 'application/json' })
    const url = URL.createObjectURL(blob)
    const a = document.createElement('a'); a.href = url; a.download = 'apps-export.json'; a.click()
    URL.revokeObjectURL(url)
  } catch {}
}

async function doImport(event) {
  const file = event.target.files?.[0]
  if (!file) return
  try {
    const text = await file.text()
    const data = JSON.parse(text)
    await importExportApi.importAll(data)
    showImportInfo.value = false
    fetch()
  } catch (e) { alert('Import failed: ' + (e.message || 'Invalid JSON')) }
  event.target.value = ''
}
</script>
