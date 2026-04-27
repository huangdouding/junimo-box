<template>
  <main class="app">
    <section class="hero">
      <h1>🌿 Junimo Box</h1>

      <p class="subtitle">
        Stardew Valley Mod Manager & Launcher
      </p>

      <div class="actions">
        <button @click="handleSelectPath">
          选择游戏目录
        </button>

        <button
          v-if="gamePath"
          class="secondary"
          @click="handleLaunchGame"
        >
          启动游戏
        </button>
      </div>

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

        <p v-if="message" class="message">
          {{ message }}
        </p>
      </div>
    </section>
  </main>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { exists } from "@tauri-apps/plugin-fs";

const gamePath = ref("");
const stardewExists = ref(false);
const smapiExists = ref(false);
const message = ref("");

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
  message.value = "";

  await checkGameFiles(selected);
}

async function checkGameFiles(selectedPath: string) {
  const stardewExe = `${selectedPath}\\Stardew Valley.exe`;
  const smapiExe = `${selectedPath}\\StardewModdingAPI.exe`;

  stardewExists.value = await exists(stardewExe);
  smapiExists.value = await exists(smapiExe);
}

async function handleLaunchGame() {
  if (!gamePath.value) {
    message.value = "请先选择游戏目录。";
    return;
  }

  const smapiExe = `${gamePath.value}\\StardewModdingAPI.exe`;
  const stardewExe = `${gamePath.value}\\Stardew Valley.exe`;

  const targetExe = smapiExists.value ? smapiExe : stardewExe;

  if (!smapiExists.value && !stardewExists.value) {
    message.value = "未找到 Stardew Valley.exe，无法启动游戏。";
    return;
  }

  try {
    await invoke("launch_game", {
      path: targetExe,
    });

    message.value = smapiExists.value
      ? "正在通过 SMAPI 启动游戏..."
      : "正在启动原版 Stardew Valley...";
  } catch (error) {
    message.value = `启动失败：${String(error)}`;
  }
}
</script>

<style scoped>
.app {
  width: 100vw;
  height: 100vh;
  overflow: hidden;
  background: #f5efe3;
  color: #2d241b;
  font-family: system-ui, sans-serif;
}

.hero {
  height: 100%;
  box-sizing: border-box;
  padding: 64px 32px;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  gap: 22px;
}

h1 {
  font-size: 44px;
  line-height: 1;
  margin: 0;
}

.subtitle {
  color: #7a6652;
  margin: 0;
}

.actions {
  display: flex;
  gap: 12px;
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

button.secondary {
  background: #8b6f47;
}

button.secondary:hover {
  background: #755d3c;
}

.card {
  width: min(620px, 90vw);
  box-sizing: border-box;
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

.message {
  margin-top: 16px;
  color: #7a4f22;
  font-weight: 700;
}
</style>