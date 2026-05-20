<template>
  <!-- Access Password Gate -->
  <div v-if="locked" class="h-screen bg-background flex items-center justify-center">
    <div class="w-full max-w-md mx-auto p-8">
      <div class="bg-card rounded-2xl border border-border p-10 shadow-sm text-center">
        <h2 class="text-3xl font-bold mb-2 text-center">{{ t('home.welcome') }}</h2>
        <p class="text-sm text-muted-foreground text-center mb-8">{{ t('home.hint') }}</p>
        <form @submit.prevent="verifyPassword" class="space-y-4">
          <input v-model="pwd" type="password" :placeholder="t('home.password')" class="w-full px-4 py-3 rounded-lg border border-border bg-background focus:outline-none focus:ring-2 focus:ring-primary/50 text-center text-lg tracking-widest" required />
          <p v-if="pwdError" class="text-red-500 text-sm text-center">{{ pwdError }}</p>
          <button type="submit" class="w-full py-3 bg-primary text-primary-foreground rounded-lg hover:opacity-90 transition font-medium">{{ t('home.submit') }}</button>
        </form>
      </div>
    </div>
  </div>

  <!-- Main Content -->
  <div v-else class="h-screen flex flex-col overflow-hidden">
    <header class="flex-shrink-0 h-12 bg-background/90 backdrop-blur-md border-b border-border z-50">
      <div class="max-w-[1400px] mx-auto h-full flex items-center justify-between px-2">
        <span class="text-base font-bold tracking-tight pl-1">APP</span>
        <div class="flex items-center gap-2">
          <button @click="viewMode = viewMode === 'app' ? 'config' : 'app'" class="px-2 py-0.5 text-xs rounded border border-border hover:bg-muted transition-colors" :title="viewMode === 'app' ? t('home.viewConfig') : t('home.viewApp')">{{ viewMode === 'app' ? '⚙' : '📋' }}</button>
          <button @click="toggleLocale" class="px-2 py-0.5 text-xs rounded border border-border hover:bg-muted transition-colors">{{ locale === 'zh' ? '中' : 'EN' }}</button>
          <button @click="toggleDark" class="px-2 py-0.5 text-xs rounded border border-border hover:bg-muted transition-colors">{{ isDark ? '☀' : '☾' }}</button>
        </div>
      </div>
    </header>

    <div class="flex-1 flex justify-center overflow-hidden">
      <div class="w-full max-w-[1400px] flex overflow-hidden relative">
        <aside class="flex-shrink-0 w-44 border-r border-border overflow-y-auto py-3 px-2">
          <button @click="selectedCat = 0" :class="selectedCat === 0 ? 'bg-primary text-primary-foreground font-medium' : 'text-muted-foreground hover:bg-muted'" class="w-full text-left px-3 py-2 rounded-lg text-sm transition-colors truncate mb-0.5">{{ t('home.all') }}</button>
          <button v-for="cat in categories" :key="cat.id" @click="selectedCat = cat.id" :class="selectedCat === cat.id ? 'bg-primary text-primary-foreground font-medium' : 'text-muted-foreground hover:bg-muted'" class="w-full text-left px-3 py-2 rounded-lg text-sm transition-colors truncate mb-0.5">{{ cat.icon }} {{ cat.name }}</button>
        </aside>

        <main class="flex-1 overflow-y-auto p-5">
          <div class="mb-4">
            <input v-model="search" :placeholder="t('home.search')" class="w-full max-w-md px-4 py-2 rounded-lg border border-border bg-background text-sm focus:outline-none focus:ring-2 focus:ring-primary/30" />
          </div>

          <!-- App View -->
          <template v-if="viewMode === 'app'">
            <div v-if="filtered.length" class="grid grid-cols-7 gap-3">
              <div v-for="tool in filtered" :key="tool.id" class="group aspect-square bg-card rounded-xl border border-border p-3 flex flex-col items-center justify-center gap-2 hover:bg-muted/60 hover:border-primary/30 transition-all cursor-pointer relative">
                <button @click.stop="openInfo(tool)" class="absolute top-2 right-2 w-5 h-5 rounded-full bg-muted text-muted-foreground flex items-center justify-center text-[10px] hover:bg-primary hover:text-primary-foreground transition-colors z-10" title="Detail">i</button>
                <a :href="tool.url" target="_blank" @click.stop class="flex flex-col items-center justify-center gap-2 w-full h-full no-underline text-foreground">
                  <div class="w-11 h-11 rounded-lg bg-muted flex items-center justify-center text-xl flex-shrink-0 group-hover:scale-110 transition-transform overflow-hidden">
                    <img v-if="isUrl(tool.icon)" :src="tool.icon" class="w-full h-full object-cover rounded-lg" />
                    <span v-else>{{ tool.icon || '🔗' }}</span>
                  </div>
                  <span class="text-xs font-medium text-center leading-tight line-clamp-2 w-full">{{ tool.name }}</span>
                </a>
              </div>
            </div>
            <p v-else-if="loading" class="text-center text-muted-foreground py-20 text-sm">Loading...</p>
            <p v-else class="text-center text-muted-foreground py-20 text-sm">{{ t('home.empty') }}</p>
          </template>

          <!-- Config View -->
          <template v-else>
            <div v-if="filtered.length" class="grid grid-cols-7 gap-3">
              <div v-for="tool in filtered" :key="tool.id" @click="openConfigs(tool)" class="group aspect-square bg-card rounded-xl border border-border p-3 flex flex-col items-center justify-center gap-2 hover:bg-muted/60 hover:border-primary/30 transition-all cursor-pointer">
                <div class="flex flex-col items-center justify-center gap-2 w-full h-full">
                  <div class="w-11 h-11 rounded-lg bg-muted flex items-center justify-center text-xl flex-shrink-0 group-hover:scale-110 transition-transform overflow-hidden">
                    <img v-if="isUrl(tool.icon)" :src="tool.icon" class="w-full h-full object-cover rounded-lg" />
                    <span v-else>{{ tool.icon || '🔗' }}</span>
                  </div>
                  <span class="text-xs font-medium text-center leading-tight line-clamp-2 w-full">{{ tool.name }}</span>
                </div>
              </div>
            </div>
            <p v-else-if="loading" class="text-center text-muted-foreground py-20 text-sm">Loading...</p>
            <p v-else class="text-center text-muted-foreground py-20 text-sm">{{ t('home.empty') }}</p>
          </template>
        </main>
      </div>
    </div>

    <footer class="flex-shrink-0 h-9 bg-background/90 backdrop-blur-md border-t border-border z-50">
      <div class="max-w-[1400px] mx-auto h-full flex items-center justify-center gap-4 text-xs text-muted-foreground">
        <span>Powered by Apps Startpage</span>
        <span>&copy; {{ new Date().getFullYear() }}</span>
      </div>
    </footer>

    <!-- Info Drawer -->
    <Transition name="drawer">
      <div v-if="infoTool" class="fixed top-0 right-0 bottom-0 w-80 bg-card border-l border-border overflow-y-auto z-[60] shadow-xl flex flex-col">
        <div class="flex border-b border-border flex-shrink-0">
          <button @click="infoTab = 'info'" :class="infoTab === 'info' ? 'border-b-2 border-primary font-semibold text-foreground' : 'text-muted-foreground'" class="flex-1 py-2.5 text-sm text-center transition-colors">{{ t('home.tabInfo') }}</button>
          <button @click="infoTab = 'script'" :class="infoTab === 'script' ? 'border-b-2 border-primary font-semibold text-foreground' : 'text-muted-foreground'" class="flex-1 py-2.5 text-sm text-center transition-colors">{{ t('home.tabScript') }}</button>
        </div>
        <button @click="infoTool = null" class="absolute top-2 right-2 w-6 h-6 rounded-full bg-muted flex items-center justify-center text-xs hover:bg-primary hover:text-primary-foreground transition-colors z-10">✕</button>

        <div v-if="infoTab === 'info'" class="p-5">
          <h3 class="text-lg font-bold mb-4 pr-8">{{ infoTool.name }}</h3>
          <div class="w-16 h-16 rounded-xl bg-muted flex items-center justify-center text-3xl mb-4 overflow-hidden">
            <img v-if="isUrl(infoTool.icon)" :src="infoTool.icon" class="w-full h-full object-cover rounded-xl" />
            <span v-else>{{ infoTool.icon || '🔗' }}</span>
          </div>
          <p v-if="infoTool.description" class="text-sm text-muted-foreground mb-4">{{ infoTool.description }}</p>
          <div v-if="infoTool.url" class="mb-4"><a :href="infoTool.url" target="_blank" class="text-sm text-primary hover:underline break-all">{{ infoTool.url }}</a></div>
          <div v-if="infoTool.tags?.length" class="flex flex-wrap gap-1 mb-3">
            <span v-for="tag in infoTool.tags" :key="tag" class="px-2 py-0.5 text-xs rounded bg-muted text-muted-foreground">{{ tag }}</span>
          </div>
          <div v-if="infoTool.platforms?.length" class="flex flex-wrap gap-1 mb-4">
            <span v-for="p in infoTool.platforms" :key="p" class="px-2 py-0.5 text-xs rounded bg-muted text-muted-foreground">{{ p }}</span>
          </div>
          <div v-if="infoTool.versions?.length">
            <h4 class="text-sm font-semibold mb-2">{{ t('home.versions') }}</h4>
            <div class="space-y-1">
              <div v-for="(v, i) in infoTool.versions" :key="i" class="flex items-center gap-2 text-sm">
                <span class="text-muted-foreground w-6 text-right flex-shrink-0">{{ i + 1 }}.</span>
                <a v-if="v.url" :href="v.url" target="_blank" class="text-primary hover:underline">{{ v.version }}</a>
                <span v-else>{{ v.version }}</span>
              </div>
            </div>
          </div>
        </div>

        <div v-else class="p-5">
          <p v-if="!scripts.length" class="text-sm text-muted-foreground py-8 text-center">{{ t('home.noScripts') }}</p>
          <div v-else class="space-y-4">
            <div v-for="s in scripts" :key="s.id" class="bg-muted/40 rounded-lg border border-border p-3">
              <div class="flex items-center justify-between mb-2">
                <span class="text-sm font-semibold">{{ s.name }}</span>
                <span v-if="s.platform" class="px-2 py-0.5 text-xs rounded bg-primary/10 text-primary">{{ s.platform }}</span>
              </div>
              <div v-if="s.tags?.length" class="flex flex-wrap gap-1 mb-2">
                <span v-for="tag in s.tags" :key="tag" class="px-1.5 py-0.5 text-[10px] rounded bg-muted text-muted-foreground">{{ tag }}</span>
              </div>
              <pre class="text-xs bg-background rounded-md p-3 overflow-x-auto whitespace-pre-wrap break-all max-h-48 overflow-y-auto font-mono">{{ s.content }}</pre>
            </div>
          </div>
        </div>
      </div>
    </Transition>

    <!-- Config Modal -->
    <Transition name="modal">
      <div v-if="configTool" class="fixed inset-0 z-[70] flex items-center justify-center" @click.self="configTool = null">
        <div class="absolute inset-0 bg-black/40" />
        <div class="relative bg-card rounded-2xl border border-border shadow-2xl w-full max-w-lg max-h-[80vh] flex flex-col z-10">
          <div class="flex items-center justify-between p-4 border-b border-border flex-shrink-0">
            <h3 class="text-lg font-bold">{{ configTool.name }}</h3>
            <button @click="configTool = null" class="w-6 h-6 rounded-full bg-muted flex items-center justify-center text-xs hover:bg-primary hover:text-primary-foreground transition-colors">✕</button>
          </div>
          <div class="flex-1 overflow-y-auto p-4">
            <p v-if="!toolConfigs.length" class="text-sm text-muted-foreground py-8 text-center">{{ t('home.noConfigs') }}</p>
            <div v-else class="space-y-4">
              <div v-for="c in toolConfigs" :key="c.id" class="bg-muted/30 rounded-lg border border-border p-3">
                <div class="flex items-center justify-between mb-2">
                  <span class="text-sm font-semibold">{{ c.name }}</span>
                  <div class="flex items-center gap-2">
                    <span class="px-2 py-0.5 text-[10px] rounded bg-primary/10 text-primary uppercase font-mono">{{ c.format }}</span>
                    <span class="text-[10px] text-muted-foreground">{{ t('home.copyCount') }}: {{ c.copy_count }}</span>
                  </div>
                </div>
                <pre class="text-xs bg-background rounded-md p-3 overflow-x-auto whitespace-pre-wrap break-all max-h-56 overflow-y-auto font-mono leading-relaxed">{{ formatContent(c) }}</pre>
                <button @click="copyConfig(c)" class="mt-2 px-3 py-1 text-xs bg-primary text-primary-foreground rounded hover:opacity-90 transition">{{ t('home.copySuccess') }}</button>
              </div>
            </div>
          </div>
        </div>
      </div>
    </Transition>
  </div>
