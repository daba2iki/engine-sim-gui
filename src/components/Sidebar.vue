<script setup lang="ts">
defineProps<{
  files: string[]
  currentFile: string | null
  categories: string[]
  selectedCategory: string
}>()

const emit = defineEmits<{
  (e: 'select-file', file: string): void
  (e: 'select-category', category: string): void
}>()
</script>

<template>
  <aside class="sidebar">
    <section class="sidebar-section">
      <div class="sidebar-title">ファイル</div>
      <div
        v-for="file in files"
        :key="file"
        class="sidebar-item"
        :class="{ active: currentFile === file }"
        @click="emit('select-file', file)"
      >
        {{ file.split('/').pop() }}
      </div>
      <div v-if="files.length === 0" class="sidebar-item">ファイルなし</div>
    </section>

    <section class="sidebar-section">
      <div class="sidebar-title">カテゴリ</div>
      <div
        v-for="category in categories"
        :key="category"
        class="sidebar-item"
        :class="{ active: selectedCategory === category }"
        @click="emit('select-category', category)"
      >
        {{ category }}
      </div>
    </section>
  </aside>
</template>
