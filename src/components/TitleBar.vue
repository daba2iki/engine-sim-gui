<script setup lang="ts">
import { NInput } from 'naive-ui'

const props = defineProps<{ search: string; title?: string }>()
const emit = defineEmits<{ (e: 'update:search', value: string): void }>()

async function windowAction(action: 'minimize' | 'maximize' | 'close') {
  if (!('__TAURI_INTERNALS__' in window)) {
    if (action === 'close') window.close()
    return
  }

  try {
    const { getCurrentWindow } = await import('@tauri-apps/api/window')
    const win = getCurrentWindow()
    if (action === 'minimize') await win.minimize()
    if (action === 'maximize') {
      if (await win.isMaximized()) await win.unmaximize()
      else await win.maximize()
    }
    if (action === 'close') await win.close()
  } catch (e) {
    console.warn('window action failed', action, e)
  }
}
</script>

<template>
  <header class="titlebar" data-tauri-drag-region>
    <div class="titlebar-left" data-tauri-drag-region>
      <span class="logo-dot"></span>
      <span>{{ props.title || 'Engine Sim GUI' }}</span>
    </div>
    <div class="titlebar-center">
      <n-input
        :value="props.search"
        placeholder="キー名 / 説明で検索"
        clearable
        class="search-input"
        @update:value="emit('update:search', $event)"
      />
    </div>
    <div class="titlebar-right" data-tauri-drag-region>
      <button class="win-btn" title="最小化" @click="windowAction('minimize')">–</button>
      <button class="win-btn" title="最大化 / 元に戻す" @click="windowAction('maximize')">□</button>
      <button class="win-btn close" title="閉じる" @click="windowAction('close')">✕</button>
    </div>
  </header>
</template>

<style scoped>
.logo-dot {
  width: 10px;
  height: 10px;
  border-radius: 50%;
  background: var(--accent);
}

.search-input {
  border-radius: 999px;
}
</style>
