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

        <div class="header-actions">
          <button
            v-if="activeView === 'overview'"
            class="secondary compact-header-button"
            @click="handleSelectPath"
          >
            选择目录
          </button>

          <button
            v-if="activeView === 'mods' && gamePath"
            class="secondary compact-header-button"
            @click="scanMods"
          >
            重新扫描
          </button>

          <button
            v-if="activeView === 'logs'"
            class="secondary compact-header-button"
            @click="handleReadLatestSmapiLog"
          >
            读取最新日志
          </button>
        </div>
      </header>

      <div v-if="message" class="notice">
        {{ message }}
      </div>

      <section v-if="activeView === 'overview'" class="view-stack">
        <div class="overview-grid">
          <div class="panel compact-panel">
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

          <div class="panel compact-panel">
            <div class="panel-header">
              <h3>Mod 概览</h3>
              <span>{{ totalModCount }} 个</span>
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
        </div>
      </section>

      <section v-if="activeView === 'mods'" class="view-stack">
        <div class="panel filter-panel">
          <div class="filter-top-row">
            <div class="search-box">
              <span>🔎</span>
              <input
                v-model="modSearchQuery"
                type="text"
                placeholder="搜索 Mod 名称、作者、UniqueID、文件夹或描述..."
              />
            </div>

            <button class="tiny-button" @click="clearModFilters">
              清空筛选
            </button>
          </div>

          <div class="filter-row">
            <div class="filter-group">
              <span class="filter-label">状态</span>
              <button
                v-for="option in statusFilterOptions"
                :key="option.value"
                class="filter-chip"
                :class="{ active: modStatusFilter === option.value }"
                @click="modStatusFilter = option.value"
              >
                {{ option.label }}
              </button>
            </div>

            <div class="filter-group">
              <span class="filter-label">依赖</span>
              <button
                v-for="option in dependencyFilterOptions"
                :key="option.value"
                class="filter-chip"
                :class="{ active: modDependencyFilter === option.value }"
                @click="modDependencyFilter = option.value"
              >
                {{ option.label }}
              </button>
            </div>
          </div>

          <p class="filter-result-text">
            当前显示 {{ filteredMods.length }} / {{ allDisplayMods.length }} 个 Mod。
          </p>
        </div>

        <div v-if="lastInstalledZipMods.length > 0" class="panel">
          <div class="panel-header">
            <h3>最近安装</h3>
            <span>{{ lastInstalledZipMods.length }} 个</span>
          </div>

          <div class="mods-list compact-mods-list">
            <article
              v-for="mod in lastInstalledZipMods"
              :key="mod.unique_id || mod.manifest_path"
              class="mod-item install-result"
            >
              <div class="mod-main">
                <h4>{{ mod.name }}</h4>
                <p class="mod-meta">
                  {{ mod.author || "未知作者" }} · v{{ mod.version || "未知版本" }}
                </p>
                <p class="mod-description">UniqueID：{{ mod.unique_id || "未提供" }}</p>
                <p class="mod-description">安装文件夹：{{ mod.suggested_folder }}</p>
              </div>
            </article>
          </div>

          <div v-if="missingDependencies.length > 0" class="install-warning">
            <strong>⚠️ 安装后发现缺失依赖</strong>
            <p>有 {{ missingDependencies.length }} 项必需依赖未安装。请查看下方“依赖检查”面板。</p>
          </div>

          <div v-else class="install-success">
            <strong>✅ 依赖检查正常</strong>
            <p>当前已启用 Mod 没有发现缺失的必需依赖。</p>
          </div>
        </div>

        <div v-if="mods.length > 0" class="panel slim-panel">
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

        <div v-if="filteredMods.length > 0" class="panel">
          <div class="panel-header sticky-panel-header">
            <h3>Mod 列表</h3>
            <span>{{ filteredMods.length }} 个</span>
          </div>

          <div class="mods-list">
            <article
              v-for="mod in filteredMods"
              :key="`${mod.isDisabled ? 'disabled' : 'enabled'}-${mod.uniqueId || mod.folderName}`"
              class="mod-item"
              :class="{ disabled: mod.isDisabled, warning: mod.hasMissingRequiredDependency }"
            >
              <div class="mod-main">
                <div class="mod-title-row">
                  <h4>{{ mod.name }}</h4>
                  <div class="mod-badges">
                    <span class="status-badge" :class="mod.isDisabled ? 'disabled-badge' : 'enabled-badge'">
                      {{ mod.isDisabled ? "已禁用" : "已启用" }}
                    </span>
                                        <span class="status-badge type-badge">
                      {{ mod.modType.label }}
                    </span>
                    <span v-if="mod.hasMissingRequiredDependency" class="status-badge missing-badge">
                      缺失依赖
                    </span>
                  </div>
                </div>

                <p class="mod-meta">
                  {{ mod.author || "未知作者" }} · v{{ mod.version || "未知版本" }}
                </p>

                <p class="mod-description">
                  {{ mod.description || "没有描述。" }}
                </p>

                <div class="mod-extra-row">
                  <span>UniqueID：{{ mod.uniqueId || "未提供" }}</span>
                  <span>文件夹：{{ mod.folderName }}</span>
                </div>

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
                <span class="mod-folder desktop-folder">
                  {{ mod.folderName }}
                </span>

                <div class="mod-button-row">
                  <button class="tiny-button" @click="handleOpenDisplayedModFolder(mod)">
                    打开
                  </button>

                  <button
                    v-if="mod.isDisabled"
                    class="tiny-button"
                    @click="handleEnableMod(mod.folderName)"
                  >
                    启用
                  </button>

                  <button
                    v-else
                    class="tiny-button danger"
                    @click="handleDisableMod(mod.folderName)"
                  >
                    禁用
                  </button>
                </div>
              </div>
            </article>
          </div>
        </div>

        <div
          v-else-if="gamePath && allDisplayMods.length > 0"
          class="empty-state"
        >
          <h3>没有符合条件的 Mod</h3>
          <p>试试清空搜索词，或者切换筛选条件。</p>
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
          v-if="gamePath && allDisplayMods.length === 0 && skippedFolders.length === 0"
          class="empty-state"
        >
          <h3>还没有扫描到 Mod</h3>
          <p>点击“重新扫描”或进入工具箱扫描 Mods 文件夹。</p>
        </div>
      </section>

      <section v-if="activeView === 'logs'" class="view-stack">
        <div v-if="smapiLogAnalysis" class="panel">
          <div class="panel-header">
            <h3>日志诊断摘要</h3>
            <span>{{ smapiLogFileName }}</span>
          </div>

          <div class="diagnosis-grid">
            <div class="diagnosis-card">
              <span>SMAPI 版本</span>
              <strong>{{ smapiLogAnalysis.smapiVersion || "未识别" }}</strong>
            </div>

            <div class="diagnosis-card">
              <span>游戏版本</span>
              <strong>{{ smapiLogAnalysis.gameVersion || "未识别" }}</strong>
            </div>

            <div class="diagnosis-card">
              <span>警告</span>
              <strong :class="smapiLogAnalysis.warningLines.length > 0 ? 'bad' : 'ok'">
                {{ smapiLogAnalysis.warningLines.length }}
              </strong>
            </div>

            <div class="diagnosis-card">
              <span>错误</span>
              <strong :class="smapiLogAnalysis.errorLines.length > 0 ? 'bad' : 'ok'">
                {{ smapiLogAnalysis.errorLines.length }}
              </strong>
            </div>
          </div>

          <div v-if="smapiLogAnalysis.modsPath" class="diagnosis-section">
            <h4>Mods 路径</h4>
            <p class="code-text">{{ smapiLogAnalysis.modsPath }}</p>
          </div>

          <div v-if="smapiLogAnalysis.suggestions.length > 0" class="diagnosis-section">
            <h4>建议处理</h4>
            <ul class="diagnosis-list">
              <li v-for="suggestion in smapiLogAnalysis.suggestions" :key="suggestion">
                {{ suggestion }}
              </li>
            </ul>
          </div>

          <div v-if="smapiLogAnalysis.affectedAssets.length > 0" class="diagnosis-section warning-box">
            <h4>受影响的游戏文件</h4>
            <p>SMAPI 检测到游戏原始内容文件可能被修改或损坏。常见原因是旧式 XNB 模组覆盖了游戏文件。</p>
            <ul class="diagnosis-list">
              <li v-for="asset in smapiLogAnalysis.affectedAssets" :key="asset">
                {{ asset }}
              </li>
            </ul>
          </div>

          <div v-if="smapiLogAnalysis.skippedMods.length > 0" class="diagnosis-section warning-box">
            <h4>被 SMAPI 跳过的 Mod</h4>
            <div
              v-for="skippedMod in smapiLogAnalysis.skippedMods"
              :key="skippedMod.path"
              class="diagnosis-item"
            >
              <strong>{{ skippedMod.path }}</strong>
              <p>{{ skippedMod.reason || "SMAPI 跳过了这个文件夹。" }}</p>
            </div>
          </div>

          <div v-if="smapiLogAnalysis.errorLines.length > 0" class="diagnosis-section error-box">
            <h4>错误行</h4>
            <pre class="small-log">{{ smapiLogAnalysis.errorLines.join("\n") }}</pre>
          </div>

          <div v-if="smapiLogAnalysis.warningLines.length > 0" class="diagnosis-section">
            <h4>警告行</h4>
            <pre class="small-log">{{ smapiLogAnalysis.warningLines.join("\n") }}</pre>
          </div>
        </div>

        <div v-if="smapiLogContent" class="panel">
          <div class="panel-header">
            <h3>原始日志</h3>
            <button class="tiny-button" @click="showRawSmapiLog = !showRawSmapiLog">
              {{ showRawSmapiLog ? "收起" : "展开" }}
            </button>
          </div>

          <pre v-if="showRawSmapiLog" class="log-viewer">{{ smapiLogContent }}</pre>

          <p v-else class="muted-text">
            原始日志已隐藏。通常只需要查看上方诊断摘要；需要完整内容时再展开。
          </p>
        </div>

        <div v-if="!smapiLogContent" class="empty-state">
          <h3>还没有读取日志</h3>
          <p>点击“读取最新日志”，Junimo Box 会读取最近一次 SMAPI 日志并生成诊断摘要。</p>
        </div>
      </section>

      <section v-if="activeView === 'tools'" class="view-stack">
        <div class="panel compact-panel">
          <div class="panel-header">
            <h3>工具箱</h3>
            <span>快捷操作</span>
          </div>

          <div class="tool-grid">
            <button @click="handleOpenGameFolder">打开游戏目录</button>

            <button :disabled="!modsFolderExists" @click="handleOpenModsFolder">
              打开 Mods 文件夹
            </button>

            <button @click="handleOpenSmapiLogFolder">打开日志文件夹</button>

            <button
              :disabled="mods.length === 0 && disabledMods.length === 0"
              @click="handleExportModList"
            >
              导出 Mod 列表
            </button>

            <button :disabled="!gamePath" @click="handleExportProblemReport">
              导出问题报告
            </button>

            <button @click="handlePreviewZipMod">预览 ZIP Mod</button>
          </div>

          <div
            class="zip-drop-zone"
            :class="{ active: isZipDragOver }"
            @click="handlePreviewZipMod"
          >
            <div class="zip-drop-icon">📦</div>
            <div>
              <strong>拖拽 ZIP Mod 到这里</strong>
              <p>把下载好的 .zip 文件拖进窗口，Junimo Box 会自动生成安装预览；也可以点击这里手动选择。</p>
            </div>
          </div>
        </div>

        <div v-if="zipModPreviews.length > 0" class="panel">
          <div class="panel-header">
            <h3>ZIP Mod 安装预览</h3>
            <div class="panel-actions">
              <span>{{ zipModPreviews.length }} 个</span>
              <button
                class="tiny-button"
                :disabled="!gamePath"
                @click="handleInstallZipMod"
              >
                安装到 Mods
              </button>
            </div>
          </div>

          <p class="muted-text path-text">当前压缩包：{{ selectedZipPath }}</p>

          <div
            class="zip-dependency-summary"
            :class="zipMissingRequiredDependencies.length > 0 ? 'has-warning' : 'is-ok'"
          >
            <strong>安装前依赖检查</strong>
            <p v-if="zipMissingRequiredDependencies.length === 0">
              ✅ 必需依赖已满足，或依赖也包含在这个 ZIP 中。
            </p>
            <p v-else>
              ⚠️ 缺少 {{ zipMissingRequiredDependencies.length }} 项必需依赖。安装后对应 Mod 可能无法正常加载。
            </p>

            <ul v-if="zipMissingRequiredDependencies.length > 0" class="zip-missing-list">
              <li
                v-for="dependency in zipMissingRequiredDependencies"
                :key="dependency.uniqueId"
              >
                {{ dependency.uniqueId }}：被 {{ dependency.requiredBy.join("、") }} 需要
              </li>
            </ul>
          </div>

          <div class="mods-list zip-preview-list">
            <article
              v-for="mod in zipModPreviews"
              :key="mod.unique_id || mod.manifest_path"
              class="mod-item"
            >
              <div class="mod-main">
                <h4>{{ mod.name }}</h4>
                <p class="mod-meta">
                  {{ mod.author || "未知作者" }} · v{{ mod.version || "未知版本" }}
                </p>
                <p class="mod-description">{{ mod.description || "没有描述。" }}</p>
                <p class="mod-description">UniqueID：{{ mod.unique_id || "未提供" }}</p>
                <p class="mod-description">manifest：{{ mod.manifest_path }}</p>

                <div
                  v-if="getZipDependencyRows(mod).length > 0"
                  class="zip-dependencies"
                >
                  <p class="dependency-title">依赖检查</p>
                  <p
                    v-for="dependency in getZipDependencyRows(mod)"
                    :key="dependency.uniqueId"
                    class="dependency-line"
                  >
                    依赖：
                    <span :class="dependency.className">
                      {{ dependency.uniqueId }}
                      {{ dependency.statusLabel }}
                    </span>
                  </p>
                </div>
              </div>

              <div class="mod-actions">
                <span class="mod-type">{{ getZipModType(mod).label }}</span>
                <span class="mod-folder">{{ mod.suggested_folder }}</span>
              </div>
            </article>
          </div>
        </div>
      </section>

      <section v-if="activeView === 'settings'" class="view-stack">
        <div class="panel compact-panel">
          <div class="panel-header">
            <h3>基础设置</h3>
            <span>本地配置</span>
          </div>

          <div class="setting-block">
            <span>当前游戏路径</span>
            <strong>{{ gamePath || "未选择" }}</strong>
          </div>

          <div class="setting-actions">
            <button @click="handleSelectPath">重新选择游戏目录</button>
          </div>
        </div>
      </section>
    </section>

    <aside class="right-panel">
      <div class="launch-card">
        <div class="junimo-badge">🌱</div>
        <div>
          <h3>启动中心</h3>
          <p>管理你的星露谷 Mod 环境</p>
        </div>

        <button
          class="launch-button"
          :disabled="!smapiExists"
          @click="handleLaunchSmapi"
        >
          启动 SMAPI
        </button>

        <button
          class="launch-button vanilla-button"
          :disabled="!stardewExists"
          @click="handleLaunchVanilla"
        >
          启动原版
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

        <div class="info-line">
          <span>已启用 Mods</span>
          <strong>{{ mods.length }}</strong>
        </div>

        <div class="info-line">
          <span>缺失依赖</span>
          <strong :class="missingDependencies.length > 0 ? 'bad' : 'ok'">
            {{ missingDependencies.length }}
          </strong>
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
import { computed, onMounted, onUnmounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWebview } from "@tauri-apps/api/webview";
import { open, save } from "@tauri-apps/plugin-dialog";
import { exists, readDir, readTextFile } from "@tauri-apps/plugin-fs";
import JSON5 from "json5";

