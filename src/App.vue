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

        <button
          v-if="gamePath"
          class="secondary"
          @click="scanMods"
        >
          扫描 Mods
        </button>
        <button
        v-if="modsFolderExists"
        class="secondary"
        @click="handleOpenModsFolder"
        >
          打开 Mods 文件夹
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

        <p>
          <strong>Mods 文件夹：</strong>
          <span :class="modsFolderExists ? 'ok' : 'bad'">
            {{ modsFolderExists ? "已找到" : "未找到" }}
          </span>
        </p>

        <p v-if="message" class="message">
          {{ message }}
        </p>
      </div>

      <div v-if="mods.length > 0" class="mods-panel">
        <div class="mods-header">
          <h2>已安装 Mods</h2>
          <span>{{ mods.length }} 个</span>
        </div>

        <div class="mods-list">
          <article
            v-for="mod in mods"
            :key="mod.uniqueId || mod.folderName"
            class="mod-item"
          >
            <div>
              <h3>{{ mod.name }}</h3>
              <p class="mod-meta">
                {{ mod.author || "未知作者" }} · v{{ mod.version || "未知版本" }}
              </p>
              <p class="mod-description">
                {{ mod.description || "没有描述。" }}
              </p>
              <div
  v-if="mod.contentPackFor || mod.dependencies.length > 0"
  class="dependencies"
>
  <p v-if="mod.contentPackFor" class="dependency-line">
    内容包依赖：
    <span :class="mod.contentPackFor.isInstalled ? 'ok' : 'bad'">
      {{ mod.contentPackFor.uniqueId }}
      {{ mod.contentPackFor.isInstalled ? "已安装" : "缺失" }}
    </span>
  </p>

  <p
    v-for="dependency in mod.dependencies"
    :key="dependency.uniqueId"
    class="dependency-line"
  >
    依赖：
    <span :class="dependency.isInstalled ? 'ok' : dependency.isRequired ? 'bad' : 'optional'">
      {{ dependency.uniqueId }}
      {{ dependency.isInstalled ? "已安装" : dependency.isRequired ? "缺失" : "可选未安装" }}
    </span>
  </p>
</div>
            </div>

            <div class="mod-actions">
              <span class="mod-folder">
                {{ mod.folderName }}
              </span>
              
              <button
              class="tiny-button"
              @click="handleOpenModFolder(mod.folderName)"
              >
    打开文件夹
  </button>
</div>
          </article>
        </div>
      </div>
      <div v-if="skippedFolders.length > 0" class="mods-panel">
  <div class="mods-header">
    <h2>未识别文件夹</h2>
    <span>{{ skippedFolders.length }} 个</span>
  </div>

  <div class="mods-list">
    <article
      v-for="folder in skippedFolders"
      :key="folder"
      class="mod-item"
    >
      <div>
        <h3>{{ folder }}</h3>
        <p class="mod-description">
          这个文件夹没有被识别为 Mod。通常是因为没有 manifest.json，或者 manifest.json 读取失败。
        </p>
      </div>
    </article>
  </div>
</div>
<div v-if="mods.length > 0" class="card">
  <div class="mods-header">
    <h2>依赖检查</h2>
    <span>
      {{ missingDependencies.length === 0 ? "正常" : `${missingDependencies.length} 项缺失` }}
    </span>
  </div>

  <p v-if="missingDependencies.length === 0" class="success-text">
    ✅ 所有必需依赖都已安装。
  </p>

  <div v-else class="missing-list">
    <article
      v-for="dependency in missingDependencies"
      :key="dependency.uniqueId"
      class="missing-item"
    >
      <strong>{{ dependency.uniqueId }}</strong>
      <p>
        被 {{ dependency.requiredBy.length }} 个 Mod 需要：
        {{ dependency.requiredBy.join("、") }}
      </p>
    </article>
  </div>
</div>
    </section>
  </main>
</template>

<script setup lang="ts">
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { exists, readDir, readTextFile } from "@tauri-apps/plugin-fs";
import JSON5 from "json5";

const STORAGE_KEY = "junimo-box-game-path";

type ModDependency = {
  uniqueId: string;
  isRequired: boolean;
  isInstalled: boolean;
};

type MissingDependency = {
  uniqueId: string;
  requiredBy: string[];
};

type ModInfo = {
  name: string;
  author: string;
  version: string;
  description: string;
  uniqueId: string;
  folderName: string;
  dependencies: ModDependency[];
  contentPackFor?: ModDependency;
};

const gamePath = ref("");
const stardewExists = ref(false);
const smapiExists = ref(false);
const modsFolderExists = ref(false);
const message = ref("");
const mods = ref<ModInfo[]>([]);
const skippedFolders = ref<string[]>([]);
const missingDependencies = ref<MissingDependency[]>([]);

