<template>
  <MainLayout>
    <div>
      <h1 class="text-[28px] font-semibold tracking-tight mb-6" style="color:var(--fg)">{{ t('adminUsers.title') }}</h1>
      <div class="apple-card overflow-hidden">
        <table class="apple-table">
          <thead>
            <tr>
              <th>ID</th>
              <th>{{ t('adminUsers.username') }}</th>
              <th>{{ t('adminUsers.email') }}</th>
              <th>{{ t('adminUsers.role') }}</th>
              <th>{{ t('adminUsers.isActive') }}</th>
              <th class="text-right">{{ t('adminUsers.action') }}</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="u in users" :key="u.id">
              <td style="color:var(--fg-tertiary)">{{ u.id }}</td>
              <td><span class="font-medium" style="color:var(--fg)">{{ u.username }}</span></td>
              <td style="color:var(--fg-secondary)">{{ u.email }}</td>
              <td><span class="inline-flex items-center px-2 py-0.5 rounded-full text-xs font-medium" :style="u.role==='admin'?'background:rgba(0,125,255,0.1);color:var(--accent)':'background:var(--bg-sidebar-hover);color:var(--fg-secondary)'">{{ u.role === 'admin' ? t('adminUsers.admin') : t('adminUsers.user') }}</span></td>
              <td><span class="inline-flex items-center gap-1 text-xs"><span class="w-1.5 h-1.5 rounded-full" :class="u.is_active?'bg-apple-green':'bg-apple-red'"></span>{{ u.is_active ? t('adminUsers.active') : t('adminUsers.inactive') }}</span></td>
              <td class="text-right">
                <div class="flex items-center justify-end gap-1">
                  <button v-if="u.is_active" @click="toggleActive(u,false)" class="apple-btn-danger text-xs py-1 px-2">{{ t('adminUsers.disable') }}</button>
                  <button v-else @click="toggleActive(u,true)" class="apple-btn-secondary text-xs py-1 px-2" style="color:var(--accent)">{{ t('adminUsers.enable') }}</button>
                  <button v-if="u.role!=='admin'" @click="toggleRole(u)" class="apple-btn-secondary text-xs py-1 px-2">{{ t('adminUsers.admin') }}</button>
                  <button v-if="u.role==='admin'&&u.username!=='admin'" @click="toggleRole(u)" class="apple-btn-secondary text-xs py-1 px-2">{{ t('adminUsers.user') }}</button>
                  <button @click="deleteUser(u)" class="apple-btn-danger text-xs py-1 px-2">{{ t('adminUsers.deleteUser') }}</button>
                </div>
              </td>
            </tr>
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
import { userApi } from '../api'

const { t } = useI18n()
const users = ref([])
const page = ref(1)
const total = ref(0)
const pageSize = 20
const totalPages = computed(() => Math.ceil(total.value / pageSize))

onMounted(fetch)

async function fetch() { try { const res = await userApi.list({ page: page.value, page_size: pageSize }); users.value = res.data.items || []; total.value = res.data.total || 0 } catch {} }
async function toggleActive(u, val) { try { await userApi.update(u.id, { is_active: val }); fetch() } catch (e) { alert(e.response?.data?.detail || 'Error') } }
async function toggleRole(u) { try { await userApi.update(u.id, { role: u.role === 'admin' ? 'user' : 'admin' }); fetch() } catch (e) { alert(e.response?.data?.detail || 'Error') } }
async function deleteUser(u) { if (!confirm(t('adminUsers.confirmDelete'))) return; try { await userApi.delete(u.id); fetch() } catch (e) { alert(e.response?.data?.detail || 'Error') } }
</script>
