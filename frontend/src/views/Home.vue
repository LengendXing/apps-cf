<template>
  <div v-if="locked" class="h-screen flex items-center justify-center" style="background:var(--background)">
    <div class="w-full max-w-md mx-auto p-8">
      <div class="p-10 rounded-2xl text-center" style="background:var(--card);border:1px solid var(--border-color)">
        <h2 class="text-3xl font-bold mb-2" style="color:var(--foreground)">{{ t('home.welcome') }}</h2>
        <p class="text-sm mb-8" style="color:var(--muted-foreground)">{{ t('home.hint') }}</p>
        <form @submit.prevent="verifyPassword" class="space-y-4">
          <input v-model="pwd" type="password" :placeholder="t('home.password')" class="w-full px-4 py-3 rounded-xl border text-center text-lg tracking-widest" style="background:var(--background);border-color:var(--border-color);color:var(--foreground)" required />
          <p v-if="pwdError" class="text-sm" style="color:#FF3B30">{{ pwdError }}</p>
          <button type="submit" class="w-full py-3 rounded-xl font-medium transition-opacity hover:opacity-80" style="background:var(--primary);color:var(--primary-foreground)">{{ t('home.submit') }}</button>
        </form>
      </div>
    </div>
  </div>

  <div v-else class="h-screen flex flex-col overflow-hidden" style="background:var(--background)">
    <!-- Header -->
    <header class="flex-shrink-0 h-12 z-50" style="background:var(--card);border-bottom:1px solid var(--border-color)">
      <div class="max-w-[1400px] mx-auto h-full flex items-center justify-between px-3">
        <span class="text-sm font-bold tracking-tight" style="color:var(--foreground)">APPS</span>
        <div class="flex items-center gap-2">
          <!-- Notes button -->
          <button @click="viewMode = viewMode==='notes'?'app':'notes'" class="p-1.5 rounded-lg transition-colors hover:opacity-70" :style="viewMode==='notes'?'background:var(--foreground);color:var(--background)':'color:var(--muted-foreground)'" title="Notes">
            <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><polyline points="14 2 14 8 20 8"/><line x1="16" y1="13" x2="8" y2="13"/><line x1="16" y1="17" x2="8" y2="17"/></svg>
          </button>
          <!-- Config toggle -->
          <button @click="viewMode = viewMode==='config'?'app':'config'" class="p-1.5 rounded-lg transition-colors hover:opacity-70" :style="viewMode==='config'?'background:var(--foreground);color:var(--background)':'color:var(--muted-foreground)'" title="Config">
            <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="3"/><path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83-2.83l.06-.06A1.65 1.65 0 0 0 4.68 15a1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 2.83-2.83l.06.06A1.65 1.65 0 0 0 9 4.68a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 2.83l-.06.06A1.65 1.65 0 0 0 19.4 9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z"/></svg>
          </button>
          <!-- Locale -->
          <button @click="toggleLocale" class="p-1.5 rounded-lg transition-colors hover:opacity-70" style="color:var(--muted-foreground)" title="Language">
            <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><line x1="2" y1="12" x2="22" y2="12"/><path d="M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z"/></svg>
          </button>
          <!-- Dark -->
          <button @click="toggleDark" class="p-1.5 rounded-lg transition-colors hover:opacity-70" style="color:var(--muted-foreground)" :title="isDark?'Light':'Dark'">
            <svg v-if="isDark" class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="5"/><line x1="12" y1="1" x2="12" y2="3"/><line x1="12" y1="21" x2="12" y2="23"/><line x1="4.22" y1="4.22" x2="5.64" y2="5.64"/><line x1="18.36" y1="18.36" x2="19.78" y2="19.78"/><line x1="1" y1="12" x2="3" y2="12"/><line x1="21" y1="12" x2="23" y2="12"/><line x1="4.22" y1="19.78" x2="5.64" y2="18.36"/><line x1="18.36" y1="5.64" x2="19.78" y2="4.22"/></svg>
            <svg v-else class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"/></svg>
          </button>
        </div>
      </div>
    </header>

    <!-- Notes View -->
    <template v-if="viewMode==='notes'">
      <div class="flex-1 flex overflow-hidden">
        <aside class="w-56 flex-shrink-0 overflow-y-auto py-3 px-2" style="border-right:1px solid var(--border-color)">
          <div v-for="folder in noteFolders" :key="folder.id" class="mb-2">
            <div @click="toggleFolder(folder.id)" class="flex items-center gap-1.5 px-2 py-1.5 rounded-lg cursor-pointer text-sm font-medium transition-colors" style="color:var(--foreground)">
              <svg class="w-3.5 h-3.5 transition-transform" :style="openFolders.has(folder.id)?'transform:rotate(90deg)':''" viewBox="0 0 24 24" fill="currentColor"><path d="M8 5l8 7-8 7z"/></svg>
              <svg class="w-3.5 h-3.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/></svg>
              <span class="truncate">{{ folder.name }}</span>
            </div>
            <div v-if="openFolders.has(folder.id)" class="ml-4 space-y-0.5">
              <div v-for="n in folderNotes(folder.id)" :key="n.id" @click="selectedNote=n" class="px-2 py-1 rounded-lg cursor-pointer text-xs truncate transition-colors" :style="selectedNote?.id===n.id?'background:var(--foreground);color:var(--background)':'color:var(--muted-foreground)'">{{ n.title }}</div>
            </div>
          </div>
          <div v-if="!noteFolders.length" class="text-xs text-center py-8" style="color:var(--muted-foreground)">{{ t('home.noFolders') }}</div>
        </aside>
        <main class="flex-1 flex flex-col overflow-hidden">
          <div v-if="selectedNote" class="flex-1 flex flex-col">
            <div class="flex items-center justify-between px-4 py-2" style="border-bottom:1px solid var(--border-color)">
              <span class="text-sm font-medium" style="color:var(--foreground)">{{ selectedNote.title }}</span>
              <span class="text-xs flex items-center gap-1" :style="autoSaveState==='saving'?'color:var(--muted-foreground)':'color:#34C759'">
                <svg v-if="autoSaveState==='saving'" class="w-3 h-3 animate-spin" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M12 2v4m0 12v4m-7.07-3.93l2.83-2.83m8.48-8.48l2.83-2.83M2 12h4m12 0h4M4.93 4.93l2.83 2.83m8.48 8.48l2.83 2.83"/></svg>
                <svg v-else-if="autoSaveState==='saved'" class="w-3 h-3" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3"><polyline points="20 6 9 17 4 12"/></svg>
                {{ autoSaveState==='saving'?t('home.saving'):autoSaveState==='saved'?t('home.saved'):'' }}
              </span>
            </div>
            <textarea v-model="noteEditContent" class="flex-1 w-full p-4 resize-none text-sm font-mono leading-relaxed outline-none" style="background:var(--background);color:var(--foreground)" @input="onNoteInput"></textarea>
          </div>
          <div v-else class="flex-1 flex items-center justify-center text-sm" style="color:var(--muted-foreground)">{{ t('home.noNotes') }}</div>
        </main>
      </div>
    </template>

    <!-- App/Config View -->
    <template v-else>
      <div class="flex-1 flex justify-center overflow-hidden">
        <div class="w-full max-w-[1400px] flex overflow-hidden relative">
          <aside class="flex-shrink-0 w-44 overflow-y-auto py-3 px-2" style="border-right:1px solid var(--border-color)">
            <button @click="selectedCat = 0" :class="selectedCat===0?'font-medium':'hover:opacity-70'" class="w-full text-left px-3 py-2 rounded-lg text-sm transition-colors truncate mb-0.5" :style="selectedCat===0?'background:var(--foreground);color:var(--background)':'color:var(--muted-foreground)'">{{ t('home.all') }}</button>
            <button v-for="cat in categories" :key="cat.id" @click="selectedCat = cat.id" :class="selectedCat===cat.id?'font-medium':'hover:opacity-70'" class="w-full text-left px-3 py-2 rounded-lg text-sm transition-colors truncate mb-0.5" :style="selectedCat===cat.id?'background:var(--foreground);color:var(--background)':'color:var(--muted-foreground)'">
              <img v-if="isUrl(cat.icon)" :src="cat.icon" class="w-4 h-4 inline rounded mr-1" />
              <span>{{ cat.name }}</span>
            </button>
          </aside>
          <main class="flex-1 overflow-y-auto p-5">
            <div class="mb-4">
              <input v-model="search" :placeholder="t('home.search')" class="w-full max-w-md px-4 py-2 rounded-xl border text-sm outline-none" style="background:var(--background);border-color:var(--border-color);color:var(--foreground)" />
            </div>
            <template v-if="viewMode==='app'">
              <div v-if="filtered.length" class="grid grid-cols-7 gap-3">
                <div v-for="tool in filtered" :key="tool.id" class="group aspect-square rounded-xl p-3 flex flex-col items-center justify-center gap-2 transition-all cursor-pointer relative" style="background:var(--card);border:1px solid var(--border-color)">
                  <button @click.stop="openInfo(tool)" class="absolute top-2 right-2 w-5 h-5 rounded-full flex items-center justify-center text-[10px] transition-colors z-10" style="background:var(--muted);color:var(--muted-foreground)" title="Detail">i</button>
                  <a :href="tool.url" target="_blank" @click.stop class="flex flex-col items-center justify-center gap-2 w-full h-full no-underline" style="color:var(--foreground)">
                    <div class="w-11 h-11 rounded-xl flex items-center justify-center flex-shrink-0 group-hover:scale-110 transition-transform overflow-hidden" style="background:var(--muted)">
                      <img v-if="isUrl(tool.icon)" :src="tool.icon" class="w-7 h-7 object-contain" />
                      <span v-else class="text-lg">{{ tool.icon || '🔗' }}</span>
                    </div>
                    <span class="text-xs font-medium text-center leading-tight line-clamp-2 w-full">{{ tool.name }}</span>
                  </a>
                </div>
              </div>
              <p v-else-if="loading" class="text-center py-20 text-sm" style="color:var(--muted-foreground)">Loading...</p>
              <p v-else class="text-center py-20 text-sm" style="color:var(--muted-foreground)">{{ t('home.empty') }}</p>
            </template>
            <template v-else>
              <div v-if="filtered.length" class="grid grid-cols-7 gap-3">
                <div v-for="tool in filtered" :key="tool.id" @click="openConfigs(tool)" class="group aspect-square rounded-xl p-3 flex flex-col items-center justify-center gap-2 transition-all cursor-pointer" style="background:var(--card);border:1px solid var(--border-color)">
                  <div class="flex flex-col items-center justify-center gap-2 w-full h-full">
                    <div class="w-11 h-11 rounded-xl flex items-center justify-center flex-shrink-0 group-hover:scale-110 transition-transform overflow-hidden" style="background:var(--muted)">
                      <img v-if="isUrl(tool.icon)" :src="tool.icon" class="w-7 h-7 object-contain" />
                      <span v-else class="text-lg">{{ tool.icon || '🔗' }}</span>
                    </div>
                    <span class="text-xs font-medium text-center leading-tight line-clamp-2 w-full" style="color:var(--foreground)">{{ tool.name }}</span>
                  </div>
                </div>
              </div>
              <p v-else-if="loading" class="text-center py-20 text-sm" style="color:var(--muted-foreground)">Loading...</p>
              <p v-else class="text-center py-20 text-sm" style="color:var(--muted-foreground)">{{ t('home.empty') }}</p>
            </template>
          </main>
        </div>
      </div>
    </template>

    <!-- Footer -->
    <footer class="flex-shrink-0 h-9 z-50" style="background:var(--card);border-top:1px solid var(--border-color)">
      <div class="max-w-[1400px] mx-auto h-full flex items-center justify-center gap-4 text-xs" style="color:var(--muted-foreground)">
        <span>Powered by Apps Startpage</span>
        <span>&copy; {{ new Date().getFullYear() }}</span>
      </div>
    </footer>

    <!-- Info Drawer -->
    <Transition name="drawer">
      <div v-if="infoTool" class="fixed top-0 right-0 bottom-0 w-80 overflow-y-auto z-[60] shadow-xl flex flex-col" style="background:var(--bg-card);border-left:1px solid var(--divider)">
        <div class="flex flex-shrink-0" style="border-bottom:1px solid var(--divider)">
          <button @click="infoTab = 'info'" :class="infoTab==='info'?'font-semibold':''" class="flex-1 py-2.5 text-sm text-center transition-colors" :style="infoTab==='info'?'border-bottom:2px solid var(--accent);color:var(--fg)':'color:var(--fg-secondary)'">{{ t('home.tabInfo') }}</button>
          <button @click="infoTab = 'script'" :class="infoTab==='script'?'font-semibold':''" class="flex-1 py-2.5 text-sm text-center transition-colors" :style="infoTab==='script'?'border-bottom:2px solid var(--accent);color:var(--fg)':'color:var(--fg-secondary)'">{{ t('home.tabScript') }}</button>
        </div>
        <button @click="infoTool=null" class="absolute top-2 right-2 w-6 h-6 rounded-full flex items-center justify-center text-xs hover:opacity-60 z-10" style="background:var(--bg-sidebar-hover);color:var(--fg-secondary)">✕</button>
        <div v-if="infoTab==='info'" class="p-5">
          <h3 class="text-lg font-bold mb-4 pr-8" style="color:var(--fg)">{{ infoTool.name }}</h3>
          <div class="w-16 h-16 rounded-xl flex items-center justify-center mb-4 overflow-hidden" style="background:var(--muted)">
            <img v-if="isUrl(infoTool.icon)" :src="infoTool.icon" class="w-10 h-10 object-contain" />
            <span v-else class="text-3xl">{{ infoTool.icon || '🔗' }}</span>
          </div>
          <p v-if="infoTool.description" class="text-sm mb-4" style="color:var(--fg-secondary)">{{ infoTool.description }}</p>
          <div v-if="infoTool.url" class="mb-4"><a :href="infoTool.url" target="_blank" class="text-sm hover:underline break-all" style="color:var(--accent)">{{ infoTool.url }}</a></div>
          <div v-if="infoTool.tags?.length" class="flex flex-wrap gap-1 mb-3"><span v-for="tag in infoTool.tags" :key="tag" class="px-2 py-0.5 text-xs rounded" style="background:var(--muted);color:var(--muted-foreground)">{{ tag }}</span></div>
          <div v-if="infoTool.platforms?.length" class="flex flex-wrap gap-1 mb-4"><span v-for="p in infoTool.platforms" :key="p" class="px-2 py-0.5 text-xs rounded" style="background:var(--muted);color:var(--muted-foreground)">{{ p }}</span></div>
          <div v-if="infoTool.versions?.length">
            <h4 class="text-sm font-semibold mb-2" style="color:var(--fg)">{{ t('home.versions') }}</h4>
            <div class="space-y-1">
              <div v-for="(v,i) in infoTool.versions" :key="i" class="flex items-center gap-2 text-sm">
                <span class="w-6 text-right flex-shrink-0" style="color:var(--muted-foreground)">{{ i+1 }}.</span>
                <a v-if="v.url" :href="v.url" target="_blank" class="hover:underline" style="color:var(--accent)">{{ v.version }}</a>
                <span v-else style="color:var(--fg-secondary)">{{ v.version }}</span>
              </div>
            </div>
          </div>
        </div>
        <div v-else class="p-5">
          <p v-if="!scripts.length" class="text-sm py-8 text-center" style="color:var(--muted-foreground)">{{ t('home.noScripts') }}</p>
          <div v-else class="space-y-4">
            <div v-for="s in scripts" :key="s.id" class="rounded-lg p-3" style="background:var(--muted);border:1px solid var(--border-color)">
              <div class="flex items-center justify-between mb-2">
                <span class="text-sm font-semibold" style="color:var(--foreground)">{{ s.name }}</span>
                <span v-if="s.platform" class="px-2 py-0.5 text-xs rounded" style="background:var(--card);color:var(--accent)">{{ s.platform }}</span>
              </div>
              <pre class="text-xs rounded-md p-3 overflow-x-auto whitespace-pre-wrap break-all max-h-48 overflow-y-auto font-mono" style="background:var(--background);color:var(--foreground)">{{ s.content }}</pre>
            </div>
          </div>
        </div>
      </div>
    </Transition>

    <!-- Config Modal -->
    <Transition name="modal">
      <div v-if="configTool" class="fixed inset-0 z-[70] flex items-center justify-center" @click.self="configTool=null">
        <div class="absolute inset-0" style="background:rgba(0,0,0,0.4)" />
        <div class="relative rounded-2xl shadow-2xl w-full max-w-lg max-h-[80vh] flex flex-col z-10" style="background:var(--bg-card);border:1px solid var(--divider)">
          <div class="flex items-center justify-between p-4 flex-shrink-0" style="border-bottom:1px solid var(--divider)">
            <h3 class="text-lg font-bold" style="color:var(--fg)">{{ configTool.name }}</h3>
            <button @click="configTool=null" class="w-6 h-6 rounded-full flex items-center justify-center text-xs hover:opacity-60" style="background:var(--bg-sidebar-hover);color:var(--fg-secondary)">✕</button>
          </div>
          <div class="flex-1 overflow-y-auto p-4">
            <p v-if="!toolConfigs.length" class="text-sm py-8 text-center" style="color:var(--muted-foreground)">{{ t('home.noConfigs') }}</p>
            <div v-else class="space-y-4">
              <div v-for="c in toolConfigs" :key="c.id" class="rounded-lg p-3" style="background:var(--muted);border:1px solid var(--border-color)">
                <div class="flex items-center justify-between mb-2">
                  <span class="text-sm font-semibold" style="color:var(--foreground)">{{ c.name }}</span>
                  <div class="flex items-center gap-2">
                    <span class="px-2 py-0.5 text-[10px] rounded uppercase font-mono" style="background:var(--card);color:var(--accent)">{{ c.format }}</span>
                    <span class="text-[10px]" style="color:var(--muted-foreground)">{{ t('home.copyCount') }}: {{ c.copy_count }}</span>
                  </div>
                </div>
                <pre class="text-xs rounded-md p-3 overflow-x-auto whitespace-pre-wrap break-all max-h-56 overflow-y-auto font-mono leading-relaxed" style="background:var(--background);color:var(--foreground)">{{ formatContent(c) }}</pre>
                <button @click="copyConfig(c)" class="mt-2 px-3 py-1 text-xs rounded-lg transition-opacity hover:opacity-80" style="background:var(--foreground);color:var(--background)">{{ t('home.copySuccess') }}</button>
              </div>
            </div>
          </div>
        </div>
      </div>
    </Transition>
  </div>