const STORAGE_KEY = "junimo-box-game-path";

type ViewId = "overview" | "mods" | "logs" | "tools" | "settings";
type ModStatusFilter = "all" | "enabled" | "disabled";
type ModDependencyFilter = "all" | "missing";

type ViewMeta = {
  eyebrow: string;
  title: string;
  description: string;
};

type ModDependency = {
  uniqueId: string;
  isRequired: boolean;
  isInstalled: boolean;
};

type MissingDependency = {
  uniqueId: string;
  requiredBy: string[];
};

type ModTypeId =
  | "smapi-plugin"
  | "content-patcher-pack"
  | "fashion-sense-pack"
  | "json-assets-pack"
  | "generic-mod-config-menu"
  | "stardew-valley-expanded-pack"
  | "content-pack"
  | "unknown";

type ModTypeInfo = {
  id: ModTypeId;
  label: string;
};

type ModInfo = {
  name: string;
  author: string;
  version: string;
  description: string;
  uniqueId: string;
  folderName: string;
  entryDll: string;
  dependencies: ModDependency[];
  contentPackFor?: ModDependency;
};

type DisplayModInfo = ModInfo & {
  isDisabled: boolean;
  hasMissingRequiredDependency: boolean;
  modType: ModTypeInfo;
};

type SkippedModInfo = {
  path: string;
  reason: string;
};

