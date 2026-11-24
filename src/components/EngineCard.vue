<script setup lang="ts">
import { defineProps } from 'vue'

// プロパティの型定義
interface EngineProperty {
  key: string;
  label: string;
  description?: string;
  type: 'boolean' | 'number' | 'string' | 'select' | 'array';
  value: any;
}

const props = defineProps<{
  property: EngineProperty;
}>()

// 編集イベント
const emit = defineEmits(['update:value'])

const handleValueChange = (value: any) => {
  emit('update:value', value)
}
</script>

<template>
  <div class="engine-card">
    <div class="card-header">
      <!-- 変更マーク -->
      <div class="change-indicator"></div>
      
      <!-- キー名と説明 -->
      <div class="property-info">
        <h3>{{ property.label }}</h3>
        <p v-if="property.description" class="description">{{ property.description }}</p>
      </div>
    </div>

    <!-- 編集部品 -->
    <div class="edit-controls">
      <template v-if="property.type === 'boolean'">
        <input 
          type="checkbox" 
          :checked="property.value"
          @change="handleValueChange(!property.value)"
          class="toggle-switch"
        />
      </template>
      
      <template v-else-if="property.type === 'number'">
        <input 
          type="range" 
          :value="property.value"
          @input="handleValueChange($event.target.valueAsNumber)"
          class="slider"
        />
        <input 
          type="number" 
          :value="property.value"
          @input="handleValueChange($event.target.valueAsNumber)"
          class="number-input"
        />
      </template>
      
      <template v-else-if="property.type === 'string'">
        <input 
          type="text" 
          :value="property.value"
          @input="handleValueChange($event.target.value)"
          class="text-input"
        />
      </template>
      
      <template v-else-if="property.type === 'select'">
        <select 
          :value="property.value"
          @change="handleValueChange($event.target.value)"
          class="select-input"
        >
          <!-- 選択肢は実際にはデータから生成 -->
          <option value="option1">Option 1</option>
          <option value="option2">Option 2</option>
        </select>
      </template>
      
      <template v-else-if="property.type === 'array'">
        <!-- 配列/オブジェクト用の折りたたみカード -->
        <div class="accordion">
          <details>
            <summary>詳細を表示</summary>
            <p>配列またはオブジェクトの内容がここに表示されます</p>
          </details>
        </div>
      </template>
    </div>
  </div>
</template>

<style scoped lang="css">
.engine-card {
  display: flex;
  flex-direction: column;
  background-color: var(--card-background);
  border-radius: var(--border-radius);
  padding: var(--spacing-m);
  margin-bottom: var(--spacing-s);
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.05);
  position: relative;
}

.change-indicator {
  position: absolute;
  left: 0;
  top: 0;
  bottom: 0;
  width: 4px;
  background-color: var(--accent-color);
  border-radius: 2px 0 0 2px;
}

.card-header {
  display: flex;
  align-items: flex-start;
  margin-bottom: var(--spacing-s);
}

.property-info h3 {
  color: var(--primary-text);
  font-size: 16px;
  margin: 0 0 var(--spacing-xxs) 0;
}

.description {
  color: var(--secondary-text);
  font-size: 14px;
  margin: 0;
}

.edit-controls {
  display: flex;
  align-items: center;
  gap: var(--spacing-s);
  width: 100%;
}

.toggle-switch {
  height: 24px;
  width: 48px;
  -webkit-appearance: none;
  border-radius: 12px;
  background-color: #ccc;
  position: relative;
  cursor: pointer;
}

.toggle-switch:checked {
  background-color: var(--accent-color);
}

.toggle-switch::before {
  content: '';
  position: absolute;
  height: 20px;
  width: 20px;
  border-radius: 50%;
  background-color: white;
  top: 2px;
  left: 2px;
  transition: transform 0.15s ease;
}

.toggle-switch:checked::before {
  transform: translateX(24px);
}

.slider {
  flex: 1;
  height: 6px;
  border-radius: 3px;
  background-color: #ddd;
  -webkit-appearance: none;
}

.number-input {
  width: 80px;
  padding: var(--spacing-xxs) var(--spacing-xs);
  border-radius: var(--border-radius);
  border: 1px solid var(--border-color);
  text-align: center;
}

.text-input {
  flex: 1;
  padding: var(--spacing-xxs) var(--spacing-xs);
  border-radius: var(--border-radius);
  border: 1px solid var(--border-color);
  font-family: var(--font-family);
  transition: border-color 0.15s ease;
}

.text-input:focus {
  outline: none;
  border-color: var(--accent-color);
}

.select-input {
  flex: 1;
  padding: var(--spacing-xxs) var(--spacing-xs);
  border-radius: var(--border-radius);
  border: 1px solid var(--border-color);
  background-color: white;
  font-family: var(--font-family);
  cursor: pointer;
}

.accordion {
  width: 100%;
}
</style>