</template>

<script setup>
import { ref, computed, onMounted, watch, nextTick } from 'vue'
import { useI18n } from 'vue-i18n'
import { categoryApi, toolApi, scriptApi, configApi, settingsApi, noteFolderApi, noteApi } from '../api'

const { t, locale } = useI18n()
const isDark = ref(localStorage.getItem('dark') === 'true')
const locked = ref(true)
const pwd = ref('')
const pwdError = ref('')
const categories = ref([])
const tools = ref([])
const selectedCat = ref(0)
const search = ref('')
const loading = ref(false)
const viewMode = ref('app')
const infoTool = ref(null)
const infoTab = ref('info')
const scripts = ref([])
const configTool = ref(null)
const toolConfigs = ref([])

// Notes state
const noteFolders = ref([])
const noteFoldersNotes = ref([])
const openFolders = ref(new Set())
const selectedNote = ref(null)
const noteEditContent = ref('')
const autoSaveState = ref('')
let autoSaveTimer = null

function isUrl(s) { return s && (s.startsWith('http://') || s.startsWith('https://')) }

const filtered = computed(() => {
  let list = tools.value
  if (selectedCat.value) list = list.filter(t => t.category_id === selectedCat.value)
  if (search.value) {
    const q = search.value.toLowerCase()
    list = list.filter(t => t.name.toLowerCase().includes(q) || (t.description || '').toLowerCase().includes(q))
  }
  return list
})

