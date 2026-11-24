<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/invoke'
import EngineCard from '../components/EngineCard.vue'

// エンジンプロパティの型定義
interface EngineProperty {
  key: string;
  label: string;
  description?: string;
  type: 'boolean' | 'number' | 'string' | 'select' | 'array';
  value: any;
}

// プロパティリスト
const properties = ref<EngineProperty[]>([
  {
    key: "engine_type",
    label: "エンジンタイプ",
    description: "使用するエンジンの種類を指定します",
    type: "select",
    value: "internal_combustion"
  },
  {
    key: "displacement",
    label: "排気量",
    description: "エンジンの排気量（cc）",
    type: "number",
    value: 2000
  },
  {
    key: "power_output",
    label: "出力功率",
    description: "エンジンの最大出力（hp）",
    type: "number",
    value: 150
  },
  {
    key: "fuel_type",
    label: "燃料タイプ",
    description: "使用する燃料の種類",
    type: "select",
    value: "gasoline"
  },
  {
    key: "is_turbocharged",
    label: "ターボチャージャー付き",
    description: "ターボチャージャーが搭載されているか",
    type: "boolean",
    value: false
  },
  {
    key: "compression_ratio",
    label: "圧縮比",
    description: "エンジンの圧縮比",
    type: "number",
    value: 10.5
  }
])

// カードの変更状態を管理
const changes = ref<Record<string, boolean>>({})

// 値が変更されたときに呼び出される関数
const handleValueChange = (key: string, newValue: any) => {
  // 変更を記録
  changes.value[key] = true
  
  // プロパティ値の更新
  const propertyIndex = properties.value.findIndex(p => p.key === key)
  if (propertyIndex !== -1) {
    properties.value[propertyIndex].value = newValue
  }
}

// 変更数を計算する関数
const getChangeCount = () => {
  return Object.values(changes.value).filter(Boolean).length
}

// サンプルデータの読み込み
const loadSampleData = async () => {
  try {
    // TODO: 実際にはTauriのファイル操作APIを使用してJSONを読み込む
    console.log('Loading sample data...')
    
    // 現在はサンプルデータを使用
    properties.value = [
      {
        key: "engine_type",
        label: "エンジンタイプ",
        description: "使用するエンジンの種類を指定します",
        type: "select",
        value: "internal_combustion"
      },
      {
        key: "displacement",
        label: "排気量",
        description: "エンジンの排気量（cc）",
        type: "number",
        value: 2000
      },
      {
        key: "power_output",
        label: "出力功率",
        description: "エンジンの最大出力（hp）",
        type: "number",
        value: 150
      },
      {
        key: "fuel_type",
        label: "燃料タイプ",
        description: "使用する燃料の種類",
        type: "select",
        value: "gasoline"
      },
      {
        key: "is_turbocharged",
        label: "ターボチャージャー付き",
        description: "ターボチャージャーが搭載されているか",
        type: "boolean",
        value: false
      },
      {
        key: "compression_ratio",
        label: "圧縮比",
        description: "エンジンの圧縮比",
        type: "number",
        value: 10.5
      }
    ]
    
  } catch (error) {
    console.error('Failed to load sample data:', error)
  }
}

// サーバーへの保存処理（仮実装）
const saveChanges = async () => {
  try {
    // TODO: 実際にはTauriのファイル操作APIを使用してデータを保存
    console.log('Saving changes...')
    
    const dataToSave = properties.value.reduce((acc, prop) => {
      acc[prop.key] = prop.value
      return acc
    }, {} as Record<string, any>)
    
    // 仮実装としてコンソール出力
    console.log('Data to save:', JSON.stringify(dataToSave, null, 2))
    
    // セーブ成功時の処理（ここでは例示のみ）
    alert('変更が保存されました')
  } catch (error) {
    console.error('Failed to save changes:', error)
    alert('保存に失敗しました')
  }
}

// リセット機能
const resetChanges = () => {
  // 元のデータに戻す
  loadSampleData()
  
  // 変更状態をクリア
  changes.value = {}
  
  console.log('Changes have been reset')
}

onMounted(() => {
  loadSampleData()
})
</script>