onMounted(async () => {
  const savedPath = localStorage.getItem(STORAGE_KEY);

  if (!savedPath) {
    return;
  }

  gamePath.value = savedPath;
  await checkGameFiles(savedPath);
  await scanMods();
});

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
  mods.value = [];

  localStorage.setItem(STORAGE_KEY, selected);

  await checkGameFiles(selected);
  await scanMods();
}

async function checkGameFiles(selectedPath: string) {
  const stardewExe = `${selectedPath}\\Stardew Valley.exe`;
  const smapiExe = `${selectedPath}\\StardewModdingAPI.exe`;
  const modsFolder = `${selectedPath}\\Mods`;

  stardewExists.value = await exists(stardewExe);
  smapiExists.value = await exists(smapiExe);
  modsFolderExists.value = await exists(modsFolder);
}

async function scanMods() {
  if (!gamePath.value) {
    message.value = "请先选择游戏目录。";
    return;
  }

  const modsFolder = `${gamePath.value}\\Mods`;

  modsFolderExists.value = await exists(modsFolder);

  if (!modsFolderExists.value) {
    mods.value = [];
    message.value = "未找到 Mods 文件夹。";
    return;
  }

  try {
  skippedFolders.value = [];
  const foundMods = await collectModsFromFolder(modsFolder, "");

    mods.value = attachDependencyStatus(
      foundMods.sort((a, b) => a.name.localeCompare(b.name))
    );
    missingDependencies.value = collectMissingDependencies(mods.value);

    message.value =
      foundMods.length > 0
        ? `扫描完成：找到 ${foundMods.length} 个 Mod。`
        : "扫描完成：没有找到带 manifest.json 的 Mod。";
  } catch (error) {
    message.value = `扫描 Mods 失败：${String(error)}`;
  }
}

async function collectModsFromFolder(
  folderPath: string,
  relativePath: string
): Promise<ModInfo[]> {
  const entries = await readDir(folderPath);
  const foundMods: ModInfo[] = [];

  const folderLabel = relativePath || getFolderName(folderPath);
  const manifestPath = `${folderPath}\\manifest.json`;
  const hasManifest = await exists(manifestPath);

  if (hasManifest) {
    try {
      const manifestText = await readTextFile(manifestPath);
      const manifest = JSON5.parse(manifestText);

      foundMods.push({
        name: manifest.Name || getFolderName(folderPath),
        author: manifest.Author || "",
        version: manifest.Version || "",
        description: manifest.Description || "",
        uniqueId: manifest.UniqueID || "",
        folderName: folderLabel,
        dependencies: normalizeDependencies(manifest.Dependencies),
        contentPackFor: normalizeContentPackFor(manifest.ContentPackFor),
});
      // 关键：已经识别为一个 Mod，就不要继续扫它内部的 assets/i18n 等文件夹
      return foundMods;
    } catch (error) {
      console.warn(`读取 manifest 失败：${manifestPath}`, error);
      skippedFolders.value.push(
         `${folderLabel}：manifest.json 读取或解析失败 - ${String(error)}`
  );
  return foundMods;
}
  }

  for (const entry of entries) {
    if (!entry.isDirectory) {
      continue;
    }

    const childPath = `${folderPath}\\${entry.name}`;
    const childRelativePath = relativePath
      ? `${relativePath}\\${entry.name}`
      : entry.name;

    const childMods = await collectModsFromFolder(
      childPath,
      childRelativePath
    );

    foundMods.push(...childMods);
  }

  // 只记录比较浅层的未识别文件夹，避免把 Mod 内部普通资源文件夹全部列出来
  const depth = relativePath ? relativePath.split("\\").length : 0;

  if (!hasManifest && foundMods.length === 0 && depth <= 2 && relativePath) {
    skippedFolders.value.push(`${folderLabel}：没有找到 manifest.json`);
  }

  return foundMods;
}


function getFolderName(path: string) {
  const parts = path.split("\\").filter(Boolean);
  return parts[parts.length - 1] || path;
}

async function handleOpenModsFolder() {
  if (!gamePath.value) {
    message.value = "请先选择游戏目录。";
    return;
  }

  const modsFolder = `${gamePath.value}\\Mods`;

  if (!(await exists(modsFolder))) {
    message.value = "未找到 Mods 文件夹。";
    return;
  }

  try {
    await invoke("open_folder", {
      path: modsFolder,
    });

    message.value = "已打开 Mods 文件夹。";
  } catch (error) {
    message.value = `打开 Mods 文件夹失败：${String(error)}`;
  }
}