function toggleDark() {
  isDark.value = !isDark.value
  localStorage.setItem('dark', String(isDark.value))
  document.documentElement.classList.toggle('dark')
}

function toggleLocale() {
  const next = locale.value === 'zh' ? 'en' : 'zh'
  locale.value = next
  localStorage.setItem('locale', next)
}

async function verifyPassword() {
  pwdError.value = ''
  try {
    const res = await settingsApi.verifyAccessPassword({ password: pwd.value })
    if (res.data?.verified) {
      sessionStorage.setItem('access_verified', 'true')
      locked.value = false
      loadData()
    } else { pwdError.value = t('home.wrongPassword') }
  } catch (e) { pwdError.value = e.response?.data?.message || t('home.wrongPassword') }
}

async function loadData() {
  loading.value = true
  try {
    const [catRes, toolRes] = await Promise.all([categoryApi.list(), toolApi.list({ page_size: 100 })])
    categories.value = catRes.data || []
    tools.value = toolRes.data?.items || []
  } catch { await loadFallback() } finally { loading.value = false }
}

async function loadFallback() {
  try { const res = await fetch('/data.json'); const data = await res.json(); categories.value = data.categories || []; tools.value = data.tools || [] } catch {}
}

async function openInfo(tool) {
  infoTool.value = tool; infoTab.value = 'info'
  try { const res = await scriptApi.list({ tool_id: tool.id }); scripts.value = res.data?.items || [] } catch { scripts.value = [] }
}

