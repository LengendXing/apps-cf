<template>
  <MainLayout>
    <div>
      <h1 class="text-[28px] font-semibold tracking-tight mb-6" style="color:var(--fg)">{{ t('adminNotes.title') }}</h1>
      <div class="flex gap-4">
        <!-- Folders -->
        <div class="apple-card p-4 w-56 flex-shrink-0">
          <div class="flex items-center justify-between mb-3">
            <span class="text-xs font-semibold uppercase tracking-wide" style="color:var(--fg-secondary)">{{ t('adminNotes.folder') }}</span>
            <button @click="editFolder(null)" class="apple-btn text-xs py-1 px-2">+</button>
          </div>
          <div v-if="!folders.length" class="text-xs py-4 text-center" style="color:var(--fg-tertiary)">{{ t('adminNotes.noFolders') }}</div>
          <div v-for="f in folders" :key="f.id" @click="selectedFolder=f.id" class="flex items-center justify-between px-2 py-1.5 rounded-lg cursor-pointer transition-colors mb-0.5" :style="selectedFolder===f.id?'background:var(--bg-sidebar-active);color:var(--accent)':'color:var(--fg-secondary)'">
            <span class="text-[13px] font-medium truncate">{{ f.name }}</span>
            <div class="flex gap-0.5">
              <button @click.stop="editFolder(f)" class="w-5 h-5 rounded flex items-center justify-center text-[10px] hover:opacity-60" style="color:var(--fg-tertiary)">✎</button>
              <button @click.stop="confirmDeleteFolder(f)" class="w-5 h-5 rounded flex items-center justify-center text-[10px] hover:opacity-60" style="color:#FF3B30">✕</button>
            </div>
          </div>
        </div>
        <!-- Notes -->
        <div class="flex-1">
          <div class="flex items-center gap-2 mb-3">
            <input v-model="search" :placeholder="t('adminNotes.search')" class="apple-input h-[34px]" style="width:224px" />
            <div class="flex-1" />
            <button @click="editNote(null)" class="apple-btn text-xs h-[34px] whitespace-nowrap">{{ t('adminNotes.addNote') }}</button>
          </div>
          <div class="apple-card overflow-hidden">
            <table class="apple-table">
              <thead><tr><th>ID</th><th>{{ t('adminNotes.noteTitle') }}</th><th>{{ t('adminNotes.folder') }}</th><th>{{ t('adminNotes.size') }}</th><th>{{ t('adminNotes.createdAt') }}</th><th class="text-right">{{ t('adminTools.delete') }}</th></tr></thead>
              <tbody>
                <tr v-for="n in filteredNotes" :key="n.id" @click="openNoteDrawer(n)" class="cursor-pointer">
                  <td style="color:var(--fg-tertiary)">{{ n.id }}</td>
                  <td><span class="font-medium" style="color:var(--fg)">{{ n.title }}</span></td>
                  <td style="color:var(--fg-secondary)">{{ folderName(n.folder_id) }}</td>
                  <td style="color:var(--fg-tertiary)">{{ n.size }}B</td>
                  <td style="color:var(--fg-tertiary)" class="whitespace-nowrap">{{ n.created_at }}</td>
                  <td class="text-right">
                    <button @click.stop="editNote(n)" class="apple-btn-secondary text-xs py-1 px-2 mr-1">{{ t('adminTools.edit') }}</button>
                    <button @click.stop="confirmDeleteNote(n)" class="apple-btn-danger text-xs py-1 px-2">{{ t('adminTools.delete') }}</button>
                  </td>
                </tr>
                <tr v-if="!filteredNotes.length"><td colspan="6" class="text-center py-12 text-sm" style="color:var(--fg-tertiary)">{{ t('adminNotes.noNotes') }}</td></tr>
              </tbody>
            </table>
          </div>
          <div v-if="totalPages>1" class="flex items-center justify-center gap-1.5 mt-4">
            <button v-for="p in totalPages" :key="p" @click="page=p;fetchNotes()" class="w-8 h-8 rounded-lg text-xs font-medium transition-colors" :style="p===page?'background:var(--accent);color:#fff':'color:var(--fg-secondary)'">{{ p }}</button>
          </div>
        </div>
      </div>

      <!-- Folder Modal -->
      <Transition name="modal">
        <div v-if="showFolderModal" class="fixed inset-0 z-50 flex items-center justify-center">
          <div class="absolute inset-0 bg-black/30 backdrop-blur-sm" @click="showFolderModal=false" />
          <div class="relative w-full max-w-sm rounded-apple-xl overflow-hidden" style="background:var(--bg-card);box-shadow:0 24px 80px rgba(0,0,0,0.25)">
            <div class="px-6 py-4" style="border-bottom:1px solid var(--divider)">
              <h3 class="text-[15px] font-semibold" style="color:var(--fg)">{{ editingFolder ? t('adminNotes.editFolder') : t('adminNotes.addFolder') }}</h3>
            </div>
            <div class="p-6"><label class="text-xs block mb-1" style="color:var(--fg-secondary)">{{ t('adminNotes.folderName') }}</label><input v-model="folderForm.name" class="apple-input" /></div>
            <div class="flex justify-end gap-2 px-6 py-4" style="border-top:1px solid var(--divider)">
              <button @click="showFolderModal=false" class="apple-btn-secondary">{{ t('adminNotes.cancel') }}</button>
              <button @click="saveFolder" class="apple-btn">{{ t('adminNotes.save') }}</button>
            </div>
          </div>
        </div>
      </Transition>

      <!-- Note Modal -->
      <Transition name="modal">
        <div v-if="showNoteModal" class="fixed inset-0 z-50 flex items-center justify-center">
          <div class="absolute inset-0 bg-black/30 backdrop-blur-sm" @click="showNoteModal=false" />
          <div class="relative w-full max-w-lg max-h-[85vh] flex flex-col rounded-apple-xl overflow-hidden" style="background:var(--bg-card);box-shadow:0 24px 80px rgba(0,0,0,0.25)">
            <div class="px-6 py-4" style="border-bottom:1px solid var(--divider)">
              <h3 class="text-[15px] font-semibold" style="color:var(--fg)">{{ editingNote ? t('adminNotes.editNote') : t('adminNotes.addNote') }}</h3>
            </div>
            <div class="flex-1 overflow-y-auto p-6 space-y-4">
              <div><label class="text-xs block mb-1" style="color:var(--fg-secondary)">{{ t('adminNotes.noteTitle') }}</label><input v-model="noteForm.title" class="apple-input" /></div>
              <div>
                <label class="text-xs block mb-1" style="color:var(--fg-secondary)">{{ t('adminNotes.folder') }}</label>
                <select v-model="noteForm.folder_id" class="apple-select w-full">
                  <option v-for="f in folders" :key="f.id" :value="f.id">{{ f.name }}</option>
                </select>
                <div class="mt-2 flex items-center gap-2">
                  <span class="text-xs" style="color:var(--fg-tertiary)">{{ t('adminNotes.or') }}</span>
                  <button @click="inlineNewFolder=true" class="text-xs font-medium" style="color:var(--accent)">{{ t('adminNotes.createFolder') }}</button>
                </div>
                <div v-if="inlineNewFolder" class="mt-2 flex gap-2">
                  <input v-model="newFolderName" :placeholder="t('adminNotes.folderName')" class="apple-input flex-1" />
                  <button @click="createInlineFolder" class="apple-btn text-xs">OK</button>
                </div>
              </div>
              <div><label class="text-xs block mb-1" style="color:var(--fg-secondary)">{{ t('adminNotes.noteContent') }}</label><textarea v-model="noteForm.content" rows="10" class="apple-input font-mono leading-relaxed" /></div>
            </div>
            <div class="flex justify-end gap-2 px-6 py-4" style="border-top:1px solid var(--divider)">
              <button @click="showNoteModal=false" class="apple-btn-secondary">{{ t('adminNotes.cancel') }}</button>
              <button @click="saveNote" class="apple-btn">{{ t('adminNotes.save') }}</button>
            </div>
          </div>
        </div>
      </Transition>

      <!-- Note Detail Drawer -->
      <Transition name="drawer">
        <div v-if="drawerNote" class="fixed top-0 right-0 bottom-0 w-96 overflow-y-auto z-[60] shadow-xl flex flex-col" style="background:var(--bg-card);border-left:1px solid var(--divider)">
          <div class="flex items-center justify-between p-4" style="border-bottom:1px solid var(--divider)">
            <h3 class="text-[15px] font-semibold" style="color:var(--fg)">{{ drawerNote.title }}</h3>
            <button @click="drawerNote=null" class="w-6 h-6 rounded-full flex items-center justify-center text-xs hover:opacity-60" style="background:var(--bg-sidebar-hover);color:var(--fg-secondary)">✕</button>
          </div>
          <div class="p-5 space-y-3 flex-1">
            <div class="text-xs" style="color:var(--fg-secondary)">{{ t('adminNotes.folder') }}: {{ folderName(drawerNote.folder_id) }}</div>
            <pre class="text-sm whitespace-pre-wrap break-words font-mono leading-relaxed" style="color:var(--fg)">{{ drawerNote.content }}</pre>
          </div>
          <div class="p-4 text-xs space-y-1" style="color:var(--fg-tertiary);border-top:1px solid var(--divider)">
            <div>{{ t('adminNotes.createdAt') }}: {{ drawerNote.created_at }}</div>
            <div>{{ t('adminNotes.updatedAt') }}: {{ drawerNote.updated_at }}</div>
            <div>{{ t('adminNotes.createdIp') }}: {{ drawerNote.created_ip }}</div>
            <div>{{ t('adminNotes.size') }}: {{ drawerNote.size }}B</div>
            <div>{{ t('adminNotes.charCount') }}: {{ drawerNote.char_count }}</div>
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
            <input v-if="macDialog.type==='prompt'" v-model="macDialog.inputVal" class="mac-dialog-input" :placeholder="macDialog.placeholder" @keyup.enter="macDialog.show=false;macDialog.resolve(macDialog.inputVal)" />
            <div class="mac-dialog-actions">
              <button class="mac-dialog-cancel" @click="macDialog.show=false;macDialog.resolve(macDialog.type==='prompt'?'':false)">{{ t('adminNotes.cancel') }}</button>
              <button class="mac-dialog-ok" @click="macDialog.show=false;macDialog.resolve(macDialog.type==='prompt'?macDialog.inputVal:true)">{{ macDialog.okText || t('adminNotes.save') }}</button>
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
import { noteFolderApi, noteApi } from '../api'

