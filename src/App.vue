<template>
  <main class="app-shell">
    <aside class="sidebar">
      <div class="brand">
        <div class="brand-icon">🌿</div>
        <div>
          <h1>Junimo Box</h1>
          <p>祝尼魔盒</p>
        </div>
      </div>

      <nav class="nav">
        <button
          v-for="item in navItems"
          :key="item.id"
          class="nav-button"
          :class="{ active: activeView === item.id }"
          @click="activeView = item.id"
        >
          <span>{{ item.icon }}</span>
          {{ item.label }}
        </button>
      </nav>

      <div class="sidebar-footer">
        <p>Stardew Valley</p>
        <strong>Mod Manager & Launcher</strong>
      </div>
    </aside>

    <section class="content">
      <header class="content-header">
        <div>
          <p class="eyebrow">{{ currentViewMeta.eyebrow }}</p>
          <h2>{{ currentViewMeta.title }}</h2>
          <p>{{ currentViewMeta.description }}</p>
        </div>

        <button
          v-if="activeView === 'mods' && gamePath"
          class="secondary"
          @click="scanMods"
        >
          重新扫描
        </button>

        <button
          v-if="activeView === 'logs'"
          class="secondary"
          @click="handleReadLatestSmapiLog"
        >
          读取最新日志
        </button>
      </header>

      <div v-if="message" class="notice">
        {{ message }}
      </div>

      <section v-if="activeView === 'overview'" class="view-stack">
        <div class="panel">
          <div class="panel-header">
            <h3>当前环境</h3>
            <span>{{ gamePath ? "已配置" : "未配置" }}</span>
          </div>

          <div class="status-grid">
            <div class="status-card">
              <span>游戏目录</span>
              <strong :class="gamePath ? 'ok' : 'bad'">
                {{ gamePath ? "已选择" : "未选择" }}
              </strong>
            </div>

            <div class="status-card">
              <span>Stardew Valley</span>
              <strong :class="stardewExists ? 'ok' : 'bad'">
                {{ stardewExists ? "已找到" : "未找到" }}
              </strong>
            </div>

            <div class="status-card">
              <span>SMAPI</span>
              <strong :class="smapiExists ? 'ok' : 'bad'">
                {{ smapiExists ? "已安装" : "未安装" }}
              </strong>
            </div>

            <div class="status-card">
              <span>Mods 文件夹</span>
              <strong :class="modsFolderExists ? 'ok' : 'bad'">
                {{ modsFolderExists ? "已找到" : "未找到" }}
              </strong>
            </div>
          </div>
        </div>

        <div class="panel">
          <div class="panel-header">
            <h3>Mod 概览</h3>
            <span>{{ mods.length + disabledMods.length }} 个</span>
          </div>

          <div class="summary-row">
            <div>
              <span>已启用</span>
              <strong>{{ mods.length }}</strong>
            </div>

            <div>
              <span>已禁用</span>
              <strong>{{ disabledMods.length }}</strong>
            </div>

            <div>
              <span>缺失依赖</span>
              <strong :class="missingDependencies.length > 0 ? 'bad' : 'ok'">
                {{ missingDependencies.length }}
              </strong>
            </div>

            <div>
              <span>未识别</span>
              <strong>{{ skippedFolders.length }}</strong>
            </div>
          </div>
        </div>
      </section>

      <section v-if="activeView === 'mods'" class="view-stack">
        <div v-if="mods.length > 0" class="panel">
          <div class="panel-header">
            <h3>依赖检查</h3>
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

        <div v-if="mods.length > 0" class="panel">
          <div class="panel-header">
            <h3>已安装 Mods</h3>
            <span>{{ mods.length }} 个</span>
          </div>

          <div class="mods-list">
            <article
              v-for="mod in mods"
              :key="mod.uniqueId || mod.folderName"
              class="mod-item"
            >
              <div class="mod-main">
                <h4>{{ mod.name }}</h4>

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
                    <span
                      :class="
                        dependency.isInstalled
                          ? 'ok'
                          : dependency.isRequired
                            ? 'bad'
                            : 'optional'
                      "
                    >
                      {{ dependency.uniqueId }}
                      {{
                        dependency.isInstalled
                          ? "已安装"
                          : dependency.isRequired
                            ? "缺失"
                            : "可选未安装"
                      }}
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
                  打开
                </button>

                <button
                  class="tiny-button danger"
                  @click="handleDisableMod(mod.folderName)"
                >
                  禁用
                </button>
              </div>
            </article>
          </div>
        </div>

        <div v-if="disabledMods.length > 0" class="panel">
          <div class="panel-header">
            <h3>已禁用 Mods</h3>
            <span>{{ disabledMods.length }} 个</span>
          </div>

          <div class="mods-list">
            <article
              v-for="mod in disabledMods"
              :key="mod.uniqueId || mod.folderName"
              class="mod-item disabled"
            >
              <div class="mod-main">
                <h4>{{ mod.name }}</h4>

                <p class="mod-meta">
                  {{ mod.author || "未知作者" }} · v{{ mod.version || "未知版本" }}
                </p>

                <p class="mod-description">
                  {{ mod.description || "没有描述。" }}
                </p>
              </div>

              <div class="mod-actions">
                <span class="mod-folder">
                  {{ mod.folderName }}
                </span>

                <button
                  class="tiny-button"
                  @click="handleEnableMod(mod.folderName)"
                >
                  启用
                </button>
              </div>
            </article>
          </div>
        </div>

        <div v-if="skippedFolders.length > 0" class="panel">
          <div class="panel-header">
            <h3>未识别文件夹</h3>
            <span>{{ skippedFolders.length }} 个</span>
          </div>

          <div class="mods-list">
            <article
              v-for="folder in skippedFolders"
              :key="folder"
              class="mod-item warning"
            >
              <div class="mod-main">
                <h4>{{ folder }}</h4>
                <p class="mod-description">
                  这个文件夹没有被识别为 Mod。通常是因为没有 manifest.json，或者 manifest.json 读取失败。
                </p>
              </div>
            </article>
          </div>
        </div>

        <div
          v-if="
            gamePath &&
            mods.length === 0 &&
            disabledMods.length === 0 &&
            skippedFolders.length === 0
          "
          class="empty-state"
        >
          <h3>还没有扫描到 Mod</h3>
          <p>点击右侧的“扫描 Mods”开始读取 Mods 文件夹。</p>
        </div>
      </section>

      <section v-if="activeView === 'logs'" class="view-stack">
        <div v-if="smapiLogContent" class="panel">
          <div class="panel-header">
            <h3>SMAPI 日志</h3>
            <span>{{ smapiLogFileName }}</span>
          </div>

          <pre class="log-viewer">{{ smapiLogContent }}</pre>
        </div>

        <div v-else class="empty-state">
          <h3>还没有读取日志</h3>
          <p>点击“读取最新日志”，Junimo Box 会打开最近一次 SMAPI 日志。</p>
        </div>
      </section>

      <section v-if="activeView === 'tools'" class="view-stack">
        <div class="panel">
          <div class="panel-header">
            <h3>工具箱</h3>
            <span>快捷操作</span>
          </div>

          <div class="tool-grid">
            <button @click="handleOpenGameFolder">
              打开游戏目录
            </button>

            <button
              :disabled="!modsFolderExists"
              @click="handleOpenModsFolder"
            >
              打开 Mods 文件夹
            </button>

            <button @click="handleOpenSmapiLogFolder">
              打开日志文件夹
            </button>

            <button
              :disabled="mods.length === 0 && disabledMods.length === 0"
              @click="handleExportModList"
            >
              导出 Mod 列表
            </button>
          </div>
        </div>
      </section>

      <section v-if="activeView === 'settings'" class="view-stack">
        <div class="panel">
          <div class="panel-header">
            <h3>基础设置</h3>
            <span>本地配置</span>
          </div>

          <div class="setting-block">
            <span>当前游戏路径</span>
            <strong>{{ gamePath || "未选择" }}</strong>
          </div>

          <div class="setting-actions">
            <button @click="handleSelectPath">
              重新选择游戏目录
            </button>
          </div>
        </div>
      </section>
    </section>

    <aside class="right-panel">
      <div class="launch-card">
        <div class="junimo-badge">🌱</div>
        <h3>启动中心</h3>
        <p>管理你的星露谷 Mod 环境</p>

        <button class="launch-button" @click="handleLaunchGame">
          启动游戏
        </button>
      </div>

      <div class="side-card">
        <h4>游戏状态</h4>

        <div class="info-line">
          <span>Stardew Valley</span>
          <strong :class="stardewExists ? 'ok' : 'bad'">
            {{ stardewExists ? "已找到" : "未找到" }}
          </strong>
        </div>

        <div class="info-line">
          <span>SMAPI</span>
          <strong :class="smapiExists ? 'ok' : 'bad'">
            {{ smapiExists ? "已安装" : "未安装" }}
          </strong>
        </div>

        <div class="info-line">
          <span>Mods 文件夹</span>
          <strong :class="modsFolderExists ? 'ok' : 'bad'">
            {{ modsFolderExists ? "已找到" : "未找到" }}
          </strong>
        </div>
      </div>

      <div class="side-card">
        <h4>快捷操作</h4>

        <div class="side-actions">
          <button @click="handleSelectPath">
            选择目录
          </button>

          <button
            :disabled="!gamePath"
            @click="scanMods"
          >
            扫描 Mods
          </button>

          <button
            :disabled="!modsFolderExists"
            @click="handleOpenModsFolder"
          >
            Mods 文件夹
          </button>

          <button
            :disabled="mods.length === 0 && disabledMods.length === 0"
            @click="handleExportModList"
          >
            导出列表
          </button>
        </div>
      </div>

      <div class="side-card path-card">
        <h4>当前路径</h4>
        <p>{{ gamePath || "尚未选择 Stardew Valley 安装目录" }}</p>
      </div>
    </aside>
  </main>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { confirm, open, save } from "@tauri-apps/plugin-dialog";
