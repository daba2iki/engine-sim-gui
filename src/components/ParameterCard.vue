<script setup lang="ts">
import { computed } from 'vue'
import { NSwitch, NSlider, NInput, NInputNumber, NSelect } from 'naive-ui'
import type { Param } from '../types'

const props = defineProps<{ param: Param; changed: boolean }>()
const emit = defineEmits<{ (e: 'update:value', value: Param['value']): void }>()

const sliderMax = computed(() => {
  const base = Number(props.param.default)
  if (!Number.isFinite(base) || base === 0) return 10
  const abs = Math.abs(base)
  return Math.max(10, Math.ceil(abs * 3))
})

function setValue(value: Param['value']) {
  emit('update:value', value)
}
</script>

<template>
  <article class="card" :class="{ modified: changed }">
    <div class="card-header">
      <div class="card-meta">
        <div class="card-title">{{ param.label }}</div>
        <div class="card-key">{{ param.key }}</div>
        <div v-if="param.description" class="card-description">{{ param.description }}</div>
      </div>
    </div>

    <div class="card-body">
      <div class="editor">
        <NSwitch
          v-if="param.type === 'bool'"
          :value="param.value === true"
          @update:value="(v: boolean) => setValue(v)"
        />

        <div v-else-if="param.type === 'number'" class="number-editor">
          <NSlider
            :value="Number(param.value)"
            :min="0"
            :max="sliderMax"
            :step="Math.max(0.01, Number(props.param.default) / 100 || 1)"
            class="slider"
            @update:value="(v: number) => setValue(v)"
          />
          <NInputNumber
            :value="Number(param.value)"
            :step="Math.max(0.01, Number(props.param.default) / 100 || 1)"
            class="value-input"
            @update:value="(v: number | null) => setValue(v ?? 0)"
          />
          <span v-if="param.unit" class="unit">{{ param.unit }}</span>
        </div>

        <NInput
          v-else-if="param.type === 'string'"
          :value="String(param.value)"
          class="text-input"
          @update:value="(v: string) => setValue(v)"
        />

        <NSelect
          v-else-if="param.type === 'select'"
          :value="String(param.value)"
          :options="(param.options ?? []).map((option) => ({ label: option, value: option }))"
          class="select"
          @update:value="(v: string | null) => setValue(v ?? (param.options?.[0] ?? ''))"
        />
      </div>
    </div>
  </article>
</template>

<style scoped>
.slider {
  flex: 1;
}

.value-input {
  width: 120px;
}

.text-input {
  border-radius: var(--radius);
}
</style>