const { t } = useI18n()
const folders = ref([])
const notes = ref([])
const selectedFolder = ref(0)
const search = ref('')
const page = ref(1)
const total = ref(0)
const pageSize = 20
const totalPages = computed(() => Math.ceil(total.value / pageSize))

const showFolderModal = ref(false)
const editingFolder = ref(null)
const folderForm = ref({ name: '' })

const showNoteModal = ref(false)
const editingNote = ref(null)
const noteForm = ref({ title: '', content: '', folder_id: 0 })
const inlineNewFolder = ref(false)
const newFolderName = ref('')

const drawerNote = ref(null)

// macOS dialog state
const macDialog = reactive({ show: false, title: '', message: '', type: 'confirm', inputVal: '', placeholder: '', okText: '', resolve: () => {} })

function macConfirm(title, message, okText) {
  return new Promise(resolve => {
    Object.assign(macDialog, { show: true, title, message, type: 'confirm', inputVal: '', placeholder: '', okText: okText || t('adminTools.delete'), resolve })
  })
}

function macPrompt(title, message, placeholder, okText) {
  return new Promise(resolve => {
    Object.assign(macDialog, { show: true, title, message, type: 'prompt', inputVal: '', placeholder: placeholder || '', okText: okText || t('adminNotes.save'), resolve })
  })
}

