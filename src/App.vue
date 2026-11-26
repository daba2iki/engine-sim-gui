<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import { NConfigProvider, NButton } from 'naive-ui'
import { invoke } from '@tauri-apps/api'
import TitleBar from './components/TitleBar.vue'
import Sidebar from './components/Sidebar.vue'
import ParameterCard from './components/ParameterCard.vue'
import type { EngineDoc, Param } from './types'

const fallbackDoc: EngineDoc = {
  name: 'sample-engine',
  parameters: [
    { key: 'engine_name', label: 'エンジン名', type: 'string', value: 'sample-engine', default: 'sample-engine', description: 'エンジンの識別名。', category: '基本' },
    { key: 'cylinders', label: '気筒数', type: 'number', value: 4, default: 4, description: 'シリンダー気筒数。', category: '基本' },
    { key: 'starter_torque', label: 'スタータートルク', type: 'number', value: 70, default: 70, unit: 'lb·ft', description: 'セルがクランクを回す力。', category: '基本' },
    { key: 'vtec_enabled', label: 'VTECを有効化', type: 'bool', value: false, default: false, description: 'true にすると高回転側カムへ切り替えます。', category: 'カム・バルブ' }
  ]
}

const params = ref<Param[]>([])
const search = ref('')
const selectedCategory = ref('all')
const files = ref<string[]>([])
const currentFile = ref<string | null>(null)
const status = ref('サンプルを読み込みます…')

const categories = computed(() => {
  const cats = Array.from(new Set(params.value.map((p) => p.category || 'その他')))
  return ['すべて', ...cats]
})

const filtered = computed(() => {
  const q = search.value.trim().toLowerCase()
  return params.value.filter((p) => {
    const matchesCategory = selectedCategory.value === 'すべて' || (p.category || 'その他') === selectedCategory.value
    const matchesSearch =
      !q ||
      p.key.toLowerCase().includes(q) ||
      p.label.toLowerCase().includes(q) ||
      (p.description || '').toLowerCase().includes(q)
    return matchesCategory && matchesSearch
  })
})

const changedCount = computed(() => params.value.filter(isChanged).length)

function isChanged(p: Param) {
  return JSON.stringify(p.value) !== JSON.stringify(p.default)
}

function normalizeDoc(data: any): EngineDoc {
  if (Array.isArray(data)) return { parameters: data as Param[] }
  if (data && Array.isArray(data.parameters)) return data as EngineDoc
  return fallbackDoc
}

function setParams(doc: EngineDoc) {
  params.value = doc.parameters.map((p) => ({
    ...p,
    value: p.value,
    default: p.default
  }))
}

async function loadFile(file: string) {
  currentFile.value = file
  try {
    const raw = await invoke<string>('read_json', { path: file })
    setParams(normalizeDoc(JSON.parse(raw)))
    status.value = `${file} を読み込みました`
  } catch (e) {
    const message = e instanceof Error ? e.message : String(e)
    console.warn('Tauri load failed, trying browser fallback', message)
    try {
      const res = await fetch(file)
      setParams(normalizeDoc(await res.json()))
      status.value = `${file} を読み込みました (fallback)`
    } catch (fallbackError) {
      setParams(fallbackDoc)
      status.value = `読み込みに失敗しました: ${message}`
    }
  }
}

async function save() {
  const payload = JSON.stringify({ name: params.value.find((p) => p.key === 'engine_name')?.value || 'engine', parameters: params.value }, null, 2)
  const target = currentFile.value || 'sample/sample.json'
  try {
    await invoke('write_json', { path: target, content: payload })
    status.value = `${target} に保存しました`
  } catch (e) {
    status.value = `保存に失敗しました: ${e instanceof Error ? e.message : String(e)}`
  }
}

function reset() {
  params.value.forEach((p) => {
    p.value = p.default
  })
  status.value = 'デフォルト値に戻しました'
}

async function listFiles() {
  try {
    const listed = await invoke<string[]>('list_json_files', { dir: 'sample' })
    files.value = listed.map((f) => `sample/${f}`)
  } catch {
    files.value = ['sample/sample.json']
  }
}

onMounted(async () => {
  await listFiles()
  await loadFile(files.value[0] || 'sample/sample.json')
})
</script>

<template>
  <NConfigProvider :theme-overrides="{
    common: {
      primaryColor: 'var(--accent)',
      primaryColorHover: 'var(--accent-hover)',
      primaryColorPressed: 'var(--accent-active)',
      primaryColorSuppl: 'var(--accent-hover)'
    },
    Button: {
      borderRadius: '8px',
      colorPrimary: 'var(--accent-text)',
      colorPrimaryHover: 'var(--accent-text)',
      colorPrimaryPressed: 'var(--accent-text)',
      colorFocus: 'var(--accent-text)'
    },
    Input: {
      borderRadius: '8px',
      borderFocus: 'var(--accent)',
      boxShadowFocus: '0 0 0 2px rgba(255, 200, 30, 0.20)',
      colorFocus: 'var(--selected-bg)'
    },
    InputNumber: {
      borderRadius: '8px'
    },
    Switch: {
      railColorActive: 'var(--accent)',
      railColorHoverActive: 'var(--accent-hover)',
      railColorPressedActive: 'var(--accent-active)',
      nodeColorChecked: '#FFFFFF',
      nodeColorCheckedHover: '#FFFFFF',
      nodeColorCheckedPressed: '#FFFFFF'
    },
    Slider: {
      railFillColor: 'var(--accent)',
      railFillColorHover: 'var(--accent-hover)',
      handleColor: 'var(--accent)',
      handleColorHover: 'var(--accent-hover)',
      handleColorPressed: 'var(--accent-active)'
    },
    Select: {
      borderRadius: '8px',
      borderHover: 'var(--accent)',
      borderActive: 'var(--accent)',
      boxShadowFocus: '0 0 0 2px rgba(255, 200, 30, 0.20)'
    }
  }">
    <div class="app-shell">
      <TitleBar v-model:search="search" />

      <div class="body">
        <Sidebar
          :files="files"
          :current-file="currentFile"
          :categories="categories"
          :selected-category="selectedCategory"
          @select-file="loadFile"
          @select-category="selectedCategory = $event"
        />

        <main class="main">
          <ParameterCard
            v-for="p in filtered"
            :key="p.key"
            :param="p"
            :changed="isChanged(p)"
            @update:value="(v: Param['value']) => (p.value = v)"
          />
          <div v-if="filtered.length === 0" class="empty-state">
            表示できる項目がありません。検索条件やカテゴリを確認してください。
          </div>
        </main>
      </div>

      <footer class="actionbar">
        <div class="actionbar-left">
          <span class="status-text">{{ status }}</span>
          <span class="status-text">変更件数: {{ changedCount }}</span>
        </div>
        <div class="actions">
          <NButton @click="reset">リセット</NButton>
          <NButton type="primary" class="pill" @click="save">保存</NButton>
        </div>
      </footer>
    </div>
  </NConfigProvider>
</template>