</template>

<script setup>
import { ref, computed, onMounted, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { categoryApi, toolApi, scriptApi, configApi, settingsApi } from '../api'

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
const offline = ref(false)

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
    } else {
      pwdError.value = t('home.wrongPassword')
    }
  } catch (e) {
    pwdError.value = e.response?.data?.message || t('home.wrongPassword')
  }
}

async function loadData() {
  loading.value = true
  try {
    const [catRes, toolRes] = await Promise.all([categoryApi.list(), toolApi.list({ page_size: 100 })])
    categories.value = catRes.data || []
    tools.value = toolRes.data?.items || []
  } catch {
    offline.value = true
    await loadFallback()
  } finally {
    loading.value = false
  }
}

async function loadFallback() {
  try {
    const res = await fetch('/data.json')
    const data = await res.json()
    categories.value = data.categories || []
    tools.value = data.tools || []
  } catch {}
}

async function openInfo(tool) {
  infoTool.value = tool
  infoTab.value = 'info'
  if (!offline.value) {
    try {
      const res = await scriptApi.list({ tool_id: tool.id })
      scripts.value = res.data?.items || []
    } catch { scripts.value = [] }
  } else {
    scripts.value = []
  }
}

async function openConfigs(tool) {
  configTool.value = tool
  if (!offline.value) {
    try {
      const res = await configApi.list()
      toolConfigs.value = res.data?.items || []
    } catch { toolConfigs.value = [] }
  } else {
    toolConfigs.value = []
  }
}

function formatContent(c) {
  if (c.format === 'json') {
    try { return JSON.stringify(JSON.parse(c.content), null, 2) } catch { return c.content }
  }
  return c.content
}

async function copyConfig(c) {
  try {
    await navigator.clipboard.writeText(c.content)
    if (!offline.value) {
      try { await configApi.copy(c.id) } catch {}
    }
    c.copy_count = (c.copy_count || 0) + 1
  } catch {}
}

watch(infoTool, (val) => { if (!val) scripts.value = [] })

onMounted(() => {
  if (isDark.value) document.documentElement.classList.add('dark')
  if (offline.value || sessionStorage.getItem('access_verified') === 'true') {
    locked.value = false
    offline.value ? loadFallback() : loadData()
  } else {
    loadData().then(() => { if (offline.value) locked.value = false })
  }
})
</script>
