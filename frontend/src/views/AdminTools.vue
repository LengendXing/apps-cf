<template>
  <MainLayout>
    <div>
      <div class="flex items-center justify-between mb-6">
        <h2 class="text-2xl font-bold">{{ t('adminTools.title') }}</h2>
        <div class="flex gap-2">
          <button @click="editCategory(null)" class="px-3 py-1.5 text-sm bg-primary text-primary-foreground rounded-md hover:opacity-90 transition">{{ t('adminTools.addCategory') }}</button>
          <button @click="editTool(null)" class="px-3 py-1.5 text-sm bg-primary text-primary-foreground rounded-md hover:opacity-90 transition">{{ t('adminTools.addTool') }}</button>
        </div>
      </div>

      <!-- Category Form -->
      <div v-if="showCatForm" class="mb-6 bg-card rounded-lg border border-border p-4">
        <h3 class="font-semibold mb-3">{{ editingCat ? t('adminTools.editCategory') : t('adminTools.categoryForm') }}</h3>
        <div class="flex gap-2 items-end">
          <div class="flex-1">
            <label class="text-xs text-muted-foreground block mb-1">{{ t('adminTools.categoryName') }}</label>
            <input v-model="catForm.name" :placeholder="t('adminTools.categoryName')" class="w-full px-3 py-1.5 text-sm rounded-md border border-border bg-background" />
          </div>
          <div class="w-24">
            <label class="text-xs text-muted-foreground block mb-1">{{ t('adminTools.icon') }}</label>
            <input v-model="catForm.icon" placeholder="Icon" class="w-full px-3 py-1.5 text-sm rounded-md border border-border bg-background" />
          </div>
          <div class="w-20">
            <label class="text-xs text-muted-foreground block mb-1">{{ t('adminTools.sortOrder') }}</label>
            <input v-model.number="catForm.sort_order" type="number" class="w-full px-3 py-1.5 text-sm rounded-md border border-border bg-background" />
          </div>
          <button @click="saveCategory" class="px-3 py-1.5 text-sm bg-primary text-primary-foreground rounded-md">{{ t('adminTools.save') }}</button>
          <button @click="showCatForm = false" class="px-3 py-1.5 text-sm border border-border rounded-md">{{ t('adminTools.cancel') }}</button>
        </div>
      </div>

      <!-- Tool Form -->
      <div v-if="showToolForm" class="mb-6 bg-card rounded-lg border border-border p-4">
        <h3 class="font-semibold mb-3">{{ editingTool ? t('adminTools.editTool') : t('adminTools.toolForm') }}</h3>
        <div class="grid grid-cols-2 gap-3">
          <div>
            <label class="text-xs text-muted-foreground block mb-1">{{ t('adminTools.toolName') }}</label>
            <input v-model="toolForm.name" :placeholder="t('adminTools.toolName')" class="w-full px-3 py-1.5 text-sm rounded-md border border-border bg-background" />
          </div>
          <div>
            <label class="text-xs text-muted-foreground block mb-1">{{ t('adminTools.categoryName') }}</label>
            <select v-model="toolForm.category_id" class="w-full px-3 py-1.5 text-sm rounded-md border border-border bg-background">
              <option :value="0">{{ t('adminTools.noCategory') }}</option>
              <option v-for="c in categories" :key="c.id" :value="c.id">{{ c.name }}</option>
            </select>
          </div>
          <div class="col-span-2">
            <label class="text-xs text-muted-foreground block mb-1">{{ t('adminTools.toolUrl') }}</label>
            <input v-model="toolForm.url" :placeholder="t('adminTools.toolUrl')" class="w-full px-3 py-1.5 text-sm rounded-md border border-border bg-background" />
          </div>
          <div>
            <label class="text-xs text-muted-foreground block mb-1">{{ t('adminTools.icon') }}</label>
            <input v-model="toolForm.icon" :placeholder="toolForm.iconType === 'url' ? t('adminTools.iconUrlPlaceholder') : '🐙'" class="w-full px-3 py-1.5 text-sm rounded-md border border-border bg-background" />
          </div>
          <div>
            <label class="text-xs text-muted-foreground block mb-1">{{ t('adminTools.description') }}</label>
            <input v-model="toolForm.description" :placeholder="t('adminTools.description')" class="w-full px-3 py-1.5 text-sm rounded-md border border-border bg-background" />
          </div>
          <div>
            <label class="text-xs text-muted-foreground block mb-1">{{ t('adminTools.tags') }}</label>
            <input v-model="toolForm.tagsStr" placeholder="tag1,tag2" class="w-full px-3 py-1.5 text-sm rounded-md border border-border bg-background" />
          </div>
          <div>
            <label class="text-xs text-muted-foreground block mb-1">{{ t('adminTools.platforms') }}</label>
            <input v-model="toolForm.platformsStr" placeholder="mac,windows" class="w-full px-3 py-1.5 text-sm rounded-md border border-border bg-background" />
          </div>
        </div>

        <div class="mt-4">
          <div class="flex items-center justify-between mb-2">
            <label class="text-sm font-semibold">{{ t('adminTools.versions') }}</label>
            <button @click="toolForm.versions.push({ version: '', url: '' })" class="px-2 py-0.5 text-xs bg-muted rounded hover:bg-primary hover:text-primary-foreground transition-colors">{{ t('adminTools.addVersion') }}</button>
          </div>
          <div v-for="(v, i) in toolForm.versions" :key="i" class="flex gap-2 items-center mb-2">
            <span class="text-xs text-muted-foreground w-5 text-right">{{ i + 1 }}</span>
            <input v-model="v.version" :placeholder="t('adminTools.versionNumber')" class="w-32 px-2 py-1 text-sm rounded-md border border-border bg-background" />
            <input v-model="v.url" :placeholder="t('adminTools.versionUrl')" class="flex-1 px-2 py-1 text-sm rounded-md border border-border bg-background" />
            <button @click="toolForm.versions.splice(i, 1)" class="text-red-400 hover:text-red-600 text-xs">{{ t('adminTools.removeVersion') }}</button>
          </div>
        </div>

        <div class="flex items-center gap-4 mt-3">
          <label class="flex items-center gap-1 text-sm cursor-pointer">
            <input v-model="toolForm.is_featured" type="checkbox" /> {{ t('adminTools.featured') }}
          </label>
          <div>
            <label class="text-xs text-muted-foreground mr-1">{{ t('adminTools.sortOrder') }}</label>
            <input v-model.number="toolForm.sort_order" type="number" class="w-20 px-3 py-1.5 text-sm rounded-md border border-border bg-background" />
          </div>
          <div class="flex-1" />
          <button @click="saveTool" class="px-3 py-1.5 text-sm bg-primary text-primary-foreground rounded-md">{{ t('adminTools.save') }}</button>
          <button @click="showToolForm = false" class="px-3 py-1.5 text-sm border border-border rounded-md">{{ t('adminTools.cancel') }}</button>
        </div>
      </div>

      <!-- Categories & Tools -->
      <div v-for="cat in categories" :key="cat.id" class="mb-8">
        <div class="flex items-center gap-2 mb-3">
          <h3 class="text-lg font-semibold">{{ cat.icon }} {{ cat.name }}</h3>
          <span class="text-xs text-muted-foreground">({{ toolsByCat(cat.id).length }})</span>
          <button @click="editCategory(cat)" class="text-xs text-muted-foreground hover:text-foreground ml-2">{{ t('adminTools.edit') }}</button>
          <button @click="deleteCategory(cat)" class="text-xs text-red-400 hover:text-red-600">{{ t('adminTools.delete') }}</button>
        </div>
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3">
          <div v-for="tool in toolsByCat(cat.id)" :key="tool.id" class="bg-card rounded-lg border border-border p-4 hover:bg-muted/50 transition-all flex items-center gap-3">
            <div class="w-10 h-10 rounded-lg bg-muted flex items-center justify-center text-lg flex-shrink-0 overflow-hidden">
              <img v-if="isUrl(tool.icon)" :src="tool.icon" class="w-full h-full object-cover rounded-lg" />
              <span v-else>{{ tool.icon || '🔗' }}</span>
            </div>
            <div class="min-w-0 flex-1">
              <p class="font-medium truncate">{{ tool.name }}</p>
              <p class="text-xs text-muted-foreground truncate">{{ tool.description }}</p>
              <p v-if="tool.versions?.length" class="text-xs text-muted-foreground mt-0.5">{{ tool.versions.length }} versions</p>
            </div>
            <div class="flex gap-1 flex-shrink-0">
              <button @click="editTool(tool)" class="text-muted-foreground hover:text-foreground text-sm">{{ t('adminTools.edit') }}</button>
              <button @click="deleteTool(tool)" class="text-red-400 hover:text-red-600 text-sm">{{ t('adminTools.delete') }}</button>
            </div>
          </div>
        </div>
      </div>
      <p v-if="!categories.length" class="text-center text-muted-foreground py-12 text-sm">{{ t('adminTools.empty') }}</p>
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
const showCatForm = ref(false)
const showToolForm = ref(false)
const editingCat = ref(null)
const editingTool = ref(null)
const catForm = ref({ name: '', icon: '', sort_order: 0 })
const toolForm = ref(defaultTool())

