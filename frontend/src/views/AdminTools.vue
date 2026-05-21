<template>
  <MainLayout>
    <div>
      <div class="flex items-center justify-between mb-6">
        <h1 class="text-[28px] font-semibold tracking-tight" style="color:var(--fg)">{{ t('adminTools.title') }}</h1>
        <div class="flex gap-2">
          <button @click="editCategory(null)" class="apple-btn-secondary text-xs">{{ t('adminTools.addCategory') }}</button>
          <button @click="editTool(null)" class="apple-btn text-xs">{{ t('adminTools.addTool') }}</button>
        </div>
      </div>

      <!-- Category Form -->
      <div v-if="showCatForm" class="apple-card p-5 mb-6">
        <h3 class="text-[15px] font-semibold mb-3" style="color:var(--fg)">{{ editingCat ? t('adminTools.editCategory') : t('adminTools.categoryForm') }}</h3>
        <div class="flex gap-3 items-end">
          <div class="flex-1"><label class="text-xs block mb-1" style="color:var(--fg-secondary)">{{ t('adminTools.categoryName') }}</label><input v-model="catForm.name" class="apple-input" /></div>
          <div class="w-24"><label class="text-xs block mb-1" style="color:var(--fg-secondary)">{{ t('adminTools.icon') }}</label><input v-model="catForm.icon" class="apple-input" /></div>
          <div class="w-20"><label class="text-xs block mb-1" style="color:var(--fg-secondary)">{{ t('adminTools.sortOrder') }}</label><input v-model.number="catForm.sort_order" type="number" class="apple-input" /></div>
          <button @click="saveCategory" class="apple-btn text-xs">{{ t('adminTools.save') }}</button>
          <button @click="showCatForm = false" class="apple-btn-secondary text-xs">{{ t('adminTools.cancel') }}</button>
        </div>
      </div>

      <!-- Tool Form -->
      <div v-if="showToolForm" class="apple-card p-5 mb-6">
        <h3 class="text-[15px] font-semibold mb-4" style="color:var(--fg)">{{ editingTool ? t('adminTools.editTool') : t('adminTools.toolForm') }}</h3>
        <div class="grid grid-cols-2 gap-3">
          <div><label class="text-xs block mb-1" style="color:var(--fg-secondary)">{{ t('adminTools.toolName') }}</label><input v-model="toolForm.name" class="apple-input" /></div>
          <div><label class="text-xs block mb-1" style="color:var(--fg-secondary)">{{ t('adminTools.categoryName') }}</label><select v-model="toolForm.category_id" class="apple-select w-full"><option :value="0">{{ t('adminTools.noCategory') }}</option><option v-for="c in categories" :key="c.id" :value="c.id">{{ c.name }}</option></select></div>
          <div class="col-span-2"><label class="text-xs block mb-1" style="color:var(--fg-secondary)">{{ t('adminTools.toolUrl') }}</label><input v-model="toolForm.url" class="apple-input" /></div>
          <div><label class="text-xs block mb-1" style="color:var(--fg-secondary)">{{ t('adminTools.icon') }}</label><input v-model="toolForm.icon" class="apple-input" /></div>
          <div><label class="text-xs block mb-1" style="color:var(--fg-secondary)">{{ t('adminTools.description') }}</label><input v-model="toolForm.description" class="apple-input" /></div>
          <div><label class="text-xs block mb-1" style="color:var(--fg-secondary)">{{ t('adminTools.tags') }}</label><input v-model="toolForm.tagsStr" placeholder="tag1,tag2" class="apple-input" /></div>
          <div><label class="text-xs block mb-1" style="color:var(--fg-secondary)">{{ t('adminTools.platforms') }}</label><input v-model="toolForm.platformsStr" placeholder="mac,windows" class="apple-input" /></div>
        </div>
        <div class="mt-4">
          <div class="flex items-center justify-between mb-2">
            <label class="text-xs font-semibold" style="color:var(--fg)">{{ t('adminTools.versions') }}</label>
            <button @click="toolForm.versions.push({ version: '', url: '' })" class="apple-btn-secondary text-xs py-0.5 px-2">{{ t('adminTools.addVersion') }}</button>
          </div>
          <div v-for="(v, i) in toolForm.versions" :key="i" class="flex gap-2 items-center mb-2">
            <span class="text-xs w-4 text-right" style="color:var(--fg-tertiary)">{{ i+1 }}</span>
            <input v-model="v.version" class="apple-input w-32" />
            <input v-model="v.url" class="apple-input flex-1" />
            <button @click="toolForm.versions.splice(i,1)" class="apple-btn-danger text-xs py-0.5 px-2">✕</button>
          </div>
        </div>
        <div class="flex items-center gap-4 mt-4">
          <label class="flex items-center gap-2 text-[13px] cursor-pointer" style="color:var(--fg)">
            <input v-model="toolForm.is_featured" type="checkbox" class="w-4 h-4 rounded accent-[var(--accent)]" /> {{ t('adminTools.featured') }}
          </label>
          <div><label class="text-xs mr-1" style="color:var(--fg-secondary)">{{ t('adminTools.sortOrder') }}</label><input v-model.number="toolForm.sort_order" type="number" class="apple-input w-20" /></div>
          <div class="flex-1" />
          <button @click="saveTool" class="apple-btn text-xs">{{ t('adminTools.save') }}</button>
          <button @click="showToolForm = false" class="apple-btn-secondary text-xs">{{ t('adminTools.cancel') }}</button>
        </div>
      </div>

      <!-- Categories & Tools -->
      <div v-for="cat in categories" :key="cat.id" class="mb-8">
        <div class="flex items-center gap-2 mb-3">
          <h3 class="text-[15px] font-semibold" style="color:var(--fg)">{{ cat.icon }} {{ cat.name }}</h3>
          <span class="text-xs" style="color:var(--fg-tertiary)">({{ getToolsByCat(cat.id).length }})</span>
          <button @click="editCategory(cat)" class="text-xs ml-2 hover:opacity-60" style="color:var(--accent)">{{ t('adminTools.edit') }}</button>
          <button @click="deleteCategory(cat)" class="text-xs hover:opacity-60" style="color:#FF3B30">{{ t('adminTools.delete') }}</button>
        </div>
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3">
          <div v-for="tool in getToolsByCat(cat.id)" :key="tool.id" class="apple-card p-4 flex items-center gap-3 hover:shadow-apple-md transition-shadow">
            <div class="w-10 h-10 rounded-apple flex items-center justify-center text-lg flex-shrink-0 overflow-hidden" style="background:var(--bg-sidebar-hover)">
              <img v-if="isUrl(tool.icon)" :src="tool.icon" class="w-full h-full object-cover rounded-apple" />
              <span v-else>{{ tool.icon || '🔗' }}</span>
            </div>
            <div class="min-w-0 flex-1">
              <p class="text-[13px] font-medium truncate" style="color:var(--fg)">{{ tool.name }}</p>
              <p class="text-xs truncate" style="color:var(--fg-secondary)">{{ tool.description }}</p>
            </div>
            <div class="flex gap-1 flex-shrink-0">
              <button @click="editTool(tool)" class="text-xs hover:opacity-60" style="color:var(--accent)">{{ t('adminTools.edit') }}</button>
              <button @click="deleteTool(tool)" class="text-xs hover:opacity-60" style="color:#FF3B30">{{ t('adminTools.delete') }}</button>
            </div>
          </div>
        </div>
      </div>
      <p v-if="!categories.length" class="text-center py-16 text-sm" style="color:var(--fg-tertiary)">{{ t('adminTools.empty') }}</p>
    </div>
  </MainLayout>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import MainLayout from '../components/MainLayout.vue'
import { categoryApi, toolApi } from '../api'

const { t } = useI18n()
const categories = ref([])
const tools = ref([])
const showCatForm = ref(false)
const showToolForm = ref(false)
const editingCat = ref(null)
const editingTool = ref(null)
const catForm = ref({ name: '', icon: '', sort_order: 0 })
const toolForm = ref(defaultTool())

function defaultTool() { return { name: '', category_id: 0, url: '', icon: '', description: '', is_featured: false, sort_order: 0, tagsStr: '', platformsStr: '', versions: [] } }
function isUrl(s) { return s && (s.startsWith('http://') || s.startsWith('https://')) }
function getToolsByCat(cid) { return tools.value.filter(t => t.category_id === cid) }

onMounted(fetch)

async function fetch() {
  try {
    const [catRes, toolRes] = await Promise.all([categoryApi.list(), toolApi.list()])
    categories.value = catRes.data || []
    tools.value = toolRes.data.items || []
  } catch {}
}

function editCategory(cat) {
  editingCat.value = cat || null
  catForm.value = cat ? { name: cat.name, icon: cat.icon, sort_order: cat.sort_order || 0 } : { name: '', icon: '', sort_order: 0 }
  showCatForm.value = true
}

async function saveCategory() {
  if (!catForm.value.name) return
  try { if (editingCat.value) await categoryApi.update(editingCat.value.id, catForm.value); else await categoryApi.create(catForm.value); showCatForm.value = false; fetch() } catch (e) { alert(e.response?.data?.detail || 'Error') }
}

async function deleteCategory(cat) {
  if (!confirm(t('adminTools.confirmDelete'))) return
  try { await categoryApi.delete(cat.id); fetch() } catch (e) { alert(e.response?.data?.detail || 'Error') }
}

function editTool(tool) {
  editingTool.value = tool || null
  if (tool) {
    toolForm.value = { name: tool.name, category_id: tool.category_id, url: tool.url, icon: tool.icon, description: tool.description, is_featured: tool.is_featured, sort_order: tool.sort_order || 0, tagsStr: (tool.tags || []).join(','), platformsStr: (tool.platforms || []).join(','), versions: (tool.versions || []).map(v => ({ version: v.version || '', url: v.url || '' })) }
  } else { toolForm.value = defaultTool() }
  showToolForm.value = true
}

async function saveTool() {
  if (!toolForm.value.name) return
  const data = { name: toolForm.value.name, category_id: toolForm.value.category_id, url: toolForm.value.url, icon: toolForm.value.icon, description: toolForm.value.description, is_featured: toolForm.value.is_featured, sort_order: toolForm.value.sort_order, tags: toolForm.value.tagsStr ? toolForm.value.tagsStr.split(',').map(s => s.trim()).filter(Boolean) : [], platforms: toolForm.value.platformsStr ? toolForm.value.platformsStr.split(',').map(s => s.trim()).filter(Boolean) : [], versions: toolForm.value.versions.filter(v => v.version.trim()).map(v => ({ version: v.version.trim(), url: v.url.trim() })) }
  try { if (editingTool.value) await toolApi.update(editingTool.value.id, data); else await toolApi.create(data); showToolForm.value = false; fetch() } catch (e) { alert(e.response?.data?.detail || 'Error') }
}

async function deleteTool(tool) {
  if (!confirm(t('adminTools.confirmDelete'))) return
  try { await toolApi.delete(tool.id); fetch() } catch {}
}
</script>
