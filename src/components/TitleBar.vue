<script setup lang="ts">
import { NInput } from 'naive-ui'
import { onMounted, ref } from 'vue'

const props = defineProps<{ search: string; title?: string }>()
const emit = defineEmits<{ (e: 'update:search', value: string): void }>()

const isMaximized = ref(false)

async function refreshMaxState() {
  if (!('__TAURI_INTERNALS__' in window)) return
  try {
    const { getCurrentWindow } = await import('@tauri-apps/api/window')
    isMaximized.value = await getCurrentWindow().isMaximized()
  } catch {
    /* ignore */
  }
}

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

onMounted(refreshMaxState)
</script>

<template>
  <header class="titlebar" data-tauri-drag-region>
    <div class="titlebar-left" data-tauri-drag-region>
      <span class="logo-dot"></span>
      <span class="titlebar-name">{{ props.title || 'Engine Sim GUI' }}</span>
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
      <button class="win-btn" type="button" title="最小化" @click="windowAction('minimize')">
        <svg width="12" height="12" viewBox="0 0 12 12" fill="none" aria-hidden="true">
          <line x1="1" y1="6" x2="11" y2="6" stroke="currentColor" stroke-width="1" />
        </svg>
      </button>
      <button
        class="win-btn"
        type="button"
        :title="isMaximized ? '元に戻す' : '最大化'"
        @click="windowAction('maximize')"
      >
        <svg v-if="!isMaximized" width="12" height="12" viewBox="0 0 12 12" fill="none" aria-hidden="true">
          <rect x="1.5" y="1.5" width="9" height="9" stroke="currentColor" stroke-width="1" />
        </svg>
        <svg v-else width="12" height="12" viewBox="0 0 12 12" fill="none" aria-hidden="true">
          <rect x="3.5" y="1.5" width="7" height="7" stroke="currentColor" stroke-width="1" />
          <path d="M1.5 4.5v6h6" stroke="currentColor" stroke-width="1" />
        </svg>
      </button>
      <button class="win-btn close" type="button" title="閉じる" @click="windowAction('close')">
        <svg width="12" height="12" viewBox="0 0 12 12" fill="none" aria-hidden="true">
          <line x1="1.5" y1="1.5" x2="10.5" y2="10.5" stroke="currentColor" stroke-width="1" />
          <line x1="10.5" y1="1.5" x2="1.5" y2="10.5" stroke="currentColor" stroke-width="1" />
        </svg>
      </button>
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

.titlebar-name {
  font-weight: 600;
}

.search-input :deep(.n-input-wrapper) {
  border-radius: 999px;
}

.search-input :deep(.n-input__input-element) {
  color: var(--text-primary);
}

.win-btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 34px;
  height: 30px;
  border: 0;
  background: transparent;
  color: var(--text-primary);
  border-radius: var(--radius);
  cursor: pointer;
  padding: 0;
  transition: background-color 150ms ease, color 150ms ease;
}

.win-btn:hover {
  background: var(--hover-bg);
  color: var(--text-primary);
}

.win-btn.close:hover {
  background: var(--close-hover);
  color: var(--white);
}
</style>