import { exists, readDir, readTextFile } from "@tauri-apps/plugin-fs";
import JSON5 from "json5";

const STORAGE_KEY = "junimo-box-game-path";

type ViewId = "overview" | "mods" | "logs" | "tools" | "settings";

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

const navItems: Array<{
  id: ViewId;
  label: string;
  icon: string;
}> = [
  { id: "overview", label: "总览", icon: "🏡" },
  { id: "mods", label: "Mods", icon: "📦" },
  { id: "logs", label: "日志", icon: "📜" },
  { id: "tools", label: "工具箱", icon: "🧰" },
  { id: "settings", label: "设置", icon: "⚙️" },
];

const activeView = ref<ViewId>("mods");

const gamePath = ref("");
const stardewExists = ref(false);
const smapiExists = ref(false);
const modsFolderExists = ref(false);
const message = ref("");

const mods = ref<ModInfo[]>([]);
const disabledMods = ref<ModInfo[]>([]);
const skippedFolders = ref<string[]>([]);
const missingDependencies = ref<MissingDependency[]>([]);

const smapiLogFileName = ref("");
const smapiLogContent = ref("");

const currentViewMeta = computed(() => {
  const map: Record<
    ViewId,
    {
      eyebrow: string;
      title: string;
      description: string;
    }
  > = {
    overview: {
      eyebrow: "Overview",
      title: "总览",
      description: "查看当前游戏环境、Mod 数量和依赖状态。",
    },
    mods: {
      eyebrow: "Local Mods",
      title: "本地 Mod 管理",
      description: "扫描、查看、启用或禁用 Stardew Valley Mods。",
    },
    logs: {
      eyebrow: "SMAPI Logs",
      title: "SMAPI 日志",
      description: "读取最近一次 SMAPI 日志，后续会升级为诊断摘要。",
    },
    tools: {
      eyebrow: "Toolbox",
      title: "工具箱",
      description: "打开常用目录，导出 Mod 列表和问题排查资料。",
    },
    settings: {
      eyebrow: "Settings",
      title: "设置",
      description: "管理本地路径和 Junimo Box 基础偏好。",
    },
  };

  return map[activeView.value];
});

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
  disabledMods.value = [];
  skippedFolders.value = [];
  missingDependencies.value = [];

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
  const disabledModsFolder = `${gamePath.value}\\Disabled Mods`;

  modsFolderExists.value = await exists(modsFolder);

  if (!modsFolderExists.value) {
    mods.value = [];
    disabledMods.value = [];
    missingDependencies.value = [];
    skippedFolders.value = [];
    message.value = "未找到 Mods 文件夹。";
    return;
  }

  try {
    skippedFolders.value = [];

    const foundMods = await collectModsFromFolder(modsFolder, "");
    mods.value = attachDependencyStatus(
      foundMods.sort((a, b) => a.name.localeCompare(b.name))
    );

    if (await exists(disabledModsFolder)) {
      const foundDisabledMods = await collectModsFromFolder(
        disabledModsFolder,
        ""
      );

      disabledMods.value = attachDependencyStatus(
        foundDisabledMods.sort((a, b) => a.name.localeCompare(b.name))
      );
    } else {
      disabledMods.value = [];
    }

    missingDependencies.value = collectMissingDependencies(mods.value);

    message.value =
      foundMods.length > 0
        ? `扫描完成：找到 ${foundMods.length} 个已启用 Mod。`
        : "扫描完成：没有找到已启用 Mod。";
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

  const depth = relativePath ? relativePath.split("\\").length : 0;

  if (!hasManifest && foundMods.length === 0 && depth <= 2 && relativePath) {
    skippedFolders.value.push(`${folderLabel}：没有找到 manifest.json`);
  }

  return foundMods;
}

