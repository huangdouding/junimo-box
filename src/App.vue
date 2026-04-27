<template>
  <div class="container">
    <h1>🌿 Junimo Box</h1>

    <p class="subtitle">
      Stardew Valley Mod Manager & Launcher
    </p>

    <button @click="handleSelectPath">
      选择游戏目录
    </button>

    <div v-if="gamePath" class="card">
      <p>
        <strong>当前路径：</strong>
        {{ gamePath }}
      </p>

      <p>
        <strong>Stardew Valley：</strong>
        <span :class="stardewExists ? 'ok' : 'bad'">
          {{ stardewExists ? "已找到" : "未找到" }}
        </span>
      </p>

      <p>
        <strong>SMAPI：</strong>
        <span :class="smapiExists ? 'ok' : 'bad'">
          {{ smapiExists ? "已安装" : "未安装" }}
        </span>
      </p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { open } from "@tauri-apps/plugin-dialog";
import { exists } from "@tauri-apps/plugin-fs";

const gamePath = ref("");
const stardewExists = ref(false);
const smapiExists = ref(false);

async function handleSelectPath() {
  const selected = await open({
    directory: true,
    multiple: false,
    title: "选择 Stardew Valley 安装目录",
  });

  if (typeof selected !== "string") {
    return;
  }

  gamePath.value = selected;

  const stardewExe = `${selected}\\Stardew Valley.exe`;
  const smapiExe = `${selected}\\StardewModdingAPI.exe`;

  stardewExists.value = await exists(stardewExe);
  smapiExists.value = await exists(smapiExe);
}
</script>

<style scoped>
.container {
  min-height: 100vh;
  background: #f5efe3;
  color: #2d241b;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  gap: 22px;
  font-family: system-ui, sans-serif;
}

h1 {
  font-size: 44px;
  margin: 0;
}

.subtitle {
  color: #7a6652;
  margin: 0;
}

button {
  padding: 12px 22px;
  border: none;
  border-radius: 12px;
  background: #6fa85f;
  color: white;
  font-size: 16px;
  cursor: pointer;
}

button:hover {
  background: #5d944f;
}

.card {
  width: min(620px, 90vw);
  padding: 20px;
  border-radius: 18px;
  background: #fffaf0;
  box-shadow: 0 10px 30px rgba(67, 47, 27, 0.12);
}

.ok {
  color: #2f8f46;
  font-weight: 700;
}

.bad {
  color: #c0392b;
  font-weight: 700;
}
</style>