async function handleOpenModFolder(folderName: string) {
  if (!gamePath.value) {
    message.value = "请先选择游戏目录。";
    return;
  }

  const modFolder = `${gamePath.value}\\Mods\\${folderName}`;

  if (!(await exists(modFolder))) {
    message.value = `未找到 Mod 文件夹：${folderName}`;
    return;
  }

  try {
    await invoke("open_folder", {
      path: modFolder,
    });

    message.value = `已打开 Mod 文件夹：${folderName}`;
  } catch (error) {
    message.value = `打开 Mod 文件夹失败：${String(error)}`;
  }
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

function normalizeDependencies(rawDependencies: unknown): ModDependency[] {
  if (!Array.isArray(rawDependencies)) {
    return [];
  }

  return rawDependencies
    .map((dependency) => {
      if (!dependency || typeof dependency !== "object") {
        return null;
      }

      const item = dependency as {
        UniqueID?: string;
        IsRequired?: boolean;
      };

      if (!item.UniqueID) {
        return null;
      }

      return {
        uniqueId: item.UniqueID,
        isRequired: item.IsRequired !== false,
        isInstalled: false,
      };
    })
    .filter((dependency): dependency is ModDependency => dependency !== null);
}

function normalizeContentPackFor(rawContentPackFor: unknown): ModDependency | undefined {
  if (!rawContentPackFor || typeof rawContentPackFor !== "object") {
    return undefined;
  }

  const item = rawContentPackFor as {
    UniqueID?: string;
  };

  if (!item.UniqueID) {
    return undefined;
  }

  return {
    uniqueId: item.UniqueID,
    isRequired: true,
    isInstalled: false,
  };
}

function attachDependencyStatus(modList: ModInfo[]) {
  const installedUniqueIds = new Set(
    modList
      .map((mod) => mod.uniqueId)
      .filter(Boolean)
  );

  return modList.map((mod) => ({
    ...mod,
    dependencies: mod.dependencies.map((dependency) => ({
      ...dependency,
      isInstalled: installedUniqueIds.has(dependency.uniqueId),
    })),
    contentPackFor: mod.contentPackFor
      ? {
          ...mod.contentPackFor,
          isInstalled: installedUniqueIds.has(mod.contentPackFor.uniqueId),
        }
      : undefined,
  }));
}

function collectMissingDependencies(modList: ModInfo[]): MissingDependency[] {
  const missingMap = new Map<string, string[]>();

  for (const mod of modList) {
    const requiredDependencies = [
      ...(mod.contentPackFor ? [mod.contentPackFor] : []),
      ...mod.dependencies.filter((dependency) => dependency.isRequired),
    ];

    for (const dependency of requiredDependencies) {
      if (dependency.isInstalled) {
        continue;
      }

      const requiredBy = missingMap.get(dependency.uniqueId) || [];
      requiredBy.push(mod.name);
      missingMap.set(dependency.uniqueId, requiredBy);
    }
  }

  return Array.from(missingMap.entries())
    .map(([uniqueId, requiredBy]) => ({
      uniqueId,
      requiredBy,
    }))
    .sort((a, b) => a.uniqueId.localeCompare(b.uniqueId));
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
  padding: 48px 32px;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 18px;
  overflow-y: auto;
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
  flex-wrap: wrap;
  justify-content: center;
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

.card,
.mods-panel {
  width: min(680px, 90vw);
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

.mods-panel {
  margin-bottom: 32px;
}

.mods-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
}

.mods-header h2 {
  margin: 0;
  font-size: 22px;
}

.mods-header span {
  color: #7a6652;
  font-weight: 700;
}

.mods-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.mod-item {
  display: flex;
  justify-content: space-between;
  gap: 16px;
  padding: 14px;
  border-radius: 14px;
  background: #f6ead8;
}

.mod-item h3 {
  margin: 0 0 6px;
  font-size: 18px;
}

.mod-meta {
  margin: 0 0 6px;
  color: #7a6652;
  font-size: 14px;
}

.mod-description {
  margin: 0;
  color: #4b3a2a;
  font-size: 14px;
}

.mod-folder {
  flex-shrink: 0;
  align-self: flex-start;
  padding: 5px 8px;
  border-radius: 999px;
  background: #e2d1b8;
  color: #5c4630;
  font-size: 12px;
  font-weight: 700;
}
.dependencies {
  margin-top: 10px;
  padding-top: 8px;
  border-top: 1px solid rgba(92, 70, 48, 0.16);
}

.dependency-line {
  margin: 4px 0;
  font-size: 13px;
  color: #5c4630;
}

.optional {
  color: #9a6a2f;
  font-weight: 700;
}
.mod-actions {
  flex-shrink: 0;
  display: flex;
  flex-direction: column;
  align-items: flex-end;
  gap: 8px;
}

.tiny-button {
  padding: 6px 10px;
  border-radius: 999px;
  font-size: 12px;
  background: #8b6f47;
}

.tiny-button:hover {
  background: #755d3c;
}
</style>