async function openConfigs(tool) {
  configTool.value = tool
  try { const res = await configApi.list({ page_size: 100 }); toolConfigs.value = res.data?.items || [] } catch { toolConfigs.value = [] }
}

function formatContent(c) { if (c.format==='json') { try { return JSON.stringify(JSON.parse(c.content), null, 2) } catch { return c.content } } return c.content }

async function copyConfig(c) {
  try { await navigator.clipboard.writeText(c.content); try { await configApi.copy(c.id) } catch {}; c.copy_count = (c.copy_count||0)+1 } catch {}
}

// Notes functions
async function loadNoteFolders() {
  try { const res = await noteFolderApi.list(); noteFolders.value = res.data || [] } catch { noteFolders.value = [] }
}

async function loadAllNotes() {
  try { const res = await noteApi.list({ page_size: 200 }); noteFoldersNotes.value = res.data?.items || [] } catch { noteFoldersNotes.value = [] }
}

function folderNotes(folderId) { return noteFoldersNotes.value.filter(n => n.folder_id === folderId) }

function toggleFolder(folderId) {
  const s = new Set(openFolders.value)
  if (s.has(folderId)) s.delete(folderId); else s.add(folderId)
  openFolders.value = s
}

function onNoteInput() {
  autoSaveState.value = ''
  if (autoSaveTimer) clearTimeout(autoSaveTimer)
  autoSaveTimer = setTimeout(async () => {
    autoSaveState.value = 'saving'
    try {
      await noteApi.update(selectedNote.value.id, { content: noteEditContent.value })
      selectedNote.value.content = noteEditContent.value
      autoSaveState.value = 'saved'
      setTimeout(() => { autoSaveState.value = '' }, 2000)
    } catch { autoSaveState.value = '' }
  }, 5000)
}

watch(selectedNote, (n) => { if (n) { noteEditContent.value = n.content; autoSaveState.value = '' } })

watch(infoTool, (val) => { if (!val) scripts.value = [] })

watch(viewMode, async (v) => {
  if (v === 'notes') { await loadNoteFolders(); await loadAllNotes() }
})

onMounted(() => {
  if (isDark.value) document.documentElement.classList.add('dark')
  if (sessionStorage.getItem('access_verified') === 'true') { locked.value = false; loadData() }
})
</script>