const filteredNotes = computed(() => {
  if (!search.value) return notes.value
  const q = search.value.toLowerCase()
  return notes.value.filter(n => n.title.toLowerCase().includes(q) || n.content.toLowerCase().includes(q))
})

function folderName(id) { return folders.value.find(f => f.id === id)?.name || '-' }

onMounted(() => { fetchFolders(); fetchNotes() })

async function fetchFolders() { try { const res = await noteFolderApi.list(); folders.value = res.data || [] } catch {} }

async function fetchNotes() {
  try {
    const params = { page: page.value, page_size: pageSize }
    if (selectedFolder.value) params.folder_id = selectedFolder.value
    const res = await noteApi.list(params)
    notes.value = res.data?.items || []
    total.value = res.data?.total || 0
  } catch {}
}

function editFolder(f) { editingFolder.value = f || null; folderForm.value = f ? { name: f.name } : { name: '' }; showFolderModal.value = true }

async function saveFolder() {
  try {
    if (editingFolder.value) await noteFolderApi.update(editingFolder.value.id, folderForm.value)
    else await noteFolderApi.create(folderForm.value)
    showFolderModal.value = false; fetchFolders()
  } catch {}
}

async function confirmDeleteFolder(f) {
  const ok = await macConfirm(t('adminNotes.confirmDeleteFolder'), t('adminNotes.confirmDeleteFolder'))
  if (!ok) return
  try { await noteFolderApi.delete(f.id); if (selectedFolder.value === f.id) selectedFolder.value = 0; fetchFolders(); fetchNotes() } catch {}
}

function editNote(n) { editingNote.value = n || null; noteForm.value = n ? { title: n.title, content: n.content, folder_id: n.folder_id } : { title: '', content: '', folder_id: folders.value[0]?.id || 0 }; inlineNewFolder.value = false; showNoteModal.value = true }

async function saveNote() {
  try {
    if (editingNote.value) await noteApi.update(editingNote.value.id, noteForm.value)
    else await noteApi.create(noteForm.value)
    showNoteModal.value = false; fetchNotes()
  } catch {}
}

async function confirmDeleteNote(n) {
  const ok = await macConfirm(t('adminNotes.confirmDelete'), t('adminNotes.confirmDelete'))
  if (!ok) return
  try { await noteApi.delete(n.id); fetchNotes() } catch {}
}

async function createInlineFolder() {
  if (!newFolderName.value) return
  try { const res = await noteFolderApi.create({ name: newFolderName.value }); folders.value.push(res.data); noteForm.value.folder_id = res.data.id; newFolderName.value = ''; inlineNewFolder.value = false; fetchFolders() } catch {}
}

function openNoteDrawer(n) { drawerNote.value = n }
</script>
