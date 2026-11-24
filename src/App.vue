<script setup lang="ts">
import { onMounted } from 'vue'
import MainView from './views/MainView.vue'

// メイン画面コンポーネントのロード
const loadMainView = async () => {
  // Tauri側でファイルを読み込むコマンドを呼び出す例
  console.log('Loading main view...')
}

onMounted(() => {
  loadMainView()
})
</script>

<template>
  <div class="app-container">
    <MainView />
  </div>
</template>

<style scoped>
.app-container {
  width: 100vw;
  height: 100vh;
  display: flex;
  justify-content: center;
  align-items: center;
}
</style>