function defaultTool() {
  return { name: '', category_id: 0, url: '', icon: '', description: '', is_featured: false, sort_order: 0, tagsStr: '', platformsStr: '', versions: [] }
}

function isUrl(s) { return s && (s.startsWith('http://') || s.startsWith('https://')) }
function toolsByCat(cid) { return categories.value.length ? [] : [] }

const tools = ref([])
function getToolsByCat(cid) { return tools.value.filter(t => t.category_id === cid) }

onMounted(fetch)

async function fetch() {
  try {
    const [catRes, toolRes] = await Promise.all([categoryApi.list(), toolApi.list()])
    categories.value = catRes.data || []
    tools.value = toolRes.data.items || []
  } catch {}
}

// Override toolsByCat to use reactive data
Object.assign(toolsByCat, () => tools.value.filter(t => t.category_id === cid))

function editCategory(cat) {
  editingCat.value = cat || null
  catForm.value = cat ? { name: cat.name, icon: cat.icon, sort_order: cat.sort_order || 0 } : { name: '', icon: '', sort_order: 0 }
  showCatForm.value = true
}

async function saveCategory() {
  if (!catForm.value.name) return
  try {
    if (editingCat.value) await categoryApi.update(editingCat.value.id, catForm.value)
    else await categoryApi.create(catForm.value)
    showCatForm.value = false
    fetch()
  } catch (e) { alert(e.response?.data?.detail || 'Error') }
}