<template>
  <div class="main-view">
    <!-- タイトルバー -->
    <header class="title-bar">
      <h1 class="app-title">Engine Simulator</h1>
      
      <!-- 検索バー -->
      <div class="search-container">
        <input 
          type="text" 
          placeholder="キー名で絞り込み..." 
          class="search-input"
        />
      </div>
      
      <!-- ウィンドウ操作ボタン -->
      <div class="window-controls">
        <button class="control-btn minimize">_</button>
        <button class="control-btn maximize">□</button>
        <button class="control-btn close">×</button>
      </div>
    </header>

    <div class="content-wrapper">
      <!-- 左サイドバー -->
      <aside class="sidebar">
        <ul class="category-list">
          <li class="category-item active">エンジン設定</li>
          <li class="category-item">マシン設定</li>
          <li class="category-item">セーブファイル</li>
        </ul>
      </aside>

      <!-- メイン領域 -->
      <main class="main-content">
        <div class="properties-container">
          <EngineCard
            v-for="property in properties"
            :key="property.key"
            :property="property"
            @update:value="(value) => handleValueChange(property.key, value)"
          />
        </div>
        
        <!-- セーブボタン -->
        <div class="action-bar">
          <button 
            class="save-button primary" 
            @click="saveChanges"
          >
            保存
          </button>
          
          <button 
            class="reset-button secondary" 
            @click="resetChanges"
          >
            リセット
          </button>
          
          <div class="change-count">
            {{ getChangeCount() }} 変更
          </div>
        </div>
      </main>
    </div>
  </div>
</template>

<style scoped lang="css">
.main-view {
  display: flex;
  flex-direction: column;
  height: 100vh;
  width: 100vw;
  background-color: var(--background-color);
  color: var(--primary-text);
}

.title-bar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  height: 48px;
  padding: 0 var(--spacing-m);
  background-color: var(--sidebar-background);
  border-bottom: 1px solid var(--border-color);
  user-select: none;
}

.app-title {
  font-size: 18px;
  font-weight: bold;
  color: var(--accent-text);
  margin: 0;
}

.search-container {
  flex: 1;
  max-width: 300px;
  margin: 0 var(--spacing-m);
}

.search-input {
  width: 100%;
  padding: var(--spacing-xxs) var(--spacing-s);
  border-radius: var(--border-radius);
  border: 1px solid var(--border-color);
  font-family: var(--font-family);
  transition: border-color 0.15s ease;
}

.search-input:focus {
  outline: none;
  border-color: var(--accent-color);
}

.window-controls {
  display: flex;
  gap: var(--spacing-xxs);
}

.control-btn {
  width: 32px;
  height: 32px;
  border-radius: 50%;
  background-color: #f0f0f0;
  border: none;
  cursor: pointer;
  font-size: 16px;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.15s ease;
}

.control-btn:hover {
  background-color: #e0e0e0;
}

.minimize { background-color: #f8f8f8; }
.maximize { background-color: #f8f8f8; }
.close { 
  background-color: #ff6b6b; 
  color: white;
}
.close:hover {
  background-color: #ff5252;
}

.content-wrapper {
  display: flex;
  flex: 1;
  overflow: hidden;
}

.sidebar {
  width: 200px;
  background-color: var(--sidebar-background);
  border-right: 1px solid var(--border-color);
  padding-top: var(--spacing-m);
}

.category-list {
  list-style-type: none;
  margin: 0;
  padding: 0;
}

.category-item {
  padding: var(--spacing-xs) var(--spacing-s);
  cursor: pointer;
  transition: all 0.15s ease;
  color: var(--primary-text);
}

.category-item:hover,
.category-item.active {
  background-color: var(--selected-text);
  color: white;
}

.main-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  padding: var(--spacing-m) var(--spacing-l);
  overflow-y: auto;
}

.properties-container {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-s);
  margin-bottom: var(--spacing-xl);
}

.action-bar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--spacing-m) 0;
  border-top: 1px solid var(--border-color);
  position: sticky;
  bottom: 0;
  background-color: var(--background-color);
  margin-top: auto;
}

.save-button {
  background-color: var(--accent-color);
  color: var(--accent-text);
  padding: var(--spacing-s) var(--spacing-l);
  border-radius: 50px;
  border: none;
  font-size: 16px;
  font-weight: bold;
  cursor: pointer;
  transition: all 0.15s ease;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.save-button:hover {
  background-color: var(--accent-hover);
  transform: translateY(-2px);
  box-shadow: 0 3px 6px rgba(0, 0, 0, 0.15);
}

.reset-button {
  padding: var(--spacing-s) var(--spacing-l);
  border-radius: var(--border-radius);
  border: 1px solid var(--border-color);
  background-color: white;
  color: var(--primary-text);
  cursor: pointer;
  transition: all 0.15s ease;
}

.reset-button:hover {
  background-color: #f0f0f0;
}

.change-count {
  color: var(--secondary-text);
  font-size: 14px;
}
</style>