type SmapiLogAnalysis = {
  smapiVersion: string;
  gameVersion: string;
  modsPath: string;
  warningLines: string[];
  errorLines: string[];
  skippedMods: SkippedModInfo[];
  affectedAssets: string[];
  suggestions: string[];
};

type ZipModDependency = {
  unique_id: string;
  is_required: boolean;
};

type ZipModPreview = {
  name: string;
  author: string;
  version: string;
  description: string;
  unique_id: string;
  manifest_path: string;
  suggested_folder: string;
  entry_dll: string;
  dependencies: ZipModDependency[];
  content_pack_for?: ZipModDependency;
};

type ZipDependencyRow = {
  uniqueId: string;
  isRequired: boolean;
  isInstalled: boolean;
  isBundled: boolean;
  statusLabel: string;
  className: string;
};

const navItems: Array<{ id: ViewId; label: string; icon: string }> = [
  { id: "overview", label: "总览", icon: "🏡" },
  { id: "mods", label: "Mods", icon: "📦" },
  { id: "logs", label: "日志", icon: "📜" },
  { id: "tools", label: "工具箱", icon: "🧰" },
  { id: "settings", label: "设置", icon: "⚙️" },
];

const statusFilterOptions: Array<{ value: ModStatusFilter; label: string }> = [
  { value: "all", label: "全部" },
  { value: "enabled", label: "已启用" },
  { value: "disabled", label: "已禁用" },
];