async function handleLaunchGame() {
  if (!gamePath.value) {
    message.value = "请先选择游戏目录。";
    return;
  }

  await checkGameFiles(gamePath.value);

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

async function handleOpenGameFolder() {
  if (!gamePath.value) {
    message.value = "请先选择游戏目录。";
    return;
  }

  try {
    await invoke("open_folder", {
      path: gamePath.value,
    });

    message.value = "已打开游戏目录。";
  } catch (error) {
    message.value = `打开游戏目录失败：${String(error)}`;
  }
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

async function handleDisableMod(folderName: string) {
  if (!gamePath.value) {
    message.value = "请先选择游戏目录。";
    return;
  }

  const confirmed = await confirm(
    `确定要禁用这个 Mod 吗？\n\n${folderName}\n\n它会被移动到 Disabled Mods 文件夹。`,
    {
      title: "确认禁用 Mod",
      kind: "warning",
    }
  );

  if (!confirmed) {
    return;
  }

  const from = `${gamePath.value}\\Mods\\${folderName}`;
  const to = `${gamePath.value}\\Disabled Mods\\${folderName}`;

  if (!(await exists(from))) {
    message.value = `禁用失败：没有找到 Mod 文件夹：${folderName}`;
    return;
  }

  if (await exists(to)) {
    message.value = `禁用失败：Disabled Mods 中已经存在同名文件夹：${folderName}`;
    return;
  }

  try {
    await invoke("move_folder", {
      from,
      to,
    });

    message.value = `已禁用 Mod：${folderName}`;
    await scanMods();
  } catch (error) {
    message.value = `禁用 Mod 失败：${String(error)}`;
  }
}

async function handleEnableMod(folderName: string) {
  if (!gamePath.value) {
    message.value = "请先选择游戏目录。";
    return;
  }

  const confirmed = await confirm(
    `确定要启用这个 Mod 吗？\n\n${folderName}\n\n它会被移动回 Mods 文件夹。`,
    {
      title: "确认启用 Mod",
      kind: "info",
    }
  );

  if (!confirmed) {
    return;
  }

  const from = `${gamePath.value}\\Disabled Mods\\${folderName}`;
  const to = `${gamePath.value}\\Mods\\${folderName}`;

  if (!(await exists(from))) {
    message.value = `启用失败：没有找到已禁用的 Mod 文件夹：${folderName}`;
    return;
  }

  if (await exists(to)) {
    message.value = `启用失败：Mods 中已经存在同名文件夹：${folderName}`;
    return;
  }

  try {
    await invoke("move_folder", {
      from,
      to,
    });

    message.value = `已启用 Mod：${folderName}`;
    await scanMods();
  } catch (error) {
    message.value = `启用 Mod 失败：${String(error)}`;
  }
}

async function handleExportModList() {
  if (!gamePath.value) {
    message.value = "请先选择游戏目录。";
    return;
  }

  const filePath = await save({
    title: "导出 Mod 列表",
    defaultPath: "junimo-box-mod-report.json",
    filters: [
      {
        name: "JSON 文件",
        extensions: ["json"],
      },
    ],
  });

  if (!filePath) {
    return;
  }

  const report = {
    app: "Junimo Box",
    exportedAt: new Date().toISOString(),
    gamePath: gamePath.value,
    stardewExists: stardewExists.value,
    smapiExists: smapiExists.value,
    modsFolderExists: modsFolderExists.value,
    summary: {
      enabledMods: mods.value.length,
      disabledMods: disabledMods.value.length,
      missingDependencies: missingDependencies.value.length,
    },
    missingDependencies: missingDependencies.value,
    enabledMods: mods.value.map(createExportModInfo),
    disabledMods: disabledMods.value.map(createExportModInfo),
  };

  try {
    await invoke("write_text_file", {
      path: filePath,
      content: JSON.stringify(report, null, 2),
    });

    message.value = `已导出 Mod 列表：${filePath}`;
  } catch (error) {
    message.value = `导出失败：${String(error)}`;
  }
}

async function handleReadLatestSmapiLog() {
  try {
    const result = await invoke<string[]>("read_latest_smapi_log");

    smapiLogFileName.value = result[0] || "未知日志文件";
    smapiLogContent.value = result[1] || "";

    message.value = `已读取最新 SMAPI 日志：${smapiLogFileName.value}`;
  } catch (error) {
    smapiLogFileName.value = "";
    smapiLogContent.value = "";
    message.value = `读取 SMAPI 日志失败：${String(error)}`;
  }
}

async function handleOpenSmapiLogFolder() {
  try {
    const logFolder = await invoke<string>("get_smapi_log_folder");

    await invoke("open_folder", {
      path: logFolder,
    });

    message.value = "已打开 SMAPI 日志文件夹。";
  } catch (error) {
    message.value = `打开 SMAPI 日志文件夹失败：${String(error)}`;
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

function normalizeContentPackFor(
  rawContentPackFor: unknown
): ModDependency | undefined {
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
    modList.map((mod) => mod.uniqueId).filter(Boolean)
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

function createExportModInfo(mod: ModInfo) {
  return {
    name: mod.name,
    author: mod.author,
    version: mod.version,
    description: mod.description,
    uniqueId: mod.uniqueId,
    folderName: mod.folderName,
    contentPackFor: mod.contentPackFor,
    dependencies: mod.dependencies,
  };
}

function getFolderName(path: string) {
  const parts = path.split("\\").filter(Boolean);
  return parts[parts.length - 1] || path;
}
</script>

<style scoped>
.app-shell {
  width: 100vw;
  height: 100vh;
  overflow: hidden;
  display: grid;
  grid-template-columns: 220px minmax(0, 1fr) 300px;
  background:
    radial-gradient(circle at top left, rgba(132, 184, 95, 0.18), transparent 32%),
    #f5efe3;
  color: #2d241b;
  font-family:
    "Microsoft YaHei",
    system-ui,
    sans-serif;
}

.sidebar {
  height: 100%;
  padding: 22px 16px;
  box-sizing: border-box;
  background: linear-gradient(180deg, #5f432d, #3f2b1d);
  color: #fff7e8;
  display: flex;
  flex-direction: column;
  gap: 24px;
  border-right: 4px solid rgba(45, 36, 27, 0.2);
}

.brand {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px;
  border-radius: 18px;
  background: rgba(255, 250, 240, 0.1);
}

.brand-icon {
  width: 42px;
  height: 42px;
  display: grid;
  place-items: center;
  border-radius: 14px;
  background: #fffaf0;
  font-size: 24px;
}

.brand h1 {
  margin: 0;
  font-size: 20px;
  line-height: 1.1;
}

.brand p {
  margin: 4px 0 0;
  color: #e7d7be;
  font-size: 13px;
}

.nav {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.nav-button {
  width: 100%;
  padding: 12px 14px;
  border: none;
  border-radius: 14px;
  background: transparent;
  color: #f4e8d3;
  display: flex;
  align-items: center;
  gap: 10px;
  text-align: left;
  font-size: 15px;
  cursor: pointer;
}

.nav-button:hover,
.nav-button.active {
  background: #fffaf0;
  color: #3f2b1d;
}

.sidebar-footer {
  margin-top: auto;
  padding: 12px;
  border-radius: 16px;
  background: rgba(255, 250, 240, 0.1);
}

.sidebar-footer p {
  margin: 0 0 4px;
  color: #e7d7be;
  font-size: 12px;
}

.sidebar-footer strong {
  font-size: 13px;
}

.content {
  min-width: 0;
  height: 100%;
  overflow-y: auto;
  padding: 28px;
  box-sizing: border-box;
}

.content-header {
  display: flex;
  justify-content: space-between;
  gap: 20px;
  align-items: flex-start;
  margin-bottom: 18px;
}

.content-header h2 {
  margin: 4px 0 6px;
  font-size: 32px;
}

.content-header p {
  margin: 0;
  color: #7a6652;
}

.eyebrow {
  color: #8b6f47 !important;
  font-weight: 800;
  letter-spacing: 0.08em;
  text-transform: uppercase;
  font-size: 12px;
}

.view-stack {
  display: flex;
  flex-direction: column;
  gap: 18px;
}

.notice,
.panel,
.empty-state {
  border-radius: 22px;
  background: rgba(255, 250, 240, 0.92);
  box-shadow: 0 10px 30px rgba(67, 47, 27, 0.1);
}

.notice {
  margin-bottom: 18px;
  padding: 14px 18px;
  color: #7a4f22;
  font-weight: 800;
}

.panel {
  padding: 20px;
}

.empty-state {
  padding: 34px;
  text-align: center;
}

.empty-state h3 {
  margin: 0 0 8px;
}

.empty-state p {
  margin: 0;
  color: #7a6652;
}

.panel-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 12px;
  margin-bottom: 16px;
}

.panel-header h3 {
  margin: 0;
  font-size: 22px;
}

.panel-header span {
  color: #7a6652;
  font-weight: 800;
}

.status-grid,
.summary-row,
.tool-grid {
  display: grid;
  gap: 12px;
}

.status-grid {
  grid-template-columns: repeat(2, minmax(0, 1fr));
}

.status-card,
.summary-row > div,
.setting-block {
  padding: 14px;
  border-radius: 16px;
  background: #f6ead8;
}

.status-card span,
.summary-row span,
.setting-block span {
  display: block;
  color: #7a6652;
  font-size: 13px;
  margin-bottom: 6px;
}

.status-card strong,
.summary-row strong,
.setting-block strong {
  word-break: break-all;
}

.summary-row {
  grid-template-columns: repeat(4, minmax(0, 1fr));
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
  padding: 16px;
  border-radius: 18px;
  background: #f6ead8;
}

.mod-item.disabled {
  opacity: 0.72;
}

.mod-item.warning {
  background: #f7dfd8;
}

.mod-main {
  min-width: 0;
}

.mod-item h4 {
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
  line-height: 1.45;
}

.mod-actions {
  flex-shrink: 0;
  display: flex;
  flex-direction: column;
  align-items: flex-end;
  gap: 8px;
}

.mod-folder {
  max-width: 190px;
  padding: 5px 8px;
  border-radius: 999px;
  background: #e2d1b8;
  color: #5c4630;
  font-size: 12px;
  font-weight: 800;
  text-align: right;
  word-break: break-all;
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

.missing-list {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.missing-item {
  padding: 12px;
  border-radius: 12px;
  background: #f7dfd8;
  color: #6f2d20;
}

.missing-item p {
  margin: 6px 0 0;
  font-size: 14px;
}

.tool-grid {
  grid-template-columns: repeat(2, minmax(0, 1fr));
}

.setting-actions {
  margin-top: 16px;
}

.log-viewer {
  max-height: 560px;
  overflow: auto;
  margin: 0;
  padding: 14px;
  border-radius: 14px;
  background: #2d241b;
  color: #fff7e8;
  font-family: Consolas, "Courier New", monospace;
  font-size: 12px;
  line-height: 1.5;
  white-space: pre-wrap;
}

.right-panel {
  height: 100%;
  padding: 22px 18px;
  box-sizing: border-box;
  overflow-y: auto;
  background: rgba(255, 250, 240, 0.62);
  border-left: 1px solid rgba(92, 70, 48, 0.14);
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.launch-card,
.side-card {
  padding: 18px;
  border-radius: 22px;
  background: #fffaf0;
  box-shadow: 0 10px 30px rgba(67, 47, 27, 0.1);
}

.junimo-badge {
  width: 52px;
  height: 52px;
  display: grid;
  place-items: center;
  border-radius: 18px;
  background: #e3f0d6;
  font-size: 28px;
  margin-bottom: 12px;
}

.launch-card h3,
.side-card h4 {
  margin: 0 0 8px;
}

.launch-card p,
.path-card p {
  margin: 0;
  color: #7a6652;
  font-size: 14px;
  line-height: 1.45;
  word-break: break-all;
}

.launch-button {
  width: 100%;
  margin-top: 16px;
  padding: 14px 18px;
  border-radius: 16px;
  background: #6fa85f;
  font-size: 17px;
  font-weight: 800;
}

.side-actions {
  display: grid;
  gap: 10px;
}

.side-actions button {
  width: 100%;
}

.info-line {
  display: flex;
  justify-content: space-between;
  gap: 10px;
  padding: 10px 0;
  border-bottom: 1px solid rgba(92, 70, 48, 0.12);
}

.info-line:last-child {
  border-bottom: none;
}

.info-line span {
  color: #7a6652;
}

button {
  padding: 11px 16px;
  border: none;
  border-radius: 13px;
  background: #6fa85f;
  color: white;
  font-size: 15px;
  font-weight: 800;
  cursor: pointer;
}

button:hover:not(:disabled) {
  background: #5d944f;
}

button:disabled {
  opacity: 0.45;
  cursor: not-allowed;
}

button.secondary {
  background: #8b6f47;
}

button.secondary:hover:not(:disabled) {
  background: #755d3c;
}

.tiny-button {
  padding: 7px 11px;
  border-radius: 999px;
  font-size: 12px;
  background: #8b6f47;
}

.tiny-button:hover:not(:disabled) {
  background: #755d3c;
}

.tiny-button.danger {
  background: #b65b4b;
}

.tiny-button.danger:hover:not(:disabled) {
  background: #9f493c;
}

.ok {
  color: #2f8f46;
  font-weight: 800;
}

.bad {
  color: #c0392b;
  font-weight: 800;
}

.optional {
  color: #9a6a2f;
  font-weight: 800;
}

.success-text {
  margin: 0;
  color: #2f8f46;
  font-weight: 800;
}

@media (max-width: 1100px) {
  .app-shell {
    grid-template-columns: 190px minmax(0, 1fr);
  }

  .right-panel {
    display: none;
  }
}
</style>