async function deleteCategory(cat) {
  if (!confirm(t('adminTools.confirmDelete'))) return
  try { await categoryApi.delete(cat.id); fetch() } catch (e) { alert(e.response?.data?.detail || 'Error') }
}

function editTool(tool) {
  editingTool.value = tool || null
  if (tool) {
    const iconIsUrl = isUrl(tool.icon)
    toolForm.value = {
      name: tool.name, category_id: tool.category_id, url: tool.url, icon: tool.icon,
      description: tool.description, is_featured: tool.is_featured, sort_order: tool.sort_order || 0,
      tagsStr: (tool.tags || []).join(','), platformsStr: (tool.platforms || []).join(','),
      versions: (tool.versions || []).map(v => ({ version: v.version || '', url: v.url || '' })),
    }
  } else {
    toolForm.value = defaultTool()
  }
  showToolForm.value = true
}

async function saveTool() {
  if (!toolForm.value.name) return
  const data = {
    name: toolForm.value.name, category_id: toolForm.value.category_id, url: toolForm.value.url,
    icon: toolForm.value.icon, description: toolForm.value.description,
    is_featured: toolForm.value.is_featured, sort_order: toolForm.value.sort_order,
    tags: toolForm.value.tagsStr ? toolForm.value.tagsStr.split(',').map(s => s.trim()).filter(Boolean) : [],
    platforms: toolForm.value.platformsStr ? toolForm.value.platformsStr.split(',').map(s => s.trim()).filter(Boolean) : [],
    versions: toolForm.value.versions.filter(v => v.version.trim()).map(v => ({ version: v.version.trim(), url: v.url.trim() })),
  }
  try {
    if (editingTool.value) await toolApi.update(editingTool.value.id, data)
    else await toolApi.create(data)
    showToolForm.value = false
    fetch()
  } catch (e) { alert(e.response?.data?.detail || 'Error') }
}

async function deleteTool(tool) {
  if (!confirm(t('adminTools.confirmDelete'))) return
  try { await toolApi.delete(tool.id); fetch() } catch {}
}
</script>