const dependencyFilterOptions: Array<{
  value: ModDependencyFilter;
  label: string;
}> = [
  { value: "all", label: "全部依赖" },
  { value: "missing", label: "缺失依赖" },
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
const smapiLogAnalysis = ref<SmapiLogAnalysis | null>(null);
const showRawSmapiLog = ref(false);

const selectedZipPath = ref("");
const zipModPreviews = ref<ZipModPreview[]>([]);
const lastInstalledZipMods = ref<ZipModPreview[]>([]);
const isZipDragOver = ref(false);

let unlistenDragDrop: (() => void) | null = null;

const modSearchQuery = ref("");
const modStatusFilter = ref<ModStatusFilter>("all");
const modDependencyFilter = ref<ModDependencyFilter>("all");

const enabledModUniqueIds = computed(() =>
  new Set(mods.value.map((mod) => mod.uniqueId).filter(Boolean))
);

const zipPreviewUniqueIds = computed(() =>
  new Set(zipModPreviews.value.map((mod) => mod.unique_id).filter(Boolean))
);

const zipMissingRequiredDependencies = computed(() => {
  const missingMap = new Map<string, string[]>();

  for (const mod of zipModPreviews.value) {
    for (const dependency of getZipDependencyRows(mod)) {
      if (!dependency.isRequired || dependency.isInstalled || dependency.isBundled) {
        continue;
      }

      const requiredBy = missingMap.get(dependency.uniqueId) || [];
      requiredBy.push(mod.name || mod.suggested_folder);
      missingMap.set(dependency.uniqueId, requiredBy);
    }
  }

  return Array.from(missingMap.entries())
    .map(([uniqueId, requiredBy]) => ({ uniqueId, requiredBy }))
    .sort((a, b) => a.uniqueId.localeCompare(b.uniqueId));
});

const totalModCount = computed(() => mods.value.length + disabledMods.value.length);

const viewMetaMap: Record<ViewId, ViewMeta> = {
  overview: {
    eyebrow: "Overview",
    title: "总览",
    description: "查看当前游戏环境、Mod 数量和依赖状态。",
  },
  mods: {
    eyebrow: "Local Mods",
    title: "本地 Mod 管理",
    description: "搜索、筛选、查看、启用或禁用 Stardew Valley Mods。",
  },
  logs: {
    eyebrow: "SMAPI Logs",
    title: "SMAPI 日志",
    description: "读取最近一次 SMAPI 日志，并生成基础诊断摘要。",
  },
  tools: {
    eyebrow: "Toolbox",
    title: "工具箱",
    description: "打开常用目录，导出报告，预览并安装 ZIP Mod。",
  },
  settings: {
    eyebrow: "Settings",
    title: "设置",
    description: "管理本地路径和 Junimo Box 基础偏好。",
  },
};

const currentViewMeta = computed<ViewMeta>(() => viewMetaMap[activeView.value]);

const allDisplayMods = computed<DisplayModInfo[]>(() => [
  ...mods.value.map((mod) => createDisplayMod(mod, false)),
  ...disabledMods.value.map((mod) => createDisplayMod(mod, true)),
]);

const filteredMods = computed<DisplayModInfo[]>(() => {
  const query = modSearchQuery.value.trim().toLowerCase();

  return allDisplayMods.value.filter((mod: DisplayModInfo) => {
    if (modStatusFilter.value === "enabled" && mod.isDisabled) {
      return false;
    }

    if (modStatusFilter.value === "disabled" && !mod.isDisabled) {
      return false;
    }

    if (modDependencyFilter.value === "missing" && !mod.hasMissingRequiredDependency) {
      return false;
    }

    if (!query) {
      return true;
    }

    return [
      mod.name,
      mod.author,
      mod.version,
      mod.description,
      mod.uniqueId,
      mod.folderName,
      mod.modType.label,
    ]
      .join(" ")
      .toLowerCase()
      .includes(query);
  });
});

onMounted(async () => {
  await setupZipDragDrop();

  const savedPath = localStorage.getItem(STORAGE_KEY);

  if (!savedPath) {
    return;
  }

  gamePath.value = savedPath;
  await checkGameFiles(savedPath);
  await scanMods();
});

onUnmounted(() => {
  if (unlistenDragDrop) {
    unlistenDragDrop();
    unlistenDragDrop = null;
  }
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
  lastInstalledZipMods.value = [];

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

    const foundMods = await collectModsFromFolder(modsFolder, "", true);
    const enabledMods = attachDependencyStatus(foundMods, foundMods).sort((a, b) =>
      a.name.localeCompare(b.name)
    );

    mods.value = enabledMods;

    if (await exists(disabledModsFolder)) {
      const foundDisabledMods = await collectModsFromFolder(disabledModsFolder, "", false);

      disabledMods.value = attachDependencyStatus(foundDisabledMods, enabledMods).sort((a, b) =>
        a.name.localeCompare(b.name)
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
  relativePath: string,
  trackSkippedFolders: boolean
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
        entryDll: manifest.EntryDll || "",
        dependencies: normalizeDependencies(manifest.Dependencies),
        contentPackFor: normalizeContentPackFor(manifest.ContentPackFor),
      });

      return foundMods;
    } catch (error) {
      console.warn(`读取 manifest 失败：${manifestPath}`, error);

      if (trackSkippedFolders) {
        skippedFolders.value.push(
          `${folderLabel}：manifest.json 读取或解析失败 - ${String(error)}`
        );
      }

      return foundMods;
    }
  }

  for (const entry of entries) {
    if (!entry.isDirectory) {
      continue;
    }

    const childPath = `${folderPath}\\${entry.name}`;
    const childRelativePath = relativePath ? `${relativePath}\\${entry.name}` : entry.name;

    const childMods = await collectModsFromFolder(
      childPath,
      childRelativePath,
      trackSkippedFolders
    );

    foundMods.push(...childMods);
  }

  const depth = relativePath ? relativePath.split("\\").length : 0;

  if (trackSkippedFolders && !hasManifest && foundMods.length === 0 && depth <= 2 && relativePath) {
    skippedFolders.value.push(`${folderLabel}：没有找到 manifest.json`);
  }

  return foundMods;
}

async function handleLaunchSmapi() {
  if (!gamePath.value) {
    message.value = "请先选择游戏目录。";
    return;
  }

  await checkGameFiles(gamePath.value);

  if (!smapiExists.value) {
    message.value = "未找到 StardewModdingAPI.exe，无法启动 SMAPI。";
    return;
  }

  await launchExecutable(`${gamePath.value}\\StardewModdingAPI.exe`, "正在通过 SMAPI 启动游戏...");
}

async function handleLaunchVanilla() {
  if (!gamePath.value) {
    message.value = "请先选择游戏目录。";
    return;
  }

  await checkGameFiles(gamePath.value);

  if (!stardewExists.value) {
    message.value = "未找到 Stardew Valley.exe，无法启动原版游戏。";
    return;
  }

  await launchExecutable(`${gamePath.value}\\Stardew Valley.exe`, "正在启动原版 Stardew Valley...");
}

async function launchExecutable(path: string, successMessage: string) {
  try {
    await invoke("launch_game", { path });
    message.value = successMessage;
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
    await invoke("open_folder", { path: gamePath.value });
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
    await invoke("open_folder", { path: modsFolder });
    message.value = "已打开 Mods 文件夹。";
  } catch (error) {
    message.value = `打开 Mods 文件夹失败：${String(error)}`;
  }
}

async function handleOpenDisplayedModFolder(mod: DisplayModInfo) {
  if (!gamePath.value) {
    message.value = "请先选择游戏目录。";
    return;
  }

  const rootFolder = mod.isDisabled ? "Disabled Mods" : "Mods";
  const modFolder = `${gamePath.value}\\${rootFolder}\\${mod.folderName}`;

  if (!(await exists(modFolder))) {
    message.value = `未找到 Mod 文件夹：${mod.folderName}`;
    return;
  }

  try {
    await invoke("open_folder", { path: modFolder });
    message.value = `已打开 Mod 文件夹：${mod.folderName}`;
  } catch (error) {
    message.value = `打开 Mod 文件夹失败：${String(error)}`;
  }
}

