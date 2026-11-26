<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { NConfigProvider, NInput, NButton, NCard, NSwitch, NSlider, NSelect, NTag } from 'naive-ui'
import { invoke } from '@tauri-apps/api'

const themeOverrides = {
  common: {
    primaryColor: '#FFC81E',
    primaryColorHover: '#FFD54A',
    primaryColorPressed: '#E6B000'
  },
  Button: { borderRadius: '8px', borderRadiusLarge: '999px' },
  Card: { borderRadius: '8px' },
  Input: { borderRadius: '8px' },
  Select: { borderRadius: '8px' }
}

type ParamType = 'bool' | 'number' | 'string' | 'select'
interface Param {
  key: string
  label: string
  type: ParamType
  value: any
  default: any
  unit?: string
  description?: string
  category?: string
  options?: string[]
}

const sampleParams: Param[] = [
  { key: 'starter_torque', label: 'スタータートルク', type: 'number', value: 70, default: 70, unit: 'lb·ft', description: 'セルがクランクを回す力。', category: '基本' },
  { key: 'starter_speed', label: 'スターター回転数', type: 'number', value: 500, default: 500, unit: 'rpm', description: '始動時の目標回転数。', category: '基本' },
  { key: 'throttle_gamma', label: 'スロットル応答', type: 'number', value: 2.0, default: 2.0, unit: '係数', description: '大きいほど低開度が穏やかになります。', category: '基本' },
  { key: 'vtec_enabled', label: 'VTECを有効化', type: 'bool', value: false, default: false, description: 'true にすると高回転側カムへ切り替えます。', category: 'カム・バルブ' },
  { key: 'stroke', label: 'ストローク', type: 'number', value: 86, default: 86, unit: 'mm', description: 'ピストン移動量。', category: '回転部' },
  { key: 'engine_name', label: 'エンジン名', type: 'string', value: 'sample', default: 'sample', description: 'エンジンの識別名', category: '基本' },
  { key: 'layout', label: 'レイアウト', type: 'select', value: '直列', default: '直列', description: 'エンジンレイアウト', category: '基本', options: ['直列','V型','水平対向'] },
  { key: 'cylinders', label: '気筒数', type: 'number', value: 4, default: 4, description: '気筒数', category: '基本' },
]

const params = ref<Param[]>(sampleParams)
const search = ref('')
const selectedCategory = ref('all')
const categories = computed(() => ['all', ...Array.from(new Set(params.value.map(p=>p.category||'その他')))])
const filtered = computed(() => {
  return params.value.filter(p => {
    const matchSearch = !search.value || p.key.includes(search.value) || p.label.includes(search.value)
    const matchCat = selectedCategory.value==='all' || p.category===selectedCategory.value
    return matchSearch && matchCat
  })
})
const changedCount = computed(() => params.value.filter(p=>JSON.stringify(p.value)!==JSON.stringify(p.default)).length)

function isChanged(p: Param){ return JSON.stringify(p.value)!==JSON.stringify(p.default) }
function save() {
  const payload = JSON.stringify({name: params.value.find(p=>p.key==='engine_name')?.value||'sample', parameters: params.value})
  // Tauri command placeholder
}
function reset() {
  params.value.forEach(p=>p.value=p.default)
}

onMounted(async ()=>{
  // if Tauri available, try to load sample file
})
</script>

<template>
  <n-config-provider :theme-overrides="themeOverrides">
    <div style="height:100vh;display:flex;flex-direction:column;">
      <header style="height:48px;background:#fff;border-bottom:1px solid var(--border);display:flex;align-items:center;padding:0 12px;gap:12px;user-select:none;">
        <div data-tauri-drag-region style="font-weight:600;">Engine Sim GUI</div>
        <div style="flex:1;display:flex;justify-content:center;">
          <n-input v-model:value="search" placeholder="キー名で絞り込み" style="width:300px;border-radius:999px;" />
        </div>
        <div style="display:flex;gap:4px;">
          <n-button size="small" @click="window.close()">×</n-button>
        </div>
      </header>
      <div style="flex:1;display:flex;overflow:hidden;">
        <aside style="width:200px;background:var(--sidebar-bg);border-right:1px solid var(--border);overflow:auto;padding:8px;">
          <div v-for="cat in categories" :key="cat" style="padding:6px 8px;cursor:pointer;border-radius:6px;margin-bottom:4px;"
            :style="{background:selectedCategory===cat?'var(--selected-bg)':'transparent',color:selectedCategory===cat?'var(--selected-text)':'#333'}"
            @click="selectedCategory=cat">{{ cat }}</div>
        </aside>
        <main style="flex:1;overflow:auto;padding:16px;display:flex;flex-direction:column;gap:12px;">
          <n-card v-for="p in filtered" :key="p.key" :title="p.label" style="border-left:4px solid transparent;"
            :style="{borderLeftColor:isChanged(p)?'var(--accent)':'transparent',background:'var(--card-bg)'}">
            <template #header>
              <div style="font-size:14px;">{{ p.key }}</div>
              <div style="font-size:12px;color:var(--text-secondary);margin-top:2px;">{{ p.description }}</div>
            </template>
            <div style="display:flex;align-items:center;gap:12px;">
              <div style="flex:1;">
                <n-switch v-if="p.type==='bool'" v-model:value="p.value" />
                <div v-else-if="p.type==='number'" style="display:flex;gap:8px;align-items:center;">
                  <n-slider v-model:value="p.value" :min="0" :max="1000" style="flex:1"/>
                  <n-input-number v-model:value="p.value" style="width:100px"/>
                  <span v-if="p.unit" style="font-size:12px;color:var(--text-secondary)">{{ p.unit }}</span>
                </div>
                <n-input v-else-if="p.type==='string'" v-model:value="p.value" />
                <n-select v-else-if="p.type==='select'" v-model:value="p.value" :options="p.options?.map(o=>({label:o,value:o}))" />
              </div>
              <n-tag v-if="isChanged(p)" type="warning" size="small">変更</n-tag>
            </div>
          </n-card>
        </main>
      </div>
      <footer style="height:64px;border-top:1px solid var(--border);display:flex;align-items:center;justify-content:space-between;padding:0 16px;background:#fff;">
        <div>変更件数: {{ changedCount }}</div>
        <div style="display:flex;gap:8px;">
          <n-button type="primary" style="border-radius:999px;" @click="save">保存</n-button>
          <n-button @click="reset">リセット</n-button>
        </div>
      </footer>
    </div>
  </n-config-provider>
</template>
