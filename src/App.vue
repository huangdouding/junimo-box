<template>
  <div class="container">
    <h1>🌿 Junimo Box</h1>

    <p class="subtitle">
      Stardew Valley Mod Manager & Launcher
    </p>

    <button @click="handleSelectPath">
      选择游戏目录
    </button>

    <p v-if="gamePath" class="path">
      当前路径：{{ gamePath }}
    </p>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";

const gamePath = ref("");

function handleSelectPath() {
  // 暂时先模拟
  gamePath.value = "D:\\SteamLibrary\\steamapps\\common\\Stardew Valley";
}
</script>

<style scoped>
.container {
  height: 100vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  gap: 20px;
  font-family: sans-serif;
}

h1 {
  font-size: 42px;
}

.subtitle {
  color: #666;
}

button {
  padding: 10px 20px;
  font-size: 16px;
  cursor: pointer;
}

.path {
  font-size: 14px;
  color: #333;
}
</style>