async function handleDisableMod(folderName: string) {
  if (!gamePath.value) {
    message.value = "请先选择游戏目录。";
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
    await invoke("move_folder", { from, to });
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
    await invoke("move_folder", { from, to });
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
    filters: [{ name: "JSON 文件", extensions: ["json"] }],
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

async function handleExportProblemReport() {
  if (!gamePath.value) {
    message.value = "请先选择游戏目录。";
    return;
  }

  if (mods.value.length === 0 && disabledMods.value.length === 0) {
    await scanMods();
  }

  if (!smapiLogAnalysis.value) {
    try {
      const result = await invoke<string[]>("read_latest_smapi_log");

      smapiLogFileName.value = result[0] || "未知日志文件";
      smapiLogContent.value = result[1] || "";
      smapiLogAnalysis.value = analyzeSmapiLog(smapiLogContent.value);
      showRawSmapiLog.value = false;
    } catch {
      smapiLogFileName.value = "";
      smapiLogContent.value = "";
      smapiLogAnalysis.value = null;
    }
  }

  const filePath = await save({
    title: "导出问题报告",
    defaultPath: "junimo-box-problem-report.txt",
    filters: [{ name: "文本文件", extensions: ["txt"] }],
  });

  if (!filePath) {
    return;
  }

  const reportText = createProblemReportText();

  try {
    await invoke("write_text_file", { path: filePath, content: reportText });
    message.value = `已导出问题报告：${filePath}`;
  } catch (error) {
    message.value = `导出问题报告失败：${String(error)}`;
  }
}

async function setupZipDragDrop() {
  try {
    unlistenDragDrop = await getCurrentWebview().onDragDropEvent((event) => {
      const payload = event.payload;

      if (payload.type === "over") {
        isZipDragOver.value = true;
        return;
      }

      if (payload.type === "leave") {
        isZipDragOver.value = false;
        return;
      }

      if (payload.type === "drop") {
        isZipDragOver.value = false;

        const zipPath = payload.paths.find((path) =>
          path.toLowerCase().endsWith(".zip")
        );

        if (!zipPath) {
          activeView.value = "tools";
          message.value = "请拖入 .zip 格式的 Mod 压缩包。";
          return;
        }

        activeView.value = "tools";
        void previewZipPath(zipPath);
      }
    });
  } catch (error) {
    console.warn("注册 ZIP 拖拽事件失败", error);
  }
}

async function handlePreviewZipMod() {
  const selected = await open({
    directory: false,
    multiple: false,
    title: "选择 Mod ZIP 压缩包",
    filters: [{ name: "ZIP 压缩包", extensions: ["zip"] }],
  });

  if (typeof selected !== "string") {
    return;
  }

  await previewZipPath(selected);
}

async function previewZipPath(zipPath: string) {
  if (!zipPath.toLowerCase().endsWith(".zip")) {
    selectedZipPath.value = zipPath;
    zipModPreviews.value = [];
    activeView.value = "tools";
    message.value = "请选择 .zip 格式的 Mod 压缩包。";
    return;
  }

  selectedZipPath.value = zipPath;
  zipModPreviews.value = [];
  activeView.value = "tools";

  try {
    const previews = await invoke<ZipModPreview[]>("preview_zip_mods", {
      zipPath,
    });

    zipModPreviews.value = previews;
    message.value = `ZIP 预览完成：找到 ${previews.length} 个 Mod。`;
  } catch (error) {
    selectedZipPath.value = zipPath;
    zipModPreviews.value = [];
    message.value = `ZIP 预览失败：${String(error)}`;
  }
}

async function handleInstallZipMod() {
  if (!gamePath.value) {
    message.value = "请先选择游戏目录。";
    return;
  }

  if (!selectedZipPath.value || zipModPreviews.value.length === 0) {
    message.value = "请先预览 ZIP Mod。";
    return;
  }

  try {
    const installedMods = await invoke<ZipModPreview[]>("install_zip_mods", {
      zipPath: selectedZipPath.value,
      gamePath: gamePath.value,
    });

    lastInstalledZipMods.value = installedMods;
    selectedZipPath.value = "";
    zipModPreviews.value = [];

    await checkGameFiles(gamePath.value);
    await scanMods();

    message.value =
      missingDependencies.value.length > 0
        ? `安装完成：已安装 ${installedMods.length} 个 Mod，但发现 ${missingDependencies.value.length} 项缺失依赖。`
        : `安装完成：已安装 ${installedMods.length} 个 Mod，依赖检查正常。`;

    activeView.value = "mods";
  } catch (error) {
    message.value = `安装 ZIP Mod 失败：${String(error)}`;
  }
}

async function handleReadLatestSmapiLog() {
  try {
    const result = await invoke<string[]>("read_latest_smapi_log");

    smapiLogFileName.value = result[0] || "未知日志文件";
    smapiLogContent.value = result[1] || "";
    smapiLogAnalysis.value = analyzeSmapiLog(smapiLogContent.value);
    showRawSmapiLog.value = false;

    message.value = `已读取并分析最新 SMAPI 日志：${smapiLogFileName.value}`;
  } catch (error) {
    smapiLogFileName.value = "";
    smapiLogContent.value = "";
    smapiLogAnalysis.value = null;
    showRawSmapiLog.value = false;
    message.value = `读取 SMAPI 日志失败：${String(error)}`;
  }
}

async function handleOpenSmapiLogFolder() {
  try {
    const logFolder = await invoke<string>("get_smapi_log_folder");
    await invoke("open_folder", { path: logFolder });
    message.value = "已打开 SMAPI 日志文件夹。";
  } catch (error) {
    message.value = `打开 SMAPI 日志文件夹失败：${String(error)}`;
  }
}

function getZipModType(mod: ZipModPreview): ModTypeInfo {
  const contentPackFor = mod.content_pack_for?.unique_id || "";

  const contentPackTypeMap: Record<string, ModTypeInfo> = {
    "Pathoschild.ContentPatcher": {
      id: "content-patcher-pack",
      label: "CP 内容包",
    },
    "PeacefulEnd.FashionSense": {
      id: "fashion-sense-pack",
      label: "Fashion Sense",
    },
    "spacechase0.JsonAssets": {
      id: "json-assets-pack",
      label: "Json Assets",
    },
    "FlashShifter.StardewValleyExpandedCP": {
      id: "stardew-valley-expanded-pack",
      label: "SVE 内容包",
    },
  };

  if (contentPackFor && contentPackTypeMap[contentPackFor]) {
    return contentPackTypeMap[contentPackFor];
  }

  if (contentPackFor) {
    return {
      id: "content-pack",
      label: "内容包",
    };
  }

  if (mod.unique_id === "spacechase0.GenericModConfigMenu") {
    return {
      id: "generic-mod-config-menu",
      label: "GMCM",
    };
  }

  if (mod.entry_dll) {
    return {
      id: "smapi-plugin",
      label: "SMAPI 插件",
    };
  }

  return {
    id: "unknown",
    label: "未知类型",
  };
}

function getZipDependencyRows(mod: ZipModPreview): ZipDependencyRow[] {
  const dependencies = [
    ...(mod.content_pack_for ? [mod.content_pack_for] : []),
    ...(mod.dependencies || []),
  ];

  return dependencies
    .filter((dependency) => Boolean(dependency.unique_id))
    .map((dependency) => {
      const isInstalled = enabledModUniqueIds.value.has(dependency.unique_id);
      const isBundled = zipPreviewUniqueIds.value.has(dependency.unique_id);

      let statusLabel = "缺失";
      let className = dependency.is_required ? "bad" : "optional";

      if (isInstalled) {
        statusLabel = "已安装";
        className = "ok";
      } else if (isBundled) {
        statusLabel = "将随 ZIP 一起安装";
        className = "ok";
      } else if (!dependency.is_required) {
        statusLabel = "可选未安装";
      }

      return {
        uniqueId: dependency.unique_id,
        isRequired: dependency.is_required,
        isInstalled,
        isBundled,
        statusLabel,
        className,
      };
    });
}

function clearModFilters() {
  modSearchQuery.value = "";
  modStatusFilter.value = "all";
  modDependencyFilter.value = "all";
}

function createDisplayMod(mod: ModInfo, isDisabled: boolean): DisplayModInfo {
  return {
    ...mod,
    isDisabled,
    hasMissingRequiredDependency: modHasMissingRequiredDependency(mod),
    modType: detectModType(mod),
  };
}

function detectModType(mod: ModInfo): ModTypeInfo {
  const contentPackFor = mod.contentPackFor?.uniqueId || "";

  const contentPackTypeMap: Record<string, ModTypeInfo> = {
    "Pathoschild.ContentPatcher": {
      id: "content-patcher-pack",
      label: "CP 内容包",
    },
    "PeacefulEnd.FashionSense": {
      id: "fashion-sense-pack",
      label: "Fashion Sense",
    },
    "spacechase0.JsonAssets": {
      id: "json-assets-pack",
      label: "Json Assets",
    },
    "FlashShifter.StardewValleyExpandedCP": {
      id: "stardew-valley-expanded-pack",
      label: "SVE 内容包",
    },
  };

  if (contentPackFor && contentPackTypeMap[contentPackFor]) {
    return contentPackTypeMap[contentPackFor];
  }

  if (contentPackFor) {
    return {
      id: "content-pack",
      label: "内容包",
    };
  }

  if (mod.uniqueId === "spacechase0.GenericModConfigMenu") {
    return {
      id: "generic-mod-config-menu",
      label: "GMCM",
    };
  }

  if (mod.entryDll) {
    return {
      id: "smapi-plugin",
      label: "SMAPI 插件",
    };
  }

  return {
    id: "unknown",
    label: "未知类型",
  };
}

function modHasMissingRequiredDependency(mod: ModInfo): boolean {
  if (mod.contentPackFor && !mod.contentPackFor.isInstalled) {
    return true;
  }

  return mod.dependencies.some(
    (dependency) => dependency.isRequired && !dependency.isInstalled
  );
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

      const item = dependency as { UniqueID?: string; IsRequired?: boolean };

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

  const item = rawContentPackFor as { UniqueID?: string };

  if (!item.UniqueID) {
    return undefined;
  }

  return {
    uniqueId: item.UniqueID,
    isRequired: true,
    isInstalled: false,
  };
}

function attachDependencyStatus(modList: ModInfo[], installedSource: ModInfo[] = modList): ModInfo[] {
  const installedUniqueIds = new Set(
    installedSource.map((mod) => mod.uniqueId).filter(Boolean)
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
    .map(([uniqueId, requiredBy]) => ({ uniqueId, requiredBy }))
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
    entryDll: mod.entryDll,
    modType: detectModType(mod),
    contentPackFor: mod.contentPackFor,
    dependencies: mod.dependencies,
  };
}

function createProblemReportText(): string {
  const lines: string[] = [];

  lines.push("Junimo Box 问题报告");
  lines.push("=".repeat(40));
  lines.push(`导出时间：${new Date().toLocaleString()}`);
  lines.push("");

  lines.push("[游戏环境]");
  lines.push(`游戏路径：${gamePath.value || "未选择"}`);
  lines.push(`Stardew Valley：${stardewExists.value ? "已找到" : "未找到"}`);
  lines.push(`SMAPI：${smapiExists.value ? "已安装" : "未安装"}`);
  lines.push(`Mods 文件夹：${modsFolderExists.value ? "已找到" : "未找到"}`);
  lines.push("");

  lines.push("[Mod 统计]");
  lines.push(`已启用 Mods：${mods.value.length}`);
  lines.push(`已禁用 Mods：${disabledMods.value.length}`);
  lines.push(`缺失依赖：${missingDependencies.value.length}`);
  lines.push(`未识别文件夹：${skippedFolders.value.length}`);
  lines.push("");

  lines.push("[缺失依赖]");
  if (missingDependencies.value.length === 0) {
    lines.push("无");
  } else {
    for (const dependency of missingDependencies.value) {
      lines.push(`- ${dependency.uniqueId}`);
      lines.push(
        `  被 ${dependency.requiredBy.length} 个 Mod 需要：${dependency.requiredBy.join("、")}`
      );
    }
  }
  lines.push("");

  lines.push("[未识别文件夹]");
  if (skippedFolders.value.length === 0) {
    lines.push("无");
  } else {
    for (const folder of skippedFolders.value) {
      lines.push(`- ${folder}`);
    }
  }
  lines.push("");

  lines.push("[SMAPI 日志诊断]");
  if (!smapiLogAnalysis.value) {
    lines.push("未读取到 SMAPI 日志，或日志目录不存在。");
  } else {
    const analysis = smapiLogAnalysis.value;

    lines.push(`日志文件：${smapiLogFileName.value || "未知"}`);
    lines.push(`SMAPI 版本：${analysis.smapiVersion || "未识别"}`);
    lines.push(`游戏版本：${analysis.gameVersion || "未识别"}`);
    lines.push(`Mods 路径：${analysis.modsPath || "未识别"}`);
    lines.push(`警告数量：${analysis.warningLines.length}`);
    lines.push(`错误数量：${analysis.errorLines.length}`);
    lines.push("");

    lines.push("[诊断建议]");
    if (analysis.suggestions.length === 0) {
      lines.push("无");
    } else {
      for (const suggestion of analysis.suggestions) {
        lines.push(`- ${suggestion}`);
      }
    }
    lines.push("");

    lines.push("[受影响的游戏文件]");
    if (analysis.affectedAssets.length === 0) {
      lines.push("无");
    } else {
      for (const asset of analysis.affectedAssets) {
        lines.push(`- ${asset}`);
      }
    }
    lines.push("");

    lines.push("[被 SMAPI 跳过的 Mod]");
    if (analysis.skippedMods.length === 0) {
      lines.push("无");
    } else {
      for (const skippedMod of analysis.skippedMods) {
        lines.push(`- ${skippedMod.path}`);
        lines.push(`  原因：${skippedMod.reason}`);
      }
    }
    lines.push("");

    lines.push("[错误行]");
    lines.push(...(analysis.errorLines.length === 0 ? ["无"] : analysis.errorLines));
    lines.push("");

    lines.push("[警告行]");
    lines.push(...(analysis.warningLines.length === 0 ? ["无"] : analysis.warningLines));
  }

  lines.push("");
  lines.push("[已启用 Mods]");
  if (mods.value.length === 0) {
    lines.push("无");
  } else {
    for (const mod of mods.value) {
      lines.push(formatModForReport(mod));
    }
  }

  lines.push("");
  lines.push("[已禁用 Mods]");
  if (disabledMods.value.length === 0) {
    lines.push("无");
  } else {
    for (const mod of disabledMods.value) {
      lines.push(formatModForReport(mod));
    }
  }

  lines.push("");
  lines.push("=".repeat(40));
  lines.push("由 Junimo Box 生成");

  return lines.join("\n");
}

function formatModForReport(mod: ModInfo): string {
  const parts = [
    mod.name || "未知 Mod",
    mod.author ? `作者：${mod.author}` : "作者：未知",
    mod.version ? `版本：${mod.version}` : "版本：未知",
    mod.uniqueId ? `UniqueID：${mod.uniqueId}` : "UniqueID：未知",
    `类型：${detectModType(mod).label}`,
    `文件夹：${mod.folderName}`,
  ];

  return `- ${parts.join(" / ")}`;
}

function getFolderName(path: string): string {
  const parts = path.split("\\").filter(Boolean);
  return parts[parts.length - 1] || path;
}

function analyzeSmapiLog(content: string): SmapiLogAnalysis {
  const lines = content.split(/\r?\n/);

  const analysis: SmapiLogAnalysis = {
    smapiVersion: "",
    gameVersion: "",
    modsPath: "",
    warningLines: [],
    errorLines: [],
    skippedMods: [],
    affectedAssets: [],
    suggestions: [],
  };

  let isReadingAffectedAssets = false;

  for (const line of lines) {
    const trimmedLine = line.trim();

    const versionMatch = trimmedLine.match(
      /SMAPI\s+([^\s]+)\s+with Stardew Valley\s+([^\s]+)/
    );

    if (versionMatch) {
      analysis.smapiVersion = versionMatch[1];
      analysis.gameVersion = versionMatch[2];
    }

    const modsPathMatch = trimmedLine.match(/Mods go here:\s*(.+)$/);

    if (modsPathMatch) {
      analysis.modsPath = modsPathMatch[1];
    }

    if (trimmedLine.includes(" WARN ")) {
      analysis.warningLines.push(trimmedLine);
    }

    if (trimmedLine.includes(" ERROR ")) {
      analysis.errorLines.push(trimmedLine);
    }

    if (trimmedLine.includes("Affected assets:")) {
      isReadingAffectedAssets = true;
      continue;
    }

    if (isReadingAffectedAssets) {
      if (trimmedLine.startsWith("- ")) {
        analysis.affectedAssets.push(trimmedLine.replace(/^- /, ""));
        continue;
      }

      if (trimmedLine.length > 0) {
        isReadingAffectedAssets = false;
      }
    }

    const skippedMatch = trimmedLine.match(
      /Skipped\s+(.+?)(?:\s+\((.+)\)\.?$|\.?$)/
    );

    if (skippedMatch) {
      analysis.skippedMods.push({
        path: skippedMatch[1],
        reason: skippedMatch[2] || "SMAPI 跳过了这个文件夹。",
      });
    }
  }

  const hasModifiedContentWarning = analysis.warningLines.some((line) =>
    line.includes("content files were modified or corrupted")
  );

  if (hasModifiedContentWarning) {
    analysis.suggestions.push(
      "检测到游戏原始内容文件可能被修改或损坏。建议在 Steam 中验证《星露谷物语》的游戏文件完整性。"
    );
  }

  if (analysis.skippedMods.length > 0) {
    analysis.suggestions.push(
      "检测到有文件夹被 SMAPI 跳过。常见原因是文件夹名以点号开头、文件夹结构不正确，或这不是一个有效 Mod。"
    );
  }

  if (analysis.errorLines.length > 0) {
    analysis.suggestions.push(
      "检测到 ERROR 错误。建议优先查看错误行附近的 Mod 名称、缺失依赖或版本不兼容信息。"
    );
  }

  if (
    analysis.warningLines.length === 0 &&
    analysis.errorLines.length === 0 &&
    analysis.skippedMods.length === 0
  ) {
    analysis.suggestions.push("没有发现明显的 WARN、ERROR 或被跳过的 Mod。");
  }

  return analysis;
}
</script>

<style scoped>
.app-shell {
  width: 100vw;
  height: 100vh;
  overflow: hidden;
  display: grid;
  grid-template-columns: 218px minmax(0, 1fr) 270px;
  background:
    radial-gradient(circle at top left, rgba(132, 184, 95, 0.16), transparent 30%),
    #f5efe3;
  color: #2d241b;
  font-family:
    "Microsoft YaHei",
    system-ui,
    sans-serif;
}

.sidebar {
  height: 100%;
  padding: 16px 14px;
  box-sizing: border-box;
  background: linear-gradient(180deg, #5f432d, #3f2b1d);
  color: #fff7e8;
  display: flex;
  flex-direction: column;
  gap: 18px;
  border-right: 3px solid rgba(45, 36, 27, 0.2);
}

.brand {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 12px;
  border-radius: 17px;
  background: rgba(255, 250, 240, 0.1);
}

.brand-icon {
  width: 38px;
  height: 38px;
  display: grid;
  place-items: center;
  border-radius: 13px;
  background: #fffaf0;
  font-size: 22px;
}

.brand h1 {
  margin: 0;
  font-size: 19px;
  line-height: 1.05;
}

.brand p {
  margin: 4px 0 0;
  color: #e7d7be;
  font-size: 12px;
}

.nav {
  display: flex;
  flex-direction: column;
  gap: 7px;
}

.nav-button {
  width: 100%;
  padding: 11px 13px;
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
  border-radius: 15px;
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
  padding: 24px 28px;
  box-sizing: border-box;
}

.content-header {
  display: flex;
  justify-content: space-between;
  gap: 18px;
  align-items: flex-start;
  margin-bottom: 16px;
}

.header-actions {
  display: flex;
  gap: 10px;
  align-items: center;
}

.content-header h2 {
  margin: 3px 0 5px;
  font-size: 31px;
  line-height: 1.1;
}

.content-header p {
  margin: 0;
  color: #7a6652;
  line-height: 1.45;
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
  gap: 16px;
}

.notice,
.panel,
.empty-state {
  border-radius: 22px;
  background: rgba(255, 250, 240, 0.92);
  box-shadow: 0 10px 28px rgba(67, 47, 27, 0.09);
}

.notice {
  margin-bottom: 16px;
  padding: 13px 18px;
  color: #7a4f22;
  font-weight: 800;
}

.panel {
  padding: 20px;
}

.compact-panel {
  padding: 18px 20px;
}

.slim-panel {
  padding: 18px 20px;
}

.empty-state {
  padding: 32px;
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
  margin-bottom: 14px;
}

.panel-header h3 {
  margin: 0;
  font-size: 22px;
}

.panel-header span {
  color: #7a6652;
  font-weight: 800;
}

.sticky-panel-header {
  position: sticky;
  top: -24px;
  z-index: 1;
  padding: 4px 0 10px;
  background: rgba(255, 250, 240, 0.95);
}

.panel-actions {
  display: flex;
  align-items: center;
  gap: 10px;
}

.overview-grid {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 16px;
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
  padding: 13px 14px;
  border-radius: 15px;
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

.filter-panel {
  padding: 16px;
}

.filter-top-row {
  display: flex;
  gap: 12px;
  align-items: center;
}

.search-box {
  flex: 1;
  min-width: 0;
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 11px 13px;
  border-radius: 15px;
  background: #f6ead8;
}

.search-box input {
  width: 100%;
  border: none;
  outline: none;
  background: transparent;
  color: #2d241b;
  font-size: 14px;
}

.search-box input::placeholder {
  color: #9a8065;
}

.filter-row {
  display: flex;
  flex-wrap: wrap;
  gap: 10px 18px;
  margin-top: 12px;
}

.filter-group {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 8px;
}

.filter-label {
  color: #7a6652;
  font-size: 13px;
  font-weight: 800;
}

.filter-chip {
  padding: 7px 11px;
  border-radius: 999px;
  background: #eadcc8;
  color: #5c4630;
  font-size: 13px;
}

.filter-chip:hover,
.filter-chip.active {
  background: #8b6f47;
  color: #fffaf0;
}

.filter-result-text {
  margin: 12px 0 0;
  color: #7a6652;
  font-size: 13px;
}

.mods-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.compact-mods-list {
  gap: 10px;
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
  opacity: 0.74;
}

.mod-item.warning {
  background: #f8e7c8;
}

.mod-main {
  min-width: 0;
}

.mod-title-row {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 12px;
}

.mod-item h4 {
  margin: 0 0 6px;
  font-size: 18px;
}

.mod-badges {
  flex-shrink: 0;
  display: flex;
  flex-wrap: wrap;
  justify-content: flex-end;
  gap: 6px;
}

.status-badge {
  padding: 4px 8px;
  border-radius: 999px;
  font-size: 12px;
  font-weight: 800;
}

.enabled-badge {
  background: #dcefd4;
  color: #2f8f46;
}

.disabled-badge {
  background: #e5d6c2;
  color: #7a6652;
}

.missing-badge {
  background: #f1c8bc;
  color: #9f493c;
}

.type-badge {
  background: #e8dac0;
  color: #6f5636;
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

.mod-extra-row {
  display: flex;
  flex-wrap: wrap;
  gap: 8px 14px;
  margin-top: 8px;
  color: #7a6652;
  font-size: 12px;
}

.mod-actions {
  flex-shrink: 0;
  display: flex;
  flex-direction: column;
  align-items: flex-end;
  gap: 8px;
}

.mod-button-row {
  display: flex;
  gap: 7px;
}

.mod-folder {
  max-width: 175px;
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

.diagnosis-grid {
  display: grid;
  grid-template-columns: repeat(4, minmax(0, 1fr));
  gap: 12px;
}

.diagnosis-card {
  padding: 14px;
  border-radius: 16px;
  background: #f6ead8;
}

.diagnosis-card span {
  display: block;
  margin-bottom: 6px;
  color: #7a6652;
  font-size: 13px;
}

.diagnosis-card strong {
  font-size: 18px;
}

.diagnosis-section {
  margin-top: 18px;
  padding-top: 16px;
  border-top: 1px solid rgba(92, 70, 48, 0.14);
}

.diagnosis-section h4 {
  margin: 0 0 8px;
  font-size: 17px;
}

.diagnosis-section p {
  margin: 0 0 10px;
  color: #5c4630;
  line-height: 1.5;
}

.diagnosis-list {
  margin: 0;
  padding-left: 20px;
  color: #4b3a2a;
  line-height: 1.6;
}

.diagnosis-item {
  padding: 12px;
  border-radius: 12px;
  background: rgba(255, 250, 240, 0.72);
}

.diagnosis-item + .diagnosis-item {
  margin-top: 10px;
}

.diagnosis-item p {
  margin: 6px 0 0;
}

.warning-box {
  padding: 14px;
  border: none;
  border-radius: 14px;
  background: #f8e7c8;
}

.error-box {
  padding: 14px;
  border: none;
  border-radius: 14px;
  background: #f7dfd8;
}

.code-text {
  padding: 10px 12px;
  border-radius: 10px;
  background: #f6ead8;
  color: #5c4630;
  font-family: Consolas, "Courier New", monospace;
  word-break: break-all;
}

.small-log {
  max-height: 220px;
  overflow: auto;
  margin: 0;
  padding: 12px;
  border-radius: 12px;
  background: #2d241b;
  color: #fff7e8;
  font-family: Consolas, "Courier New", monospace;
  font-size: 12px;
  line-height: 1.5;
  white-space: pre-wrap;
}

.muted-text {
  margin: 0;
  color: #7a6652;
  line-height: 1.5;
}

.path-text {
  word-break: break-all;
}

.zip-dependency-summary {
  margin-top: 14px;
  padding: 14px;
  border-radius: 14px;
}

.zip-dependency-summary.is-ok {
  background: #e8f3df;
  color: #2f6f3c;
}

.zip-dependency-summary.has-warning {
  background: #f8e7c8;
  color: #7a4f22;
}

.zip-dependency-summary p {
  margin: 6px 0 0;
  line-height: 1.5;
}

.zip-missing-list {
  margin: 8px 0 0;
  padding-left: 18px;
  line-height: 1.6;
}

.zip-dependencies {
  margin-top: 10px;
  padding-top: 8px;
  border-top: 1px solid rgba(92, 70, 48, 0.16);
}

.dependency-title {
  margin: 0 0 5px;
  color: #5c4630;
  font-size: 13px;
  font-weight: 800;
}

.zip-drop-zone {
  margin-top: 14px;
  padding: 18px;
  border: 2px dashed rgba(111, 168, 95, 0.45);
  border-radius: 18px;
  background: rgba(232, 243, 223, 0.58);
  display: flex;
  align-items: center;
  gap: 14px;
  cursor: pointer;
  transition:
    border-color 0.18s ease,
    background 0.18s ease,
    transform 0.18s ease;
}

.zip-drop-zone:hover,
.zip-drop-zone.active {
  border-color: #6fa85f;
  background: #e8f3df;
  transform: translateY(-1px);
}

.zip-drop-icon {
  width: 46px;
  height: 46px;
  flex: 0 0 auto;
  display: grid;
  place-items: center;
  border-radius: 16px;
  background: #fffaf0;
  font-size: 25px;
}

.zip-drop-zone strong {
  display: block;
  margin-bottom: 4px;
  color: #2f6f3c;
  font-size: 16px;
}

.zip-drop-zone p {
  margin: 0;
  color: #6f5c48;
  font-size: 13px;
  line-height: 1.45;
}

.zip-preview-list {
  margin-top: 14px;
}

.install-result {
  background: #e8f3df;
}

.install-warning,
.install-success {
  margin-top: 14px;
  padding: 14px;
  border-radius: 14px;
}

.install-warning {
  background: #f8e7c8;
  color: #7a4f22;
}

.install-success {
  background: #e8f3df;
  color: #2f6f3c;
}

.install-warning p,
.install-success p {
  margin: 6px 0 0;
  line-height: 1.5;
}

.right-panel {
  height: 100%;
  padding: 16px 14px;
  box-sizing: border-box;
  overflow-y: auto;
  background: rgba(255, 250, 240, 0.62);
  border-left: 1px solid rgba(92, 70, 48, 0.14);
  display: flex;
  flex-direction: column;
  gap: 14px;
}

.launch-card,
.side-card {
  padding: 16px;
  border-radius: 20px;
  background: #fffaf0;
  box-shadow: 0 10px 26px rgba(67, 47, 27, 0.09);
}

.launch-card {
  display: grid;
  grid-template-columns: 44px 1fr;
  column-gap: 12px;
  align-items: center;
}

.launch-card .launch-button {
  grid-column: 1 / -1;
}

.junimo-badge {
  width: 44px;
  height: 44px;
  display: grid;
  place-items: center;
  border-radius: 15px;
  background: #e3f0d6;
  font-size: 25px;
}

.launch-card h3,
.side-card h4 {
  margin: 0 0 6px;
}

.launch-card p,
.path-card p {
  margin: 0;
  color: #7a6652;
  font-size: 13px;
  line-height: 1.45;
  word-break: break-all;
}

.launch-button {
  width: 100%;
  margin-top: 14px;
  padding: 13px 16px;
  border-radius: 15px;
  background: #6fa85f;
  font-size: 16px;
  font-weight: 800;
}

.vanilla-button {
  margin-top: 8px;
  background: #8b6f47;
}

.vanilla-button:hover:not(:disabled) {
  background: #755d3c;
}

.info-line {
  display: flex;
  justify-content: space-between;
  gap: 10px;
  padding: 9px 0;
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

.compact-header-button {
  white-space: nowrap;
  padding: 10px 15px;
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
    grid-template-columns: 205px minmax(0, 1fr);
  }

  .right-panel {
    display: none;
  }
}

@media (max-width: 820px) {
  .overview-grid,
  .status-grid,
  .summary-row,
  .tool-grid,
  .diagnosis-grid {
    grid-template-columns: 1fr;
  }

  .filter-top-row {
    flex-direction: column;
    align-items: stretch;
  }

  .mod-item {
    flex-direction: column;
  }

  .mod-actions {
    align-items: flex-start;
  }
}
</style>
