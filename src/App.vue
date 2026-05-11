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
        <button
          class="sidebar-launch-button"
          :disabled="!smapiExists"
          @click="handleLaunchSmapi"
        >
          启动游戏
        </button>
      </div>
    </aside>

    <section
      class="content"
      :class="{ 'content-fixed': activeView === 'mods' }"
    >
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
            :disabled="isScanning"
            @click="scanMods"
          >
            {{ isScanning ? "扫描中..." : "重新扫描" }}
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

      <div
        v-if="notice"
        class="notice"
        :class="`notice-${notice.type}`"
      >
        <span class="notice-icon">{{ noticeIcon }}</span>
        <span>{{ notice.text }}</span>
      </div>

      <div class="toast-container">
        <div
          v-for="toast in toasts"
          :key="toast.id"
          class="toast-item"
          :class="'toast-' + toast.type"
        >
          <span class="toast-text">{{ toast.text }}</span>
          <button
            v-if="toast.action"
            class="toast-undo"
            @click="toast.action.handler(); removeToast(toast.id)"
          >
            {{ toast.action.label }}
          </button>
          <button class="toast-close" @click="removeToast(toast.id)">&#10005;</button>
        </div>
      </div>

      <section v-if="activeView === 'overview'" class="view-stack">
        <div class="overview-hero panel">
          <div class="overview-hero-copy">
            <p class="eyebrow">Junimo Box</p>
            <h3>像素风的星露谷 Mod 小屋</h3>
            <p>
              把目录、安装、更新和启动收进一个温暖清楚的第三方管理器里，让玩家少折腾文件夹。
            </p>

            <div class="hero-chip-row">
              <span>{{ gamePath ? "游戏目录已就绪" : "先选择游戏目录" }}</span>
              <span>{{ smapiExists ? "SMAPI 可启动" : "SMAPI 待安装" }}</span>
              <span>{{ totalModCount }} 个 Mod</span>
              <span>{{ currentProfile?.name || "默认配置" }}</span>
            </div>

            <div class="hero-action-row">
              <button class="hero-action-button" @click="handleSelectPath">
                选择游戏目录
              </button>
              <button class="hero-action-button secondary" :disabled="!gamePath || isScanning" @click="scanMods">
                重新扫描
              </button>
              <button class="hero-action-button" :disabled="!smapiExists" @click="handleLaunchSmapi">
                启动游戏
              </button>
            </div>
          </div>

          <div class="overview-hero-scene" aria-hidden="true">
            <div class="scene-sky"></div>
            <div class="scene-mountains"></div>
            <div class="scene-field"></div>
            <div class="scene-house">
              <span class="scene-roof"></span>
              <span class="scene-wall"></span>
              <span class="scene-door"></span>
              <span class="scene-window left"></span>
              <span class="scene-window right"></span>
            </div>
            <div class="scene-tree scene-tree-left"></div>
            <div class="scene-tree scene-tree-right"></div>
            <div class="scene-crate"></div>
            <div class="scene-junimo"></div>
          </div>
        </div>

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
                <small v-if="smapiExists">版本：{{ smapiDetectedVersion || "未识别" }}</small>
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

      <section v-if="activeView === 'mods'" class="view-stack mods-view mods-page-fixed">
        <div class="panel filter-panel">
          <div class="filter-top-row">
            <div class="search-box">
              <span>🔎</span>
              <input
                v-model="modSearchQuery"
                type="text"
                placeholder="搜索 Mod 名称、作者、UniqueID、文件夹或描述..."
                @keydown.escape="modSearchQuery = ''"
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

          <div class="filter-summary-row">
            <p class="filter-result-text">
              当前显示 {{ filteredMods.length }} / {{ allDisplayMods.length }} 个 Mod。
            </p>
            <p v-if="modSearchQuery || modStatusFilter !== 'all' || modDependencyFilter !== 'all'" class="active-filter-text">
              筛选已启用
            </p>
          </div>

          <div v-if="lastInstalledZipMods.length > 0" class="inline-install-summary">
            <span>最近安装：</span>
            <strong>
              {{
                lastInstalledZipMods
                  .slice(0, 3)
                  .map((mod) => mod.name || mod.suggested_folder)
                  .join("、")
              }}
            </strong>
            <span v-if="lastInstalledZipMods.length > 3">
              等 {{ lastInstalledZipMods.length }} 个 Mod
            </span>
          </div>

          <div v-if="mods.length > 0 && missingDependencies.length > 0" class="inline-dependency-summary">
            <span>依赖提醒：</span>
            <strong>{{ missingDependencies.length }} 项缺失依赖</strong>
            <span>，请在相关 Mod 详情中查看。</span>
          </div>
        </div>

        <div v-if="selectedModKeys.size > 0" class="batch-action-bar">
          <span class="batch-count">已选择 {{ selectedModKeys.size }} 个 Mod</span>
          <div class="batch-actions">
            <button class="tiny-button" @click="handleBatchEnable">批量启用</button>
            <button class="tiny-button danger" @click="handleBatchDisable">批量禁用</button>
            <button class="tiny-button danger delete-mod-button" @click="handleBatchDelete">批量删除</button>
            <button class="tiny-button ghost-button" @click="selectedModKeys = new Set()">取消选择</button>
          </div>
        </div>

        <div v-if="filteredMods.length > 0" class="mods-workspace">
          <div class="panel mods-list-panel">
            <div class="panel-header sticky-panel-header">
              <div>
                <h3>Mod 列表</h3>
                <p class="detail-subtitle">
                  点击列表项打开独立详情卡片。
                </p>
              </div>
              <span>{{ filteredMods.length }} 个</span>
            </div>

            <div class="mods-list compact-mods-list scrollable-mods-list">
              <article
                v-for="mod in filteredMods"
                :key="getModKey(mod)"
                class="mod-item selectable-mod-item compact-mod-card"
                :class="{
                  disabled: mod.isDisabled,
                  warning: mod.hasMissingRequiredDependency,
                  selected: isSelectedMod(mod),
                }"
                @click="selectMod(mod, $event)"
              >
                <div class="mod-card-content">
                  <div class="mod-card-main-row">
                    <div class="mod-title-block">
                      <h4><span v-html="highlightText(mod.name, modSearchQuery)"></span></h4>
                      <p class="mod-meta">
                        <span v-html="highlightText(mod.author || '未知作者', modSearchQuery)"></span> · v{{ mod.version || "未知版本" }}
                      </p>
                    </div>

                    <div class="mod-badges compact-badges">
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

                  <p class="mod-description compact-description">
                    {{ mod.description || "没有描述。" }}
                  </p>

                  <div class="mod-card-footer">
                    <span class="folder-chip" v-html="highlightText(mod.folderName, modSearchQuery)"></span>

                    <div class="mod-actions compact-card-actions">
                      <button class="tiny-button ghost-button" @click.stop="handleOpenDisplayedModFolder(mod)">
                        打开
                      </button>

                      <button
                        v-if="mod.isDisabled"
                        class="tiny-button"
                        @click.stop="handleEnableMod(mod.folderName)"
                      >
                        启用
                      </button>

                      <button
                        v-else
                        class="tiny-button danger"
                        @click.stop="handleDisableMod(mod.folderName)"
                      >
                        禁用
                      </button>

                      <button
                        class="tiny-button danger delete-mod-button"
                        @click.stop="handleDeleteDisplayedMod(mod)"
                      >
                        删除
                      </button>
                    </div>
                  </div>
                </div>
              </article>
            </div>
          </div>


        </div>

        <div
          v-if="selectedMod"
          class="mod-detail-overlay"
          @click.self="closeModDetail"
        >
          <section class="mod-detail-card">
            <div class="mod-detail-card-header">
              <div>
                <p class="eyebrow">Mod Detail</p>
                <h3>{{ selectedMod.name }}</h3>
                <p>{{ selectedMod.modType.label }} · {{ selectedMod.isDisabled ? "已禁用" : "已启用" }}</p>
              </div>

              <button class="tiny-button" @click="closeModDetail">
                关闭
              </button>
            </div>

            <div class="mod-badges detail-badges">
              <span class="status-badge" :class="selectedMod.isDisabled ? 'disabled-badge' : 'enabled-badge'">
                {{ selectedMod.isDisabled ? "已禁用" : "已启用" }}
              </span>
              <span class="status-badge type-badge">{{ selectedMod.modType.label }}</span>
              <span v-if="selectedMod.hasMissingRequiredDependency" class="status-badge missing-badge">
                缺失依赖
              </span>
            </div>

            <p class="detail-description">
              {{ selectedMod.description || "没有描述。" }}
            </p>

            <div class="detail-actions detail-actions-inline">
              <button class="tiny-button" @click="handleOpenDisplayedModFolder(selectedMod)">
                打开文件夹
              </button>

              <button
                v-if="selectedMod.isDisabled"
                class="tiny-button"
                @click="handleEnableMod(selectedMod.folderName)"
              >
                启用 Mod
              </button>

              <button
                v-else
                class="tiny-button danger"
                @click="handleDisableMod(selectedMod.folderName)"
              >
                禁用 Mod
              </button>

              <button
                class="tiny-button danger delete-mod-button"
                @click="handleDeleteDisplayedMod(selectedMod)"
              >
                删除 Mod
              </button>
            </div>

            <div class="detail-grid detail-card-grid">
              <div>
                <span>作者</span>
                <strong>{{ selectedMod.author || "未知作者" }}</strong>
              </div>
              <div>
                <span>版本</span>
                <strong>{{ selectedMod.version || "未知版本" }}</strong>
              </div>
              <div>
                <span>UniqueID</span>
                <strong>{{ selectedMod.uniqueId || "未提供" }}</strong>
              </div>
              <div>
                <span>文件夹</span>
                <strong>{{ selectedMod.folderName }}</strong>
              </div>
              <div>
                <span>EntryDll</span>
                <strong>{{ selectedMod.entryDll || "无" }}</strong>
              </div>
              <div>
                <span>当前状态</span>
                <strong :class="selectedMod.isDisabled ? 'optional' : 'ok'">
                  {{ selectedMod.isDisabled ? "已禁用" : "已启用" }}
                </strong>
              </div>
            </div>

            <div class="detail-dependencies">
              <h4>依赖关系</h4>

              <p v-if="!selectedMod.contentPackFor && selectedMod.dependencies.length === 0" class="muted-text">
                这个 Mod 没有声明依赖。
              </p>

              <div v-else class="dependency-detail-list">
                <div v-if="selectedMod.contentPackFor" class="dependency-detail-item">
                  <span>内容包依赖</span>
                  <strong :class="selectedMod.contentPackFor.isInstalled ? 'ok' : 'bad'">
                    {{ selectedMod.contentPackFor.uniqueId }}
                    {{ selectedMod.contentPackFor.isInstalled ? "已安装" : "缺失" }}
                  </strong>
                </div>

                <div
                  v-for="dependency in selectedMod.dependencies"
                  :key="dependency.uniqueId"
                  class="dependency-detail-item"
                >
                  <span>{{ dependency.isRequired ? "必需依赖" : "可选依赖" }}</span>
                  <strong
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
                  </strong>
                </div>
              </div>
            </div>
          </section>
        </div>

        <div v-if="gamePath && allDisplayMods.length > 0 && filteredMods.length === 0" class="empty-state">
          <h3>没有符合条件的 Mod</h3>
          <p>试试清空搜索词，或者切换筛选条件。</p>
          <button class="tiny-button" @click="clearModFilters" style="margin-top:8px">清空筛选</button>
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
          <p>点击下方按钮扫描 Mods 文件夹。</p>
          <button class="tool-action-button" :disabled="isScanning" @click="scanMods" style="margin-top:12px">
            {{ isScanning ? "扫描中..." : "扫描 Mods 文件夹" }}
          </button>
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
          <p>点击"读取最新日志"，Junimo Box 会读取最近一次 SMAPI 日志并生成诊断摘要。</p>
        </div>
      </section>

      <section v-if="activeView === 'tools'" class="view-stack toolbox-workspace">
        <div class="toolbox-section-block">
          <div class="toolbox-section-title-row">
            <div>
              <h3>常用操作</h3>
              <p>目录、导出和基础维护入口集中放在这里。</p>
            </div>
          </div>

          <div class="toolbox-grid">
            <article class="panel tool-section-card compact-tool-card">
              <div class="tool-section-header">
                <div class="tool-section-icon">📁</div>
                <div>
                  <h3>文件夹</h3>
                  <p>快速打开常用目录，方便手动检查文件。</p>
                </div>
              </div>

              <div class="tool-section-actions">
                <button class="tool-action-button" @click="handleOpenGameFolder">
                  打开游戏目录
                </button>

                <button
                  class="tool-action-button"
                  :disabled="!modsFolderExists"
                  @click="handleOpenModsFolder"
                >
                  打开 Mods 文件夹
                </button>

                <button class="tool-action-button" @click="handleOpenSmapiLogFolder">
                  打开日志文件夹
                </button>
              </div>
            </article>

            <article class="panel tool-section-card compact-tool-card">
              <div class="tool-section-header">
                <div class="tool-section-icon">📤</div>
                <div>
                  <h3>导出与报告</h3>
                  <p>导出 Mod 列表或问题报告，方便备份和求助。</p>
                </div>
              </div>

              <div class="tool-section-actions">
                <button
                  class="tool-action-button"
                  :disabled="mods.length === 0 && disabledMods.length === 0"
                  @click="handleExportModList"
                >
                  导出 Mod 列表
                </button>

                <button
                  class="tool-action-button"
                  :disabled="!gamePath"
                  @click="handleExportProblemReport"
                >
                  导出问题报告
                </button>
              </div>
            </article>
            <article class="panel tool-section-card compact-tool-card">
              <div class="tool-section-header">
                <div class="tool-section-icon">♻️</div>
                <div>
                  <h3>回收站</h3>
                  <p>管理已删除的 Mod，可还原或永久删除。</p>
                </div>
              </div>

              <div class="tool-section-actions">
                <button
                  class="tool-action-button"
                  :disabled="!gamePath || isRecycleBinLoading"
                  @click="handleListDeletedMods"
                >
                  {{ isRecycleBinLoading ? "扫描中..." : "扫描回收站" }}
                </button>
                <button
                  class="tool-action-button"
                  :disabled="deletedMods.length === 0"
                  @click="handleEmptyRecycleBin"
                >
                  清空回收站 ({{ deletedMods.length }})
                </button>
              </div>

              <div v-if="isRecycleBinLoading" class="rb-empty">
                正在扫描回收站...
              </div>

              <div v-else-if="deletedMods.length > 0" class="recycle-bin-list">
                <div
                  v-for="item in deletedMods"
                  :key="item.folder_name"
                  class="recycle-bin-row"
                >
                  <div class="rb-info">
                    <strong>{{ item.original_name || item.folder_name }}</strong>
                    <span class="rb-time">{{ item.deleted_at || "未知时间" }}</span>
                  </div>
                  <div class="rb-actions">
                    <button
                      class="tiny-button"
                      :disabled="isRestoringMap[item.folder_name]"
                      @click="handleRestoreDeletedMod(item.folder_name)"
                    >
                      {{ isRestoringMap[item.folder_name] ? "还原中..." : "还原" }}
                    </button>
                    <button
                      class="tiny-button danger"
                      @click="handlePermanentlyDeleteMod(item.folder_name)"
                    >
                      删除
                    </button>
                  </div>
                </div>
              </div>

              <div v-else class="rb-empty">
                点击"扫描回收站"查看已删除的 Mod
              </div>
            </article>
          </div>
        </div>

        <article class="panel tool-section-card toolbox-full-card">
          <div class="tool-section-header">
            <div class="tool-section-icon">💾</div>
            <div>
              <h3>备份与恢复</h3>
              <p>备份当前 Mod 启用状态（已启用/已禁用），还原时自动调整 Mods 文件夹状态。</p>
            </div>
          </div>

          <div class="tool-section-actions">
            <button
              class="tool-action-button"
              :disabled="!gamePath || totalModCount === 0"
              @click="handleExportBackup"
            >
              导出备份
            </button>
            <button
              class="tool-action-button"
              :disabled="!gamePath"
              @click="handleImportBackup"
            >
              还原备份
            </button>
          </div>
        </article>

        <article class="panel tool-section-card smapi-tool-card toolbox-full-card">
          <div class="tool-section-header">
            <div class="tool-section-icon">🧩</div>
            <div>
              <h3>SMAPI 管理</h3>
              <p>自动下载最新 SMAPI，并打开官方 Windows 安装器完成安装。</p>
            </div>
          </div>

          <div class="tool-status-row">
            <span>当前状态</span>
            <strong :class="smapiExists ? 'ok' : 'bad'">
              {{ smapiExists ? "已安装" : "未安装" }}
            </strong>
          </div>

          <div v-if="smapiExists" class="tool-status-row">
            <span>SMAPI 版本</span>
            <strong>{{ smapiDetectedVersion || "未识别" }}</strong>
          </div>

          <div class="tool-section-actions">
            <button
              class="tool-action-button"
              :disabled="!gamePath || !stardewExists || isSmapiInstalling"
              @click="handleInstallSmapi"
            >
              {{ isSmapiInstalling ? smapiInstallStageMessage || "正在安装 SMAPI..." : smapiExists ? "更新 / 重装 SMAPI" : "下载并安装 SMAPI" }}
            </button>

            <button
              v-if="smapiInstallerOpened"
              class="tool-action-button secondary-action"
              :disabled="isSmapiInstalling"
              @click="handleRecheckSmapiInstall"
            >
              我已完成安装，重新检测
            </button>
          </div>

          <p v-if="isSmapiInstalling" class="tool-section-note smapi-install-stage-text">
            {{ smapiInstallStageMessage || "正在准备 SMAPI 安装..." }}
          </p>

          <p v-if="smapiInstallerOpened && !isSmapiInstalling" class="tool-section-note smapi-install-stage-text">
            SMAPI {{ smapiInstallerVersion || "" }} 安装器已打开。请按官方安装器提示完成安装，完成后点击"我已完成安装，重新检测"。
          </p>

          <p class="tool-section-note">
            Junimo Box 会下载 SMAPI 最新安装包，解压后运行官方 install on Windows.bat。
            安装过程仍以 SMAPI 官方安装器为准。
          </p>
        </article>

        <article class="panel tool-section-card zip-tool-card toolbox-full-card">
          <div class="tool-section-header">
            <div class="tool-section-icon">📦</div>
            <div>
              <h3>安装 ZIP Mod</h3>
              <p>所有外部来源最终都进入 ZIP 预览和依赖检查，确认后再安装到 Mods 文件夹。</p>
            </div>
          </div>

          <div class="toolbox-install-actions">
            <div
              class="zip-drop-zone compact-drop-zone"
              :class="{ active: isZipDragOver }"
              @click="handlePreviewZipMod"
            >
              <div class="zip-drop-icon">＋</div>
              <div>
                <strong>拖拽 ZIP Mod 到这里</strong>
                <p>拖入 .zip 后会自动生成安装预览，不会直接安装。</p>
              </div>
            </div>

            <div class="zip-tool-actions compact-zip-actions">
              <button @click="handlePreviewZipMod">
                选择 ZIP 文件
              </button>

              <span class="zip-tool-hint">
                支持安装前依赖检查、链接下载、NXM 接收和临时目录安全解压。
              </span>
            </div>
          </div>

          <div class="url-zip-box">
            <div class="url-zip-header">
              <strong>从链接安装 ZIP</strong>
              <span>支持直接 .zip 下载链接</span>
            </div>

            <div class="url-zip-form">
              <input
                v-model="urlZipInput"
                class="url-zip-input"
                type="text"
                placeholder="粘贴 Mod ZIP 下载链接，例如 https://.../mod.zip"
                @keydown.enter="handleDownloadZipFromUrl"
              />

              <button
                class="url-zip-button"
                :disabled="!gamePath || !urlZipInput.trim()"
                @click="handleDownloadZipFromUrl"
              >
                下载并预览
              </button>
            </div>
          </div>

          <div class="nxm-box">
            <div class="url-zip-header">
              <strong>Nexus NXM 下载入口</strong>
              <span>接收 Nexus 的 Mod Manager Download，并自动下载后进入 ZIP 预览</span>
            </div>

            <div class="nxm-action-row">
              <button class="url-zip-button" @click="handleRegisterNxmProtocol">
                关联 NXM 协议
              </button>

              <button class="secondary" @click="handleChooseDownloadedZipForNxm">
                选择已下载 ZIP
              </button>
            </div>

            <p v-if="nxmProtocolStatus" class="tool-section-note">
              {{ nxmProtocolStatus }}
            </p>

            <div class="url-zip-form nxm-manual-form">
              <input
                v-model="nxmManualInput"
                class="url-zip-input"
                type="text"
                placeholder="也可以手动粘贴 nxm:// 链接测试下载"
                @keydown.enter="handleParseManualNxm"
              />

              <button
                class="url-zip-button"
                :disabled="!nxmManualInput.trim()"
                @click="handleParseManualNxm"
              >
                解析 / 下载
              </button>
            </div>
          </div>
        </article>

        <article class="panel tool-section-card update-check-panel toolbox-result-panel">
          <div class="tool-result-header">
            <div class="tool-section-header compact-result-header">
              <div class="tool-section-icon">🔎</div>
              <div>
                <h3>更新检测</h3>
                <p>读取 manifest.json 里的 UpdateKeys，只提供可打开的更新来源，不自动下载。</p>
              </div>
            </div>

            <button
              class="tool-action-button result-action-button"
              :disabled="totalModCount === 0"
              @click="handleRunUpdateCheck"
            >
              检查更新来源
            </button>
          </div>

          <p v-if="updateCheckResults.length > 0" class="tool-section-note update-check-summary">
            已检查 {{ updateCheckResults.length }} 个 Mod，{{ updateCheckResults.filter((item) => item.sourceUrl).length }} 个提供可打开的更新来源。
          </p>

          <div v-if="updateCheckResults.length > 0" class="update-result-table">
            <article
              v-for="item in updateCheckResults"
              :key="item.key"
              class="update-result-row"
            >
              <div class="update-result-main">
                <strong>{{ item.name }}</strong>
                <span>当前版本：{{ item.version }} · {{ item.statusLabel }}</span>
              </div>
              <button class="tiny-button" :disabled="!item.sourceUrl" @click="handleOpenUpdateSource(item)">
                打开来源
              </button>
            </article>
          </div>

          <p v-else class="tool-section-note">
            点击检查后，会在这里以全宽列表显示提供 Nexus / ModDrop / GitHub 更新来源的 Mod，不再挤在半宽卡片里。
          </p>
        </article>

        <div v-if="recentInstallHistory.length === 0" class="tool-inline-state">
          <div>
            <strong>安装历史</strong>
            <span>暂无安装历史。完成本地 ZIP、URL ZIP 或 NXM 安装后会显示在这里。</span>
          </div>
          <button class="tiny-button" @click="handleInstallHistoryZipSelect">选择 ZIP 安装</button>
        </div>

        <article v-else class="panel tool-section-card install-history-panel toolbox-result-panel">
          <div class="tool-result-header">
            <div class="tool-section-header compact-result-header">
              <div class="tool-section-icon">🧾</div>
              <div>
                <h3>安装历史</h3>
                <p>记录最近通过本地 ZIP、URL ZIP 或 NXM 处理的安装。</p>
              </div>
            </div>

            <div class="history-header-actions">
              <button class="tiny-button" @click="handleInstallHistoryZipSelect">选择 ZIP 安装</button>
              <button class="tiny-button secondary" :disabled="installHistory.length === 0" @click="handleExportInstallHistory">导出历史</button>
              <button class="tiny-button secondary" :disabled="installHistory.length === 0" @click="clearInstallHistory">清空历史</button>
            </div>
          </div>

          <div class="history-result-list">
            <article
              v-for="item in recentInstallHistory"
              :key="item.id"
              class="history-result-row"
            >
              <div>
                <strong>{{ item.mods.map((mod) => mod.name).join('、') || '未知 Mod' }}</strong>
                <p>{{ item.sourceLabel }} · {{ item.note }} · {{ formatDateTime(item.installedAt) }}</p>
              </div>
            </article>
          </div>
        </article>

        <div
          v-if="zipModPreviews.length > 0"
          class="zip-preview-overlay"
          @click.self="closeZipPreview"
        >
          <section class="zip-preview-card">
            <div class="zip-preview-card-header">
              <div>
                <p class="eyebrow">ZIP Installer</p>
                <h3>ZIP Mod 安装预览</h3>
                <p>已检测到 {{ zipModPreviews.length }} 个 Mod。确认依赖状态后再安装到 Mods 文件夹。</p>
              </div>

              <button class="tiny-button" @click="closeZipPreview">
                关闭
              </button>
            </div>

            <p class="muted-text path-text zip-card-path">当前压缩包：{{ selectedZipPath }}</p>

            <div
              v-if="hasZipInstallConflicts"
              class="zip-conflict-summary"
            >
              <strong>检测到 {{ zipInstallConflicts.length }} 个已安装 Mod</strong>
              <p>这些目标文件夹已经存在。可以取消、跳过已有 Mod，或使用安全替换 / 更新。</p>
            </div>

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

            <div class="zip-card-scroll">
              <article
                v-for="mod in zipModPreviews"
                :key="mod.unique_id || mod.manifest_path"
                class="zip-preview-item"
              >
                <div class="zip-preview-item-main">
                  <div class="zip-preview-title-row">
                    <h4>{{ mod.name }}</h4>
                    <span class="mod-type">{{ getZipModType(mod).label }}</span>
                  </div>

                  <p class="mod-meta">
                    {{ mod.author || "未知作者" }} · v{{ mod.version || "未知版本" }}
                  </p>

                  <div
                    v-if="getZipConflictForPreview(mod)"
                    class="zip-conflict-line"
                  >
                    已安装：{{ getZipConflictForPreview(mod)?.installedMod.name }}
                    v{{ getZipConflictForPreview(mod)?.installedMod.version || "未知版本" }}，
                    准备安装 v{{ mod.version || "未知版本" }}
                    <span
                      :class="'version-diff-tag ' + getVersionDiffClass(
                        compareVersions(
                          mod.version || '',
                          getZipConflictForPreview(mod)?.installedMod.version || ''
                        )
                      )"
                    >
                      {{ getVersionDiffLabel(
                        compareVersions(
                          mod.version || '',
                          getZipConflictForPreview(mod)?.installedMod.version || ''
                        )
                      ) }}
                    </span>
                  </div>

                  <p class="mod-description compact-description">
                    {{ mod.description || "没有描述。" }}
                  </p>

                  <div class="zip-preview-meta-grid">
                    <div>
                      <span>UniqueID</span>
                      <strong>{{ mod.unique_id || "未提供" }}</strong>
                    </div>
                    <div>
                      <span>目标文件夹</span>
                      <strong>{{ mod.suggested_folder }}</strong>
                    </div>
                    <div class="wide">
                      <span>manifest</span>
                      <strong>{{ mod.manifest_path }}</strong>
                    </div>
                  </div>

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
              </article>
            </div>

            <div class="zip-preview-footer">
              <button class="secondary" @click="closeZipPreview">
                取消
              </button>

              <button
                v-if="!hasZipInstallConflicts"
                :disabled="!gamePath"
                @click="handleInstallZipMod('cancel')"
              >
                安装到 Mods
              </button>

              <button
                v-if="hasZipInstallConflicts && installableZipModCount > 0"
                :disabled="!gamePath"
                @click="handleInstallZipMod('skip')"
              >
                跳过已有并安装新 Mod
              </button>

              <button
                v-if="hasZipInstallConflicts"
                :disabled="!gamePath"
                @click="handleInstallZipMod('replace')"
              >
                替换 / 更新
              </button>
            </div>
          </section>
        </div>
      </section>




        <div
          v-if="nxmRequestLink"
          class="zip-preview-overlay"
          @click.self="closeNxmRequest"
        >
          <section class="zip-preview-card nxm-request-card">
            <div class="zip-preview-card-header">
              <div>
                <p class="eyebrow">Nexus NXM</p>
                <h3>来自 Nexus 的下载请求</h3>
                <p>Junimo Box 已接收到 nxm:// 链接。现在会优先自动下载，下载完成后直接进入 ZIP 预览。</p>
              </div>

              <button class="tiny-button" @click="closeNxmRequest">
                关闭
              </button>
            </div>

            <div class="nxm-detail-grid">
              <div>
                <span>游戏</span>
                <strong>{{ parsedNxmRequest.gameDomain || "未识别" }}</strong>
              </div>

              <div>
                <span>Mod ID</span>
                <strong>{{ parsedNxmRequest.modId || "未识别" }}</strong>
              </div>

              <div>
                <span>File ID</span>
                <strong>{{ parsedNxmRequest.fileId || "未识别" }}</strong>
              </div>
            </div>

            <p class="muted-text path-text">
              原始链接：{{ nxmRequestLink }}
            </p>

            <div class="zip-dependency-summary" :class="isNxmDownloading ? 'has-warning' : 'has-info'">
              <strong>{{ isNxmDownloading ? "正在下载" : "下载说明" }}</strong>
              <p>
                {{ nxmDownloadMessage || "点击「自动下载并预览」后，Junimo Box 会使用这个 NXM 链接中的下载参数获取 ZIP。失败时仍可打开 Nexus 页面或选择已下载 ZIP。" }}
              </p>
            </div>

            <div class="zip-preview-footer">
              <button class="secondary" @click="closeNxmRequest">
                关闭
              </button>

              <button
                class="secondary"
                :disabled="!parsedNxmRequest.nexusPageUrl || isNxmDownloading"
                @click="handleOpenNxmNexusPage"
              >
                打开 Nexus 页面
              </button>

              <button
                class="secondary"
                :disabled="isNxmDownloading"
                @click="handleChooseDownloadedZipForNxm"
              >
                选择已下载 ZIP
              </button>

              <button
                :disabled="isNxmDownloading || !parsedNxmRequest.modId || !parsedNxmRequest.fileId"
                @click="handleDownloadNxmRequest"
              >
                {{ isNxmDownloading ? "正在下载..." : "自动下载并预览" }}
              </button>
            </div>
          </section>
        </div>

      <section v-if="activeView === 'profiles'" class="view-stack profiles-page">
        <div class="panel compact-panel profile-hero-panel">
          <div class="profile-hero-main">
            <div>
              <p class="eyebrow">Profiles</p>
              <h3>配置方案</h3>
              <p class="muted-text">
                配置方案会保存一组要启用的 Mod。当前版本适合快速创建和测试不同组合。
              </p>
            </div>

            <div class="profile-hero-stats">
              <span>{{ profiles.length }} 个配置</span>
              <span>{{ totalModCount }} 个可选 Mod</span>
            </div>
          </div>

          <div class="profile-action-cards">
            <button class="profile-action-card primary" @click="startCreateProfile(false)">
              <strong>＋ 新建配置</strong>
              <span>手动勾选要启用的 Mod</span>
            </button>

            <button
              class="profile-action-card"
              :disabled="mods.length === 0"
              @click="startCreateProfile(true)"
            >
              <strong>基于当前启用创建</strong>
              <span>先带入当前已启用的 {{ mods.length }} 个 Mod</span>
            </button>

            <button
              class="profile-action-card"
              :disabled="profiles.length === 0"
              @click="handleExportAllProfiles"
            >
              <strong>导出全部配置</strong>
              <span>保存为 Junimo Box 配置 JSON</span>
            </button>

            <button class="profile-action-card" @click="handleImportProfiles">
              <strong>导入配置</strong>
              <span>从 JSON 文件导入一个或多个配置</span>
            </button>
          </div>
        </div>

        <div
          v-if="isProfileEditorOpen"
          class="profile-card-overlay"
          @click.self="closeProfileEditor"
        >
          <section class="profile-editor-card compact-profile-editor">
            <div class="profile-card-header">
              <div>
                <p class="eyebrow">Profile Editor</p>
                <h3>{{ profileEditorMode === 'edit' ? '编辑配置' : '新建配置' }}</h3>
                <p>
                  勾选此配置要启用的 Mod。应用配置时，其余 Mod 会移动到 Disabled Mods。
                </p>
              </div>

              <button class="tiny-button" @click="closeProfileEditor">
                关闭
              </button>
            </div>

            <div class="profile-editor-grid">
              <label class="profile-field">
                <span>配置名称</span>
                <input
                  v-model="profileDraftName"
                  class="profile-input"
                  placeholder="例如：日常游玩 / SVE / 多人联机"
                />
              </label>

              <label class="profile-field">
                <span>搜索 Mod</span>
                <input
                  v-model="profileDraftSearchQuery"
                  class="profile-input"
                  placeholder="按名称、作者、文件夹或类型搜索"
                />
              </label>
            </div>

            <div class="profile-editor-summary compact-editor-summary">
              <span>已选择 {{ profileDraftEnabledFolders.length }} / {{ profileSelectableMods.length }} 个 Mod</span>

              <div class="profile-editor-actions">
                <button class="tiny-button" @click="selectAllProfileMods">
                  全选
                </button>
                <button class="tiny-button" @click="clearProfileDraft">
                  清空
                </button>
              </div>
            </div>

            <div class="profile-select-list compact-profile-select-list">
              <label
                v-for="mod in filteredProfileSelectableMods"
                :key="mod.folderName"
                class="profile-select-item compact-profile-select-item"
                :class="{ selected: isProfileFolderSelected(mod.folderName) }"
              >
                <input
                  type="checkbox"
                  :checked="isProfileFolderSelected(mod.folderName)"
                  @change="toggleProfileFolder(mod.folderName)"
                />

                <div class="profile-select-main">
                  <strong>{{ mod.name }}</strong>
                  <p>{{ mod.author || '未知作者' }} · v{{ mod.version || '未知版本' }}</p>
                  <span>{{ mod.folderName }}</span>
                </div>

                <div class="profile-select-tags">
                  <span class="mod-type">{{ mod.modType.label }}</span>
                  <span :class="mod.isDisabled ? 'status-badge disabled' : 'status-badge enabled'">
                    {{ mod.isDisabled ? '当前禁用' : '当前启用' }}
                  </span>
                </div>
              </label>
            </div>

            <div class="profile-editor-footer">
              <button
                :disabled="!profileDraftName.trim() || profileDraftEnabledFolders.length === 0"
                @click="handleSaveProfileDraft"
              >
                保存配置
              </button>

              <button class="secondary" @click="closeProfileEditor">
                取消
              </button>
            </div>
          </section>
        </div>

        <div v-if="profiles.length > 0" class="profile-list-light">
          <article
            v-for="profile in profiles"
            :key="profile.id"
            class="profile-card light-profile-card"
          >
            <div class="profile-card-top">
              <div class="profile-main">
                <div class="profile-title-row">
                  <h4>{{ profile.name }}</h4>
                  <span v-if="currentProfileId === profile.id" class="experiment-chip current-profile-chip">当前</span>
                  <span v-else class="experiment-chip">配置</span>
                </div>

                <p>
                  {{ profile.enabledFolderNames.length }} 个启用 Mod · 更新于 {{ formatDateTime(profile.updatedAt) }}
                </p>
              </div>

              <div class="profile-actions profile-action-groups">
                <div class="profile-primary-actions">
                  <button
                    class="tiny-button profile-apply-action"
                    :disabled="!gamePath"
                    @click="handleApplyProfile(profile)"
                  >
                    {{ currentProfileId === profile.id ? '重新应用当前配置' : '应用并切换' }}
                  </button>

                  <button
                    v-if="currentProfileId !== profile.id"
                    class="tiny-button profile-mark-action"
                    @click="handleSetCurrentProfile(profile)"
                  >
                    仅标记当前
                  </button>

                  <span v-else class="profile-current-note">
                    已是当前配置
                  </span>
                </div>

                <p class="profile-action-note">
                  "应用并切换"会移动 Mods / Disabled Mods；"仅标记当前"只改变右侧显示，不移动文件。
                </p>

                <div class="profile-secondary-actions">
                  <button
                    class="tiny-button"
                    @click="startEditProfile(profile)"
                  >
                    编辑
                  </button>

                  <button
                    class="tiny-button"
                    @click="handleRenameProfile(profile)"
                  >
                    改名
                  </button>

                  <button
                    class="tiny-button"
                    @click="handleCopyProfile(profile)"
                  >
                    复制
                  </button>

                  <button
                    class="tiny-button"
                    @click="handleExportProfile(profile)"
                  >
                    导出
                  </button>

                  <button
                    class="tiny-button danger"
                    @click="handleDeleteProfile(profile.id)"
                  >
                    删除
                  </button>
                </div>
              </div>
            </div>

            <div class="profile-card-bottom">
              <button
                class="profile-link-button"
                @click="toggleProfilePreview(profile.id)"
              >
                {{ expandedProfileId === profile.id ? '收起包含的 Mod' : '查看包含的 Mod' }}
              </button>

              <div
                v-if="expandedProfileId !== profile.id"
                class="profile-preview-inline"
              >
                <span
                  v-for="folderName in profile.enabledFolderNames.slice(0, 6)"
                  :key="folderName"
                >
                  {{ folderName }}
                </span>
                <span v-if="profile.enabledFolderNames.length > 6">
                  +{{ profile.enabledFolderNames.length - 6 }}
                </span>
              </div>

              <div
                v-if="expandedProfileId === profile.id"
                class="profile-mod-preview expanded-profile-preview"
              >
                <span
                  v-for="folderName in profile.enabledFolderNames"
                  :key="folderName"
                >
                  {{ folderName }}
                </span>
              </div>
            </div>
          </article>
        </div>

        <div v-else class="empty-state profile-empty-state">
          <h3>还没有配置方案</h3>
          <p>创建配置方案来保存不同的 Mod 启用组合。</p>
          <div class="profile-empty-actions" style="margin-top:12px;display:flex;gap:8px;justify-content:center">
            <button class="tool-action-button" @click="startCreateProfile(false)">新建配置</button>
            <button class="tool-action-button secondary" @click="handleImportProfiles">导入配置</button>
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

        <div class="panel compact-panel">
          <div class="panel-header">
            <h3>Nexus Mods</h3>
            <span>NXM 下载认证</span>
          </div>

          <p class="muted-text">
            NXM 自动下载需要 Nexus Personal API Key。Junimo Box 只会把它保存在本机 localStorage，不会写入项目文件。
          </p>

          <div class="nexus-key-form">
            <input
              v-model="nexusApiKeyDraft"
              class="url-zip-input"
              :type="showNexusApiKey ? 'text' : 'password'"
              placeholder="粘贴 Nexus Personal API Key"
              autocomplete="off"
            />

            <button class="secondary" @click="showNexusApiKey = !showNexusApiKey">
              {{ showNexusApiKey ? "隐藏" : "显示" }}
            </button>
          </div>

          <div class="setting-actions nexus-setting-actions">
            <button @click="handleSaveNexusApiKey">保存 API Key</button>
            <button
              class="secondary"
              :disabled="isTestingNexusApiKey || !nexusApiKeyDraft.trim()"
              @click="handleTestNexusApiKey"
            >
              {{ isTestingNexusApiKey ? "测试中..." : "测试连接" }}
            </button>
            <button class="danger-button" :disabled="!nexusApiKey && !nexusApiKeyDraft" @click="handleClearNexusApiKey">
              清除
            </button>
          </div>

          <div class="setting-block nexus-status-block">
            <span>连接状态</span>
            <strong>{{ nexusApiStatus }}</strong>
            <small v-if="nexusApiUserName">账号：{{ nexusApiUserName }} · {{ nexusApiIsPremium ? "Premium" : "Free / 未识别会员状态" }}</small>
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

        <button
          class="launch-button smapi-install-button"
          :disabled="!gamePath || !stardewExists || isSmapiInstalling"
          @click="handleInstallSmapi"
        >
          {{ isSmapiInstalling ? smapiInstallStageMessage || "正在安装 SMAPI..." : smapiExists ? "更新 / 重装 SMAPI" : "安装 SMAPI" }}
        </button>

        <button
          v-if="smapiInstallerOpened"
          class="launch-button smapi-recheck-button"
          :disabled="isSmapiInstalling"
          @click="handleRecheckSmapiInstall"
        >
          我已完成安装，重新检测
        </button>

        <p v-if="isSmapiInstalling" class="side-install-stage">
          {{ smapiInstallStageMessage || "正在准备 SMAPI 安装..." }}
        </p>

        <p v-if="smapiInstallerOpened && !isSmapiInstalling" class="side-install-stage">
          安装器已打开。完成安装后请重新检测。
        </p>
      </div>

      <div class="side-card current-profile-side-card">
        <h4>当前配置</h4>
        <div class="info-line">
          <span>配置方案</span>
          <strong>{{ currentProfile?.name || "默认当前状态" }}</strong>
        </div>
        <div class="info-line">
          <span>启用 Mod</span>
          <strong>{{ currentProfileEnabledCount }}</strong>
        </div>
        <button class="side-check-button" @click="activeView = 'profiles'">管理配置</button>
      </div>

      <div class="side-card path-card">
        <h4>当前路径</h4>
        <p>{{ gamePath || "尚未选择 Stardew Valley 安装目录" }}</p>
      </div>

      <div class="side-card game-status-card">
        <h4>游戏状态</h4>
        <div class="info-line">
          <span>Stardew Valley</span>
          <strong :class="stardewExists ? 'ok' : 'bad'">{{ stardewExists ? "已找到" : "未找到" }}</strong>
        </div>
        <div class="info-line">
          <span>SMAPI</span>
          <strong :class="smapiExists ? 'ok' : 'bad'">{{ smapiExists ? "已安装" : "未安装" }}</strong>
        </div>
        <div class="info-line">
          <span>缺失依赖</span>
          <strong :class="missingDependencies.length > 0 ? 'bad' : 'ok'">{{ missingDependencies.length }}</strong>
        </div>
        <div class="info-line">
          <span>启动前检查</span>
          <strong :class="launchHealthStatus.className">{{ launchHealthStatus.label }}</strong>
        </div>
        <button class="side-check-button" :disabled="!gamePath" @click="handleRunLaunchCheck">检查环境</button>
      </div>
    </aside>

    <!-- 下载队列浮动按钮 -->
    <button
      class="download-queue-fab"
      :class="{ 'has-active': downloadQueue.some(i => i.status === 'downloading' || i.status === 'connecting') }"
      @click="isDownloadQueueOpen = !isDownloadQueueOpen"
      :title="`下载队列 (${downloadQueue.filter(i => i.status === 'queued' || i.status === 'downloading' || i.status === 'connecting').length} 活跃)`"
    >
      <span class="dq-fab-count">
        {{ downloadQueue.filter(i => i.status === 'queued' || i.status === 'downloading' || i.status === 'connecting').length }}
      </span>
      ⬇
    </button>

    <!-- 下载队列面板 -->
    <div v-if="isDownloadQueueOpen" class="download-queue-panel">
      <div class="dq-header">
        <h3>下载队列</h3>
        <div class="dq-header-actions">
          <button
            v-if="downloadQueue.some(i => i.status === 'completed' || i.status === 'failed' || i.status === 'cancelled')"
            class="tiny-button"
            @click="handleClearCompletedQueue"
          >
            清除已完成
          </button>
          <button class="tiny-button" @click="isDownloadQueueOpen = false">
            关闭
          </button>
        </div>
      </div>

      <div class="dq-list">
        <div
          v-for="item in downloadQueue"
          :key="item.id"
          class="dq-item"
          :class="`dq-${item.status}`"
        >
          <div class="dq-item-top">
            <span class="dq-file-name" :title="item.sourceUrl">{{ item.fileName }}</span>
            <span class="dq-source-tag">{{ item.source === "nxm" ? "NXM" : "URL" }}</span>
            <span class="dq-status-badge" :class="`dq-badge-${item.status}`">
              {{ item.status === "queued" ? "排队" : item.status === "connecting" ? "连接" : item.status === "downloading" ? "下载" : item.status === "merging" ? "合并" : item.status === "completed" ? "完成" : item.status === "cancelled" ? "取消" : "失败" }}
            </span>
          </div>

          <div
            v-if="item.status === 'downloading' || item.status === 'merging'"
            class="dq-progress-bar"
          >
            <div
              class="dq-progress-fill"
              :style="{ width: item.totalBytes > 0 ? (item.downloadedBytes / item.totalBytes * 100) + '%' : '5%' }"
            ></div>
          </div>

          <div class="dq-item-bottom">
            <span class="dq-message">{{ item.message }}</span>
            <span v-if="item.speedBytesPerSec > 0" class="dq-speed">{{ formatSpeed(item.speedBytesPerSec) }}</span>
            <span v-if="item.totalBytes > 0" class="dq-bytes">{{ formatBytes(item.downloadedBytes) }} / {{ formatBytes(item.totalBytes) }}</span>
          </div>

          <div class="dq-item-actions">
            <button
              v-if="item.status === 'failed' || item.status === 'cancelled'"
              class="tiny-button"
              @click="handleRetryDownload(item.id)"
            >
              重试
            </button>
            <button
              class="tiny-button"
              @click="handleRemoveFromQueue(item.id)"
            >
              {{ item.status === "downloading" || item.status === "connecting" ? "取消" : "删除" }}
            </button>
          </div>
        </div>

        <div v-if="downloadQueue.length === 0" class="dq-empty">
          没有下载任务
        </div>
      </div>
    </div>
    <!-- 新手引导 -->
    <div v-if="showWizard" class="wizard-overlay" @click.self="showWizard = false">
      <div class="wizard-card">
        <div class="wizard-steps">
          <span class="wizard-dot" :class="{ active: wizardStep >= 0 }"></span>
          <span class="wizard-dot" :class="{ active: wizardStep >= 1 }"></span>
          <span class="wizard-dot" :class="{ active: wizardStep >= 2 }"></span>
          <span class="wizard-dot" :class="{ active: wizardStep >= 3 }"></span>
        </div>

        <div v-if="wizardStep === 0" class="wizard-body">
          <div class="wizard-icon">🌿</div>
          <h3>欢迎使用 Junimo Box</h3>
          <p>你的星露谷 Mod 管理小站。先选择游戏目录，开始管理 Mod 吧。</p>
          <div class="wizard-actions">
            <button class="hero-action-button wizard-primary" @click="handleWizardSelectPath">
              选择游戏目录
            </button>
            <button v-if="wizardSkippable" class="tiny-button ghost-button" @click="handleWizardSkip">
              跳过引导
            </button>
          </div>
        </div>

        <div v-if="wizardStep === 1" class="wizard-body">
          <div class="wizard-icon">🛠️</div>
          <h3>检测 SMAPI</h3>
          <p>SMAPI 是星露谷 Mod 的运行基石，大部分 Mod 都需要它。</p>
          <div class="wizard-status">
            <span v-if="smapiExists" class="wizard-status-ok">✅ SMAPI 已安装</span>
            <span v-else class="wizard-status-missing">❌ SMAPI 未安装</span>
          </div>
          <div class="wizard-actions">
            <button
              v-if="!smapiExists"
              class="hero-action-button wizard-primary"
              :disabled="isSmapiInstalling"
              @click="handleWizardInstallSmapi"
            >
              {{ isSmapiInstalling ? "正在安装..." : "下载并安装 SMAPI" }}
            </button>
            <button class="hero-action-button secondary" @click="handleWizardNext">
              下一步
            </button>
            <button v-if="wizardSkippable" class="tiny-button ghost-button" @click="handleWizardSkip">
              跳过引导
            </button>
          </div>
        </div>

        <div v-if="wizardStep === 2" class="wizard-body">
          <div class="wizard-icon">🔍</div>
          <h3>扫描 Mods</h3>
          <p>检查你的 Mods 文件夹，看看已经安装了多少 Mod。</p>
          <div class="wizard-status">
            <span v-if="totalModCount > 0">
              已找到 {{ totalModCount }} 个 Mod（{{ mods.length }} 个启用，{{ disabledMods.length }} 个禁用）
            </span>
            <span v-else>尚未扫描 Mods 文件夹。</span>
          </div>
          <div class="wizard-actions">
            <button
              class="hero-action-button wizard-primary"
              :disabled="isScanning"
              @click="handleWizardScanMods"
            >
              {{ isScanning ? "扫描中..." : "扫描 Mods 文件夹" }}
            </button>
            <button class="hero-action-button secondary" @click="handleWizardNext">
              下一步
            </button>
            <button v-if="wizardSkippable" class="tiny-button ghost-button" @click="handleWizardSkip">
              跳过引导
            </button>
          </div>
        </div>

        <div v-if="wizardStep === 3" class="wizard-body">
          <div class="wizard-icon">🎉</div>
          <h3>准备就绪</h3>
          <p>
            {{ gamePath ? `已选择游戏目录，找到 ${totalModCount} 个 Mod。` : "随时可以选择游戏目录开始管理。" }}
            {{ smapiExists ? "SMAPI 已就绪，可以启动游戏。" : "" }}
          </p>
          <div class="wizard-actions">
            <button class="hero-action-button wizard-primary" @click="handleWizardFinish">
              开始使用
            </button>
          </div>
        </div>
      </div>
    </div>
  </main>

  <!-- 自定义模态对话框 -->
  <teleport to="body">
    <div v-if="modalState.visible" class="modal-overlay" @click.self="handleModalCancel">
      <div class="modal-box">
        <h3>{{ modalState.title }}</h3>
        <p v-if="modalState.mode === 'confirm'">{{ modalState.message }}</p>
        <input
          v-if="modalState.mode === 'prompt'"
          v-model="modalState.promptValue"
          class="modal-input"
          :placeholder="modalState.placeholder"
          @keyup.enter="handleModalConfirm"
        />
        <div class="modal-actions">
          <button class="tiny-button" @click="handleModalCancel">
            {{ modalState.cancelLabel || "取消" }}
          </button>
          <button
            class="tiny-button"
            :class="modalState.confirmLabel === '确认删除' ? 'danger' : ''"
            @click="handleModalConfirm"
          >
            {{ modalState.confirmLabel || "确认" }}
          </button>
        </div>
      </div>
    </div>
  </teleport>
</template>

<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { getCurrentWebview } from "@tauri-apps/api/webview";
import { open, save } from "@tauri-apps/plugin-dialog";
import { exists, readDir, readTextFile } from "@tauri-apps/plugin-fs";
import JSON5 from "json5";

const STORAGE_KEY = "junimo-box-game-path";
const PROFILES_STORAGE_KEY = "junimo-box-profiles";
const CURRENT_PROFILE_STORAGE_KEY = "junimo-box-current-profile";
const INSTALL_HISTORY_STORAGE_KEY = "junimo-box-install-history";
const NEXUS_API_KEY_STORAGE_KEY = "junimo-box-nexus-api-key";
const DOWNLOAD_QUEUE_STORAGE_KEY = "junimo-box-download-queue";

type ViewId = "overview" | "mods" | "logs" | "tools" | "profiles" | "settings";
type ModStatusFilter = "all" | "enabled" | "disabled";
type ModDependencyFilter = "all" | "missing";
type LaunchTarget = "smapi" | "vanilla";
type LaunchIssueLevel = "error" | "warning";
type ZipInstallConflictMode = "cancel" | "skip" | "replace";
type ZipInstallSource = "local" | "drag" | "url" | "nxm";

type LaunchCheckIssue = {
  level: LaunchIssueLevel;
  title: string;
  detail?: string;
};

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
  updateKeys: string[];
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

type SmapiInstallResult = {
  version: string;
  download_url: string;
  zip_path: string;
  installer_path: string;
};

type UrlZipDownloadResult = {
  download_url: string;
  zip_path: string;
  file_name: string;
  file_size: number;
  download_id: string;
};

type NexusUserInfo = {
  name: string;
  user_id: number;
  is_premium: boolean;
};

type ParsedNxmRequest = {
  raw: string;
  gameDomain: string;
  modId: string;
  fileId: string;
  key: string;
  expires: string;
  userId: string;
  nexusPageUrl: string;
};

type SmapiInstallStagePayload = {
  stage: string;
  message: string;
  version?: string | null;
  downloaded_bytes?: number | null;
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

type DownloadQueueStatus = "queued" | "connecting" | "downloading" | "merging" | "completed" | "failed" | "cancelled";

type DownloadQueueItem = {
  id: string;
  fileName: string;
  source: "nxm" | "url";
  sourceUrl: string;
  status: DownloadQueueStatus;
  downloadedBytes: number;
  totalBytes: number;
  speedBytesPerSec: number;
  message: string;
  createdAt: string;
  completedAt?: string;
  zipPath?: string;
  errorMessage?: string;
};

type DownloadProgressPayload = {
  download_id: string;
  file_name: string;
  stage: string;
  downloaded_bytes: number;
  total_bytes: number;
  speed_bytes_per_sec: number;
  message: string;
  zip_path?: string | null;
};

type ModProfile = {
  id: string;
  name: string;
  enabledFolderNames: string[];
  createdAt: string;
  updatedAt: string;
};

type ProfileExportItem = {
  name: string;
  enabledFolderNames: string[];
  createdAt?: string;
  updatedAt?: string;
};

type ProfileExportFile = {
  app: "Junimo Box";
  type: "mod-profiles";
  version: 1;
  exportedAt: string;
  profiles: ProfileExportItem[];
};

type InstallHistoryMod = {
  name: string;
  version: string;
  folderName: string;
  uniqueId: string;
};

type InstallHistoryItem = {
  id: string;
  installedAt: string;
  source: ZipInstallSource;
  sourceLabel: string;
  conflictMode: ZipInstallConflictMode;
  zipPath: string;
  mods: InstallHistoryMod[];
  note: string;
};

type UpdateCheckItem = {
  key: string;
  name: string;
  folderName: string;
  version: string;
  isDisabled: boolean;
  updateKey: string;
  sourceLabel: string;
  sourceUrl: string;
  statusLabel: string;
};

type ZipInstallConflictRow = {
  preview: ZipModPreview;
  installedMod: DisplayModInfo;
};

type DeletedModInfo = {
  folder_name: string;
  original_name: string;
  deleted_at: string;
};

const navItems: Array<{ id: ViewId; label: string; icon: string }> = [
  { id: "overview", label: "总览", icon: "🏡" },
  { id: "mods", label: "Mods", icon: "📦" },
  { id: "logs", label: "日志", icon: "📜" },
  { id: "tools", label: "工具箱", icon: "🧰" },
  { id: "profiles", label: "配置", icon: "🧩" },
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
const smapiDetectedVersion = ref("");
const modsFolderExists = ref(false);
const isScanning = ref(false);
const isSmapiInstalling = ref(false);
const smapiInstallerOpened = ref(false);
const smapiInstallerVersion = ref("");
const smapiInstallStageMessage = ref("");
const smapiInstallDownloadedBytes = ref(0);
type NoticeType = "success" | "info" | "warning" | "error";

type NoticePayload = {
  type: NoticeType;
  text: string;
};

interface ToastItem {
  id: number;
  type: NoticeType;
  text: string;
  action?: {
    label: string;
    handler: () => void;
  };
}

const notice = ref<NoticePayload | null>(null);
const toasts = ref<ToastItem[]>([]);
let toastIdCounter = 0;

const noticeIcon = computed(() => {
  if (!notice.value) {
    return "";
  }

  const iconMap: Record<NoticeType, string> = {
    success: "✅",
    info: "ℹ️",
    warning: "⚠️",
    error: "❌",
  };

  return iconMap[notice.value.type];
});

const message = {
  get value() {
    return notice.value?.text ?? "";
  },
  set value(text: string) {
    if (!text) {
      clearNotice();
      return;
    }

    setNotice(inferNoticeType(text), text);
  },
};

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
const selectedZipInstallSource = ref<ZipInstallSource>("local");
const installHistory = ref<InstallHistoryItem[]>([]);
const updateCheckResults = ref<UpdateCheckItem[]>([]);
const isZipDragOver = ref(false);
const urlZipInput = ref("");
const nxmManualInput = ref("");
const nxmProtocolStatus = ref("");
const nxmRequestLink = ref("");
const isNxmDownloading = ref(false);
const nxmDownloadMessage = ref("");
const downloadQueue = ref<DownloadQueueItem[]>([]);
const isDownloadQueueOpen = ref(false);
let unlistenDownloadProgress: (() => void) | null = null;
const nexusApiKey = ref("");
const nexusApiKeyDraft = ref("");
const showNexusApiKey = ref(false);
const isTestingNexusApiKey = ref(false);
const nexusApiStatus = ref("未配置");
const nexusApiUserName = ref("");
const nexusApiIsPremium = ref(false);

// 模态对话框状态
interface ModalState {
  visible: boolean;
  mode: "confirm" | "prompt";
  title: string;
  message: string;
  confirmLabel?: string;
  cancelLabel?: string;
  promptValue?: string;
  placeholder?: string;
    resolve?: (value: any) => void;
}

const modalState = ref<ModalState>({
  visible: false,
  mode: "confirm",
  title: "",
  message: "",
});

function showConfirmModal(title: string, message: string): Promise<boolean> {
  return new Promise((resolve) => {
    modalState.value = {
      visible: true,
      mode: "confirm",
      title,
      message,
      confirmLabel: "确认",
      cancelLabel: "取消",
      resolve,
    };
  });
}

function showPromptModal(title: string, placeholder: string, defaultValue?: string): Promise<string | null> {
  return new Promise((resolve) => {
    modalState.value = {
      visible: true,
      mode: "prompt",
      title,
      message: "",
      promptValue: defaultValue || "",
      placeholder,
      confirmLabel: "确认",
      cancelLabel: "取消",
      resolve,
    };
  });
}

function handleModalConfirm() {
  const state = modalState.value;
  if (state.resolve) {
    if (state.mode === "prompt") {
      state.resolve(state.promptValue || "");
    } else {
      state.resolve(true);
    }
  }
  modalState.value.visible = false;
}

function handleModalCancel() {
  const state = modalState.value;
  if (state.resolve) {
    if (state.mode === "prompt") {
      state.resolve(null);
    } else {
      state.resolve(false);
    }
  }
  modalState.value.visible = false;
}

let unlistenDragDrop: (() => void) | null = null;
let unlistenSmapiInstallStage: UnlistenFn | null = null;
let nxmPendingPollTimer: ReturnType<typeof setInterval> | null = null;

const modSearchQuery = ref("");
const modStatusFilter = ref<ModStatusFilter>("all");
const modDependencyFilter = ref<ModDependencyFilter>("all");
const selectedModKey = ref("");
const selectedModKeys = ref<Set<string>>(new Set());
const lastSelectedModIndex = ref(-1);

const showWizard = ref(false);
const wizardStep = ref(0);
const wizardSkippable = ref(true);
const profiles = ref<ModProfile[]>([]);
const currentProfileId = ref("");
const isProfileEditorOpen = ref(false);
const profileEditorMode = ref<"create" | "edit">("create");
const editingProfileId = ref("");
const profileDraftName = ref("");
const profileDraftSearchQuery = ref("");
const profileDraftEnabledFolders = ref<string[]>([]);
const expandedProfileId = ref("");
const deletedMods = ref<DeletedModInfo[]>([]);
const isRecycleBinLoading = ref(false);
const isRestoringMap = ref<Record<string, boolean>>({});

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

const currentProfile = computed<ModProfile | null>(() =>
  profiles.value.find((profile) => profile.id === currentProfileId.value) || null
);

const currentProfileEnabledCount = computed(() =>
  currentProfile.value ? currentProfile.value.enabledFolderNames.length : mods.value.length
);

const zipInstallConflicts = computed<ZipInstallConflictRow[]>(() => {
  const installedMods = allDisplayMods.value;

  return zipModPreviews.value
    .map((preview) => {
      const installedMod = installedMods.find(
        (mod) => mod.folderName.toLowerCase() === preview.suggested_folder.toLowerCase()
      );

      return installedMod ? { preview, installedMod } : null;
    })
    .filter((row): row is ZipInstallConflictRow => row !== null);
});

const hasZipInstallConflicts = computed(() => zipInstallConflicts.value.length > 0);

const installableZipModCount = computed(() =>
  zipModPreviews.value.length - zipInstallConflicts.value.length
);

const recentInstallHistory = computed(() => installHistory.value.slice(0, 8));

const totalModCount = computed(() => mods.value.length + disabledMods.value.length);

const parsedNxmRequest = computed<ParsedNxmRequest>(() => parseNxmLink(nxmRequestLink.value));

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
  profiles: {
    eyebrow: "Profiles",
    title: "配置方案",
    description: "保存、查看并应用不同的 Mod 启用组合。",
  },
  settings: {
    eyebrow: "Settings",
    title: "设置",
    description: "管理本地路径和 Junimo Box 基础偏好。",
  },
};

function setNotice(type: NoticeType, text: string) {
  notice.value = { type, text };
}

function clearNotice() {
  notice.value = null;
}

function addToast(type: ToastItem["type"], text: string, action?: ToastItem["action"], timeout = 4000) {
  const id = ++toastIdCounter;
  toasts.value.push({ id, type, text, action });
  if (action) {
    setTimeout(() => {
      removeToast(id);
      if (!toasts.value.some((t) => t.action)) {
        void scanMods();
      }
    }, timeout);
  } else {
    setTimeout(() => removeToast(id), timeout);
  }
}

function removeToast(id: number) {
  toasts.value = toasts.value.filter((t) => t.id !== id);
}

function inferNoticeType(text: string): NoticeType {
  if (
    text.includes("但发现") ||
    text.includes("跳过") ||
    text.includes("缺失") ||
    text.includes("未安装")
  ) {
    return "warning";
  }

  if (
    text.includes("失败") ||
    text.includes("错误") ||
    text.includes("无法") ||
    text.includes("请先") ||
    text.includes("请选择") ||
    text.includes("请拖入") ||
    text.includes("请至少") ||
    text.includes("未找到") ||
    text.includes("没有找到")
  ) {
    return "error";
  }

  if (
    text.includes("已") ||
    text.includes("完成") ||
    text.includes("正常") ||
    text.includes("成功")
  ) {
    return "success";
  }

  return "info";
}

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

const profileSelectableMods = computed<DisplayModInfo[]>(() =>
  [...allDisplayMods.value].sort((a, b) => a.name.localeCompare(b.name))
);

const filteredProfileSelectableMods = computed<DisplayModInfo[]>(() => {
  const query = profileDraftSearchQuery.value.trim().toLowerCase();

  if (!query) {
    return profileSelectableMods.value;
  }

  return profileSelectableMods.value.filter((mod) =>
    [
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
      .includes(query)
  );
});

const launchHealthStatus = computed(() => {
  if (!gamePath.value) {
    return { label: "未配置", className: "bad" };
  }
  if (!stardewExists.value) {
    return { label: "异常", className: "bad" };
  }
  const warningCount =
    missingDependencies.value.length +
    skippedFolders.value.length +
    duplicateEnabledUniqueIds.value.length +
    (modsFolderExists.value ? 0 : 1);
  if (!smapiExists.value) {
    return { label: "缺少 SMAPI", className: "bad" };
  }
  if (warningCount > 0) {
    return { label: `${warningCount} 个警告`, className: "bad" };
  }
  return { label: "正常", className: "ok" };
});

const selectedMod = computed<DisplayModInfo | null>(() => {
  if (!selectedModKey.value) {
    return null;
  }

  return allDisplayMods.value.find((mod) => getModKey(mod) === selectedModKey.value) || null;
});

const duplicateEnabledUniqueIds = computed(() => getDuplicateUniqueIds(mods.value));

onMounted(async () => {
  unlistenSmapiInstallStage = await listen<SmapiInstallStagePayload>(
    "smapi-install-stage",
    (event) => {
      smapiInstallStageMessage.value = event.payload.message;
      smapiInstallDownloadedBytes.value = event.payload.downloaded_bytes ?? smapiInstallDownloadedBytes.value;

      if (isSmapiInstalling.value) {
        setNotice("info", event.payload.message);
      }
    }
  );

  unlistenDownloadProgress = await listen<DownloadProgressPayload>(
    "download-progress",
    (event) => {
      const payload = event.payload;
      const idx = downloadQueue.value.findIndex((item) => item.id === payload.download_id);

      if (idx === -1) return;

      const item = downloadQueue.value[idx];
      item.status = mapStageToStatus(payload.stage);
      item.downloadedBytes = payload.downloaded_bytes;
      item.totalBytes = payload.total_bytes;
      item.speedBytesPerSec = payload.speed_bytes_per_sec;
      item.message = payload.message;

      if (payload.stage === "completed" && payload.zip_path) {
        item.completedAt = new Date().toISOString();
        item.zipPath = payload.zip_path;
        void previewZipPath(payload.zip_path, item.source === "nxm" ? "nxm" : "url");
        void processNextInQueue();
      }

      if (payload.stage === "failed") {
        item.errorMessage = payload.message;
        void processNextInQueue();
      }

      if (payload.stage === "cancelled") {
        void processNextInQueue();
      }

      saveDownloadQueue();
    }
  );

  loadProfiles();
  loadCurrentProfile();
  loadInstallHistory();
  loadDownloadQueue();
  loadNexusApiKey();
  await setupZipDragDrop();

  const savedPath = localStorage.getItem(STORAGE_KEY);

  if (savedPath) {
    gamePath.value = savedPath;
    await checkGameFiles(savedPath);
    await scanMods();
  }

  await checkStartupNxmLink();
  await checkPendingNxmLink();

  nxmPendingPollTimer = setInterval(() => {
    void checkPendingNxmLink();
  }, 1000);

  document.addEventListener("keydown", handleGlobalKeydown);

  const setupDone = localStorage.getItem("junimo-box-setup-complete");
  if (!setupDone && !savedPath) {
    showWizard.value = true;
    wizardStep.value = 0;
  }
});

onUnmounted(() => {
  if (unlistenDragDrop) {
    unlistenDragDrop();
    unlistenDragDrop = null;
  }

  if (unlistenSmapiInstallStage) {
    unlistenSmapiInstallStage();
    unlistenSmapiInstallStage = null;
  }

  if (unlistenDownloadProgress) {
    unlistenDownloadProgress();
    unlistenDownloadProgress = null;
  }

  if (nxmPendingPollTimer) {
    clearInterval(nxmPendingPollTimer);
    nxmPendingPollTimer = null;
  }

  document.removeEventListener("keydown", handleGlobalKeydown);
});


function loadProfiles() {
  try {
    const rawProfiles = localStorage.getItem(PROFILES_STORAGE_KEY);

    if (!rawProfiles) {
      profiles.value = [];
      return;
    }

    const parsedProfiles = JSON.parse(rawProfiles) as ModProfile[];

    profiles.value = Array.isArray(parsedProfiles)
      ? parsedProfiles.filter(
          (profile) =>
            profile &&
            typeof profile.id === "string" &&
            typeof profile.name === "string" &&
            Array.isArray(profile.enabledFolderNames)
        )
      : [];

    if (currentProfileId.value && !profiles.value.some((profile) => profile.id === currentProfileId.value)) {
      currentProfileId.value = "";
      localStorage.removeItem(CURRENT_PROFILE_STORAGE_KEY);
    }
  } catch (error) {
    profiles.value = [];
    message.value = `读取配置方案失败：${String(error)}`;
  }
}

function saveProfiles() {
  localStorage.setItem(PROFILES_STORAGE_KEY, JSON.stringify(profiles.value));
}

function loadCurrentProfile() {
  currentProfileId.value = localStorage.getItem(CURRENT_PROFILE_STORAGE_KEY) || "";
}

function saveCurrentProfile() {
  if (currentProfileId.value) {
    localStorage.setItem(CURRENT_PROFILE_STORAGE_KEY, currentProfileId.value);
  } else {
    localStorage.removeItem(CURRENT_PROFILE_STORAGE_KEY);
  }
}

function handleSetCurrentProfile(profile: ModProfile) {
  currentProfileId.value = profile.id;
  saveCurrentProfile();
  setNotice("success", `当前配置方案已设为：${profile.name}`);
}

function loadInstallHistory() {
  try {
    const rawHistory = localStorage.getItem(INSTALL_HISTORY_STORAGE_KEY);

    if (!rawHistory) {
      installHistory.value = [];
      return;
    }

    const parsedHistory = JSON.parse(rawHistory) as InstallHistoryItem[];
    installHistory.value = Array.isArray(parsedHistory) ? parsedHistory.slice(0, 50) : [];
  } catch (error) {
    installHistory.value = [];
    console.warn("读取安装历史失败", error);
  }
}

function saveInstallHistory() {
  localStorage.setItem(INSTALL_HISTORY_STORAGE_KEY, JSON.stringify(installHistory.value.slice(0, 50)));
}

function loadDownloadQueue() {
  try {
    const raw = localStorage.getItem(DOWNLOAD_QUEUE_STORAGE_KEY);
    if (!raw) {
      downloadQueue.value = [];
      return;
    }
    const parsed = JSON.parse(raw) as DownloadQueueItem[];
    downloadQueue.value = Array.isArray(parsed) ? parsed.filter(
      (item) => item.status !== "completed" && item.status !== "failed" && item.status !== "cancelled"
    ) : [];
  } catch {
    downloadQueue.value = [];
  }
}

function saveDownloadQueue() {
  localStorage.setItem(DOWNLOAD_QUEUE_STORAGE_KEY, JSON.stringify(downloadQueue.value.slice(0, 100)));
}

function addInstallHistory(
  installedMods: ZipModPreview[],
  source: ZipInstallSource,
  conflictMode: ZipInstallConflictMode,
  zipPath: string,
  note: string
) {
  const sourceLabelMap: Record<ZipInstallSource, string> = {
    local: "本地 ZIP",
    drag: "拖拽 ZIP",
    url: "URL ZIP",
    nxm: "Nexus NXM",
  };

  installHistory.value.unshift({
    id: `install-${Date.now()}-${Math.random().toString(16).slice(2)}`,
    installedAt: new Date().toISOString(),
    source,
    sourceLabel: sourceLabelMap[source],
    conflictMode,
    zipPath,
    note,
    mods: installedMods.map((mod) => ({
      name: mod.name || mod.suggested_folder,
      version: mod.version || "",
      folderName: mod.suggested_folder,
      uniqueId: mod.unique_id || "",
    })),
  });

  installHistory.value = installHistory.value.slice(0, 50);
  saveInstallHistory();
}

function clearInstallHistory() {
  installHistory.value = [];
  saveInstallHistory();
  setNotice("info", "已清空安装历史。");
}

async function handleExportInstallHistory() {
  const filePath = await save({
    title: "导出安装历史",
    defaultPath: `junimo-box-install-history-${new Date().toISOString().slice(0, 10)}.json`,
    filters: [{ name: "JSON 文件", extensions: ["json"] }],
  });
  if (!filePath) return;
  try {
    await invoke("write_text_file", {
      path: filePath,
      content: JSON.stringify(installHistory.value, null, 2),
    });
    setNotice("success", "安装历史已导出。");
  } catch (error) {
    setNotice("error", `导出安装历史失败：${String(error)}`);
  }
}

function createProfileId() {
  return `profile-${Date.now()}-${Math.random().toString(16).slice(2)}`;
}

function formatDateTime(value: string) {
  if (!value) {
    return "未知时间";
  }

  return new Date(value).toLocaleString();
}

function getCurrentEnabledFolderNames() {
  return mods.value
    .map((mod) => mod.folderName)
    .filter(Boolean)
    .sort((a, b) => a.localeCompare(b));
}

function startCreateProfile(useCurrentEnabled: boolean) {
  profileEditorMode.value = "create";
  editingProfileId.value = "";
  profileDraftName.value = "";
  profileDraftSearchQuery.value = "";
  profileDraftEnabledFolders.value = useCurrentEnabled ? getCurrentEnabledFolderNames() : [];
  isProfileEditorOpen.value = true;

  message.value = useCurrentEnabled
    ? `已填入当前启用的 ${profileDraftEnabledFolders.value.length} 个 Mod，可以继续调整后保存。`
    : "请选择这个配置要启用的 Mod。";
}

function startEditProfile(profile: ModProfile) {
  profileEditorMode.value = "edit";
  editingProfileId.value = profile.id;
  profileDraftName.value = profile.name;
  profileDraftSearchQuery.value = "";
  profileDraftEnabledFolders.value = [...profile.enabledFolderNames];
  isProfileEditorOpen.value = true;
}

function closeProfileEditor() {
  isProfileEditorOpen.value = false;
  editingProfileId.value = "";
  profileDraftName.value = "";
  profileDraftSearchQuery.value = "";
  profileDraftEnabledFolders.value = [];
}

function isProfileFolderSelected(folderName: string) {
  return profileDraftEnabledFolders.value.includes(folderName);
}

function toggleProfileFolder(folderName: string) {
  if (isProfileFolderSelected(folderName)) {
    profileDraftEnabledFolders.value = profileDraftEnabledFolders.value.filter(
      (item) => item !== folderName
    );
    return;
  }

  profileDraftEnabledFolders.value = [...profileDraftEnabledFolders.value, folderName].sort((a, b) =>
    a.localeCompare(b)
  );
}

function selectAllProfileMods() {
  profileDraftEnabledFolders.value = profileSelectableMods.value
    .map((mod) => mod.folderName)
    .filter(Boolean)
    .sort((a, b) => a.localeCompare(b));
}

function clearProfileDraft() {
  profileDraftEnabledFolders.value = [];
}

function toggleProfilePreview(profileId: string) {
  expandedProfileId.value = expandedProfileId.value === profileId ? "" : profileId;
}

function createProfileExportItem(profile: ModProfile): ProfileExportItem {
  return {
    name: profile.name,
    enabledFolderNames: [...profile.enabledFolderNames].sort((a, b) => a.localeCompare(b)),
    createdAt: profile.createdAt,
    updatedAt: profile.updatedAt,
  };
}

function createProfileExportFile(profileItems: ProfileExportItem[]): ProfileExportFile {
  return {
    app: "Junimo Box",
    type: "mod-profiles",
    version: 1,
    exportedAt: new Date().toISOString(),
    profiles: profileItems,
  };
}

function sanitizeExportFileName(value: string) {
  return value
    .trim()
    .replace(/[<>:"/\\|?*]+/g, "_")
    .replace(/\s+/g, " ")
    .slice(0, 60) || "profile";
}

function makeUniqueProfileName(baseName: string, ignoredProfileId = "") {
  const base = baseName.trim() || "未命名配置";
  const existingNames = new Set(
    profiles.value
      .filter((profile) => profile.id !== ignoredProfileId)
      .map((profile) => profile.name)
  );

  if (!existingNames.has(base)) {
    return base;
  }

  let index = 2;
  let candidate = `${base} 副本`;

  while (existingNames.has(candidate)) {
    candidate = `${base} 副本 ${index}`;
    index += 1;
  }

  return candidate;
}

function normalizeProfileImportItem(item: unknown): ProfileExportItem | null {
  if (!item || typeof item !== "object") {
    return null;
  }

  const raw = item as {
    name?: unknown;
    enabledFolderNames?: unknown;
    createdAt?: unknown;
    updatedAt?: unknown;
  };

  if (typeof raw.name !== "string" || !Array.isArray(raw.enabledFolderNames)) {
    return null;
  }

  const enabledFolderNames = raw.enabledFolderNames
    .filter((folderName): folderName is string => typeof folderName === "string" && folderName.trim().length > 0)
    .map((folderName) => folderName.trim())
    .filter((folderName) => !/[\\/]/.test(folderName));

  if (enabledFolderNames.length === 0) {
    return null;
  }

  return {
    name: raw.name.trim() || "导入的配置",
    enabledFolderNames: Array.from(new Set(enabledFolderNames)).sort((a, b) => a.localeCompare(b)),
    createdAt: typeof raw.createdAt === "string" ? raw.createdAt : undefined,
    updatedAt: typeof raw.updatedAt === "string" ? raw.updatedAt : undefined,
  };
}

function parseProfileImportFile(content: string): ProfileExportItem[] {
  const parsed = JSON.parse(content) as unknown;

  if (!parsed || typeof parsed !== "object") {
    return [];
  }

  const raw = parsed as {
    profiles?: unknown;
    profile?: unknown;
    name?: unknown;
    enabledFolderNames?: unknown;
  };

  if (Array.isArray(raw.profiles)) {
    return raw.profiles
      .map(normalizeProfileImportItem)
      .filter((profile): profile is ProfileExportItem => profile !== null);
  }

  if (raw.profile) {
    const profile = normalizeProfileImportItem(raw.profile);
    return profile ? [profile] : [];
  }

  const singleProfile = normalizeProfileImportItem(raw);
  return singleProfile ? [singleProfile] : [];
}

async function handleExportProfile(profile: ModProfile) {
  const filePath = await save({
    title: "导出配置方案",
    defaultPath: `${sanitizeExportFileName(profile.name)}.junimo-profile.json`,
    filters: [
      {
        name: "Junimo Box 配置方案",
        extensions: ["json"],
      },
    ],
  });

  if (!filePath) {
    return;
  }

  const exportFile = createProfileExportFile([createProfileExportItem(profile)]);

  try {
    await invoke("write_text_file", {
      path: filePath,
      content: JSON.stringify(exportFile, null, 2),
    });

    setNotice("success", `已导出配置方案：${profile.name}`);
  } catch (error) {
    setNotice("error", `导出配置方案失败：${String(error)}`);
  }
}

async function handleExportAllProfiles() {
  if (profiles.value.length === 0) {
    setNotice("warning", "当前没有可导出的配置方案。");
    return;
  }

  const filePath = await save({
    title: "导出全部配置方案",
    defaultPath: "junimo-box-profiles.json",
    filters: [
      {
        name: "Junimo Box 配置方案",
        extensions: ["json"],
      },
    ],
  });

  if (!filePath) {
    return;
  }

  const exportFile = createProfileExportFile(profiles.value.map(createProfileExportItem));

  try {
    await invoke("write_text_file", {
      path: filePath,
      content: JSON.stringify(exportFile, null, 2),
    });

    setNotice("success", `已导出 ${profiles.value.length} 个配置方案。`);
  } catch (error) {
    setNotice("error", `导出配置方案失败：${String(error)}`);
  }
}

async function handleImportProfiles() {
  const selected = await open({
    directory: false,
    multiple: false,
    title: "导入配置方案",
    filters: [
      {
        name: "Junimo Box 配置方案",
        extensions: ["json"],
      },
    ],
  });

  if (typeof selected !== "string") {
    return;
  }

  try {
    const content = await readTextFile(selected);
    const rawParsed = JSON.parse(content) as Record<string, unknown>;
    const importWarnings: string[] = [];

    if (rawParsed && typeof rawParsed === "object") {
      if (rawParsed.app && rawParsed.app !== "Junimo Box") {
        importWarnings.push(`该文件来自 "${rawParsed.app}"，可能不兼容。`);
      }
      if (rawParsed.version !== undefined && rawParsed.version !== 1) {
        importWarnings.push(`配置文件版本为 ${rawParsed.version}，当前支持版本 1，可能无法正确导入。`);
      }
    }

    const importedProfiles = parseProfileImportFile(content);

    if (importedProfiles.length === 0) {
      setNotice("error", "导入失败：这个文件里没有有效的配置方案。");
      return;
    }

    const now = new Date().toISOString();
    const normalizedProfiles: ModProfile[] = importedProfiles.map((profile) => ({
      id: createProfileId(),
      name: makeUniqueProfileName(profile.name),
      enabledFolderNames: [...profile.enabledFolderNames].sort((a, b) => a.localeCompare(b)),
      createdAt: profile.createdAt || now,
      updatedAt: now,
    }));

    profiles.value = [...normalizedProfiles, ...profiles.value];
    saveProfiles();

    let noticeText = `已导入 ${normalizedProfiles.length} 个配置方案。`;
    if (importWarnings.length > 0) {
      noticeText += " " + importWarnings.join(" ");
    }
    setNotice(importWarnings.length > 0 ? "warning" : "success", noticeText);
  } catch (error) {
    setNotice("error", `导入配置方案失败：${String(error)}`);
  }
}

async function handleRenameProfile(profile: ModProfile) {
  const newName = await showPromptModal("重命名配置方案", "例如：日常游玩 / SVE / 多人联机", profile.name);

  if (!newName || newName === profile.name) {
    return;
  }

  const duplicate = profiles.value.some(
    (item) => item.id !== profile.id && item.name === newName
  );

  if (duplicate) {
    setNotice("error", `重命名失败：已经存在名为「${newName}」的配置方案。`);
    return;
  }

  profile.name = newName;
  profile.updatedAt = new Date().toISOString();
  saveProfiles();
  setNotice("success", `已重命名配置方案：${newName}`);
}

function handleCopyProfile(profile: ModProfile) {
  const now = new Date().toISOString();
  const copiedName = makeUniqueProfileName(`${profile.name} 副本`);

  profiles.value.unshift({
    id: createProfileId(),
    name: copiedName,
    enabledFolderNames: [...profile.enabledFolderNames].sort((a, b) => a.localeCompare(b)),
    createdAt: now,
    updatedAt: now,
  });

  saveProfiles();
  setNotice("success", `已复制配置方案：${copiedName}`);
}

function handleSaveProfileDraft() {
  const name = profileDraftName.value.trim();

  if (!name) {
    message.value = "请先输入配置方案名称。";
    return;
  }

  if (profileDraftEnabledFolders.value.length === 0) {
    message.value = "请至少勾选一个 Mod。";
    return;
  }

  const now = new Date().toISOString();
  const enabledFolderNames = [...profileDraftEnabledFolders.value].sort((a, b) =>
    a.localeCompare(b)
  );

  if (profileEditorMode.value === "edit" && editingProfileId.value) {
    const targetProfile = profiles.value.find((profile) => profile.id === editingProfileId.value);

    if (!targetProfile) {
      message.value = "保存失败：没有找到要编辑的配置方案。";
      return;
    }

    targetProfile.name = name;
    targetProfile.enabledFolderNames = enabledFolderNames;
    targetProfile.updatedAt = now;
    message.value = `已更新配置方案：${name}`;
  } else {
    const existingProfile = profiles.value.find((profile) => profile.name === name);

    if (existingProfile) {
      existingProfile.enabledFolderNames = enabledFolderNames;
      existingProfile.updatedAt = now;
      message.value = `已覆盖同名配置方案：${name}`;
    } else {
      profiles.value.unshift({
        id: createProfileId(),
        name,
        enabledFolderNames,
        createdAt: now,
        updatedAt: now,
      });
      message.value = `已保存配置方案：${name}`;
    }
  }

  saveProfiles();
  closeProfileEditor();
}

function handleDeleteProfile(profileId: string) {
  const targetProfile = profiles.value.find((profile) => profile.id === profileId);
  profiles.value = profiles.value.filter((profile) => profile.id !== profileId);

  if (currentProfileId.value === profileId) {
    currentProfileId.value = "";
    saveCurrentProfile();
  }

  saveProfiles();
  message.value = targetProfile ? `已删除配置方案：${targetProfile.name}` : "已删除配置方案。";
}

async function handleApplyProfile(profile: ModProfile) {
  if (!gamePath.value) {
    setNotice("error", "请先选择游戏目录。");
    return;
  }

  const targetEnabledFolders = new Set(profile.enabledFolderNames);
  const errors: string[] = [];
  let enabledCount = 0;
  let disabledCount = 0;

  for (const mod of [...disabledMods.value]) {
    if (!targetEnabledFolders.has(mod.folderName)) {
      continue;
    }

    const from = `${gamePath.value}\\Disabled Mods\\${mod.folderName}`;
    const to = `${gamePath.value}\\Mods\\${mod.folderName}`;

    try {
      if (await exists(to)) {
        errors.push(`启用跳过：Mods 中已存在 ${mod.folderName}`);
        continue;
      }

      await invoke("move_folder", { from, to });
      enabledCount += 1;
    } catch (error) {
      errors.push(`启用失败：${mod.folderName} - ${String(error)}`);
    }
  }

  for (const mod of [...mods.value]) {
    if (targetEnabledFolders.has(mod.folderName)) {
      continue;
    }

    const from = `${gamePath.value}\\Mods\\${mod.folderName}`;
    const to = `${gamePath.value}\\Disabled Mods\\${mod.folderName}`;

    try {
      if (await exists(to)) {
        errors.push(`禁用跳过：Disabled Mods 中已存在 ${mod.folderName}`);
        continue;
      }

      await invoke("move_folder", { from, to });
      disabledCount += 1;
    } catch (error) {
      errors.push(`禁用失败：${mod.folderName} - ${String(error)}`);
    }
  }

  await checkGameFiles(gamePath.value);
  await scanMods();
  selectedModKey.value = "";
  selectedModKeys.value = new Set();

  currentProfileId.value = profile.id;
  saveCurrentProfile();

  if (errors.length > 0) {
    message.value = `已应用配置：${profile.name}。启用 ${enabledCount} 个，禁用 ${disabledCount} 个；有 ${errors.length} 个操作被跳过或失败。${errors.slice(0, 2).join("；")}`;
  } else {
    message.value = `已应用配置：${profile.name}。启用 ${enabledCount} 个，禁用 ${disabledCount} 个。`;
  }

  activeView.value = "mods";
}

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
  smapiDetectedVersion.value = "";
  skippedFolders.value = [];
  missingDependencies.value = [];
  lastInstalledZipMods.value = [];
  selectedModKey.value = "";
  selectedModKeys.value = new Set();

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

  if (smapiExists.value) {
    await refreshSmapiVersionFromLatestLog();
  } else {
    smapiDetectedVersion.value = "";
  }
}

async function scanMods() {
  if (!gamePath.value) {
    message.value = "请先选择游戏目录。";
    return;
  }

  if (isScanning.value) return;
  isScanning.value = true;
  selectedModKey.value = "";
  selectedModKeys.value = new Set();

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
  } finally {
    isScanning.value = false;
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
        updateKeys: normalizeUpdateKeys(manifest.UpdateKeys),
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

function getPrimaryUpdateKey(mod: DisplayModInfo): string {
  return (mod.updateKeys || []).find((key) => /^Nexus:/i.test(key) || /^ModDrop:/i.test(key) || /^GitHub:/i.test(key)) || (mod.updateKeys || [])[0] || "";
}

function getUpdateKeyUrl(updateKey: string): string {
  const [source, id = ""] = updateKey.split(":");

  if (/^Nexus$/i.test(source) && id) {
    return `https://www.nexusmods.com/stardewvalley/mods/${encodeURIComponent(id)}`;
  }

  if (/^ModDrop$/i.test(source) && id) {
    return `https://www.moddrop.com/stardew-valley/mods/${encodeURIComponent(id)}`;
  }

  if (/^GitHub$/i.test(source) && id) {
    return id.startsWith("http") ? id : `https://github.com/${id}`;
  }

  return "";
}

function getUpdateSourceLabel(updateKey: string): string {
  if (/^Nexus:/i.test(updateKey)) return "Nexus";
  if (/^ModDrop:/i.test(updateKey)) return "ModDrop";
  if (/^GitHub:/i.test(updateKey)) return "GitHub";
  return updateKey ? "其他来源" : "未提供";
}

function handleRunUpdateCheck() {
  const checkedMods = allDisplayMods.value;

  updateCheckResults.value = checkedMods.map((mod) => {
    const updateKey = getPrimaryUpdateKey(mod);
    const sourceUrl = getUpdateKeyUrl(updateKey);
    const sourceLabel = getUpdateSourceLabel(updateKey);

    return {
      key: getModKey(mod),
      name: mod.name,
      folderName: mod.folderName,
      version: mod.version || "未知版本",
      isDisabled: mod.isDisabled,
      updateKey,
      sourceLabel,
      sourceUrl,
      statusLabel: sourceUrl ? "可打开来源页面检查更新" : "manifest 未提供可识别更新来源",
    };
  });

  const updatableCount = updateCheckResults.value.filter((item) => item.sourceUrl).length;
  setNotice("info", `更新检测完成：${updatableCount} 个 Mod 提供了可打开的更新来源。`);
}

async function handleOpenUpdateSource(item: UpdateCheckItem) {
  if (!item.sourceUrl) {
    setNotice("warning", "这个 Mod 没有可识别的更新来源。可以手动打开作者页面或 Nexus 页面检查。");
    return;
  }

  try {
    await invoke("open_url_in_browser", { url: item.sourceUrl });
  } catch (error) {
    setNotice("error", `打开更新来源失败：${String(error)}`);
  }
}

async function handleInstallHistoryZipSelect() {
  await handlePreviewZipMod();
}

async function handleInstallSmapi() {
  if (!gamePath.value) {
    setNotice("error", "请先选择游戏目录。");
    return;
  }

  await checkGameFiles(gamePath.value);

  if (!stardewExists.value) {
    setNotice("error", "未找到 Stardew Valley.exe，无法安装 SMAPI。");
    return;
  }

  if (isSmapiInstalling.value) {
    return;
  }

  isSmapiInstalling.value = true;
  smapiInstallerOpened.value = false;
  smapiInstallerVersion.value = "";
  smapiInstallStageMessage.value = "正在读取 SMAPI 下载源...";
  smapiInstallDownloadedBytes.value = 0;
  setNotice("info", smapiInstallStageMessage.value);

  try {
    const result = await invoke<SmapiInstallResult>("install_latest_smapi", {
      gamePath: gamePath.value,
    });

    smapiInstallerOpened.value = true;
    smapiInstallerVersion.value = result.version || "";
    smapiInstallStageMessage.value = "SMAPI 官方安装器已打开。";

    setNotice(
      "success",
      `SMAPI ${result.version} 安装器已打开。请在安装器中完成安装，然后点击"我已完成安装，重新检测"。`
    );
  } catch (error) {
    smapiInstallerOpened.value = false;
    setNotice("error", `安装 SMAPI 失败：${String(error)}`);
  } finally {
    isSmapiInstalling.value = false;
  }
}

async function handleRecheckSmapiInstall() {
  if (!gamePath.value) {
    setNotice("error", "请先选择游戏目录。");
    return;
  }

  await checkGameFiles(gamePath.value);

  if (smapiExists.value) {
    smapiInstallerOpened.value = false;
    await refreshSmapiVersionFromLatestLog();
    setNotice(
      "success",
      smapiDetectedVersion.value
        ? `已检测到 StardewModdingAPI.exe，SMAPI ${smapiDetectedVersion.value} 安装完成。`
        : "已检测到 StardewModdingAPI.exe，SMAPI 安装完成。"
    );
    await scanMods();
    return;
  }

  setNotice(
    "warning",
    "仍未检测到 StardewModdingAPI.exe。请确认 SMAPI 安装器已经完成安装，并且安装到了当前选择的 Stardew Valley 目录。"
  );
}

async function handleRunLaunchCheck() {
  await runLaunchEnvironmentCheck("smapi", true);
}

async function handleLaunchSmapi() {
  const checkResult = await runLaunchEnvironmentCheck("smapi", false);

  if (!checkResult.canLaunch) {
    setNotice("error", formatLaunchIssues("启动前检查未通过", checkResult.errors));
    return;
  }

  try {
    await invoke("launch_game", {
      path: `${gamePath.value}\\StardewModdingAPI.exe`,
    });

    if (checkResult.warnings.length > 0) {
      setNotice(
        "warning",
        `启动前发现 ${checkResult.warnings.length} 个警告，仍正在通过 SMAPI 启动游戏。${formatLaunchIssues("", checkResult.warnings)}`
      );
      return;
    }

    setNotice("success", "启动前检查通过，正在通过 SMAPI 启动游戏...");
  } catch (error) {
    setNotice("error", `启动失败：${String(error)}`);
  }
}

async function handleLaunchVanilla() {
  const checkResult = await runLaunchEnvironmentCheck("vanilla", false);

  if (!checkResult.canLaunch) {
    setNotice("error", formatLaunchIssues("启动前检查未通过", checkResult.errors));
    return;
  }

  try {
    await invoke("launch_game", {
      path: `${gamePath.value}\\Stardew Valley.exe`,
    });

    setNotice("success", "启动前检查通过，正在启动原版 Stardew Valley...");
  } catch (error) {
    setNotice("error", `启动失败：${String(error)}`);
  }
}

async function runLaunchEnvironmentCheck(
  target: LaunchTarget,
  showResult: boolean
): Promise<{ canLaunch: boolean; errors: LaunchCheckIssue[]; warnings: LaunchCheckIssue[] }> {
  const issues = await collectLaunchIssues(target);
  const errors = issues.filter((issue) => issue.level === "error");
  const warnings = issues.filter((issue) => issue.level === "warning");

  if (showResult) {
    if (errors.length > 0) {
      setNotice("error", formatLaunchIssues("启动前检查未通过", errors));
    } else if (warnings.length > 0) {
      setNotice("warning", formatLaunchIssues(`启动前发现 ${warnings.length} 个警告`, warnings));
    } else {
      setNotice("success", "启动前环境检查通过，可以启动 SMAPI。");
    }
  }

  return {
    canLaunch: errors.length === 0,
    errors,
    warnings,
  };
}

async function collectLaunchIssues(target: LaunchTarget): Promise<LaunchCheckIssue[]> {
  const issues: LaunchCheckIssue[] = [];

  if (!gamePath.value) {
    issues.push({
      level: "error",
      title: "尚未选择 Stardew Valley 游戏目录。",
    });
    return issues;
  }

  await checkGameFiles(gamePath.value);

  if (!stardewExists.value) {
    issues.push({
      level: "error",
      title: "未找到 Stardew Valley.exe。",
      detail: "请确认当前选择的是 Stardew Valley 安装目录。",
    });
  }

  if (target === "smapi" && !smapiExists.value) {
    issues.push({
      level: "error",
      title: "未找到 StardewModdingAPI.exe。",
      detail: "请先安装 SMAPI，或确认 SMAPI 已安装到当前游戏目录。",
    });
  }

  if (target === "smapi") {
    if (!modsFolderExists.value) {
      issues.push({
        level: "warning",
        title: "未找到 Mods 文件夹。",
        detail: "SMAPI 可以启动，但当前不会加载任何 Mod。",
      });
    } else if (mods.value.length === 0 && disabledMods.value.length === 0) {
      await scanMods();
    }

    if (missingDependencies.value.length > 0) {
      const preview = missingDependencies.value
        .slice(0, 3)
        .map((dependency) => dependency.uniqueId)
        .join("、");

      issues.push({
        level: "warning",
        title: `发现 ${missingDependencies.value.length} 项缺失依赖。`,
        detail: preview ? `例如：${preview}` : undefined,
      });
    }

    if (skippedFolders.value.length > 0) {
      issues.push({
        level: "warning",
        title: `发现 ${skippedFolders.value.length} 个未识别文件夹。`,
        detail: "这些文件夹可能不是有效 Mod，或 manifest.json 读取失败。",
      });
    }

    if (duplicateEnabledUniqueIds.value.length > 0) {
      issues.push({
        level: "warning",
        title: `发现 ${duplicateEnabledUniqueIds.value.length} 个重复 UniqueID。`,
        detail: duplicateEnabledUniqueIds.value.slice(0, 3).join("、"),
      });
    }
  }

  return issues;
}

function formatLaunchIssues(title: string, issues: LaunchCheckIssue[]) {
  const lines = issues
    .slice(0, 5)
    .map((issue) => {
      const detail = issue.detail ? `：${issue.detail}` : "";
      return `\n- ${issue.title}${detail}`;
    })
    .join("");

  const more = issues.length > 5 ? `\n- 还有 ${issues.length - 5} 个问题未显示。` : "";

  return `${title}${lines}${more}`.trim();
}

function getDuplicateUniqueIds(modList: ModInfo[]) {
  const countMap = new Map<string, number>();

  for (const mod of modList) {
    if (!mod.uniqueId) {
      continue;
    }

    countMap.set(mod.uniqueId, (countMap.get(mod.uniqueId) || 0) + 1);
  }

  return Array.from(countMap.entries())
    .filter(([, count]) => count > 1)
    .map(([uniqueId]) => uniqueId)
    .sort((a, b) => a.localeCompare(b));
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
    addToast("error", "请先选择游戏目录。");
    return;
  }

  const from = `${gamePath.value}\\Mods\\${folderName}`;
  const to = `${gamePath.value}\\Disabled Mods\\${folderName}`;

  if (!(await exists(from))) {
    addToast("error", `禁用失败：没有找到 Mod 文件夹：${folderName}`);
    return;
  }

  if (await exists(to)) {
    addToast("error", `禁用失败：Disabled Mods 中已经存在同名文件夹：${folderName}`);
    return;
  }

  try {
    await invoke("move_folder", { from, to });
    selectedModKey.value = "";
    selectedModKeys.value = new Set();
    addToast("success", `已禁用 Mod：${folderName}`, {
      label: "撤销",
      handler: () => { void handleEnableMod(folderName); },
    });
    await scanMods();
  } catch (error) {
    addToast("error", `禁用 Mod 失败：${String(error)}`);
  }
}

async function handleEnableMod(folderName: string) {
  if (!gamePath.value) {
    addToast("error", "请先选择游戏目录。");
    return;
  }

  const from = `${gamePath.value}\\Disabled Mods\\${folderName}`;
  const to = `${gamePath.value}\\Mods\\${folderName}`;

  if (!(await exists(from))) {
    addToast("error", `启用失败：没有找到已禁用的 Mod 文件夹：${folderName}`);
    return;
  }

  if (await exists(to)) {
    addToast("error", `启用失败：Mods 中已经存在同名文件夹：${folderName}`);
    return;
  }

  try {
    await invoke("move_folder", { from, to });
    selectedModKey.value = "";
    selectedModKeys.value = new Set();
    addToast("success", `已启用 Mod：${folderName}`, {
      label: "撤销",
      handler: () => { void handleDisableMod(folderName); },
    });
    await scanMods();
  } catch (error) {
    addToast("error", `启用 Mod 失败：${String(error)}`);
  }
}


async function handleDeleteDisplayedMod(mod: DisplayModInfo) {
  if (!gamePath.value) {
    addToast("error", "请先选择游戏目录。");
    return;
  }

  const confirmed = await showConfirmModal(
    "确认删除 Mod",
    `确定要删除「${mod.name}」吗？\n\nJunimo Box 会把它移动到游戏目录里的 Junimo Box Deleted Mods 文件夹，不会直接永久删除。`
  );

  if (!confirmed) {
    return;
  }

  const sourceRoot = mod.isDisabled ? "Disabled Mods" : "Mods";
  const from = `${gamePath.value}\\${sourceRoot}\\${mod.folderName}`;
  const safeFolderName = mod.folderName.replace(/[<>:"/\\|?*]/g, "_");
  const timestamp = new Date()
    .toISOString()
    .replace(/[:.]/g, "-");
  const to = `${gamePath.value}\\Junimo Box Deleted Mods\\${safeFolderName}-${timestamp}`;

  try {
    if (!(await exists(from))) {
      addToast("error", `删除失败：没有找到 Mod 文件夹：${mod.folderName}`);
      return;
    }

    await invoke("move_folder", { from, to });

    if (selectedMod.value && getModKey(selectedMod.value) === getModKey(mod)) {
      selectedModKey.value = "";
    }
    selectedModKeys.value = new Set();

    addToast("success", `已删除 Mod：${mod.name}`, {
      label: "撤销",
      handler: () => {
        const recycleFolderName = `${safeFolderName}-${timestamp}`;
        void handleRestoreDeletedMod(recycleFolderName);
      },
    });
    await scanMods();
  } catch (error) {
    addToast("error", `删除 Mod 失败：${String(error)}`);
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
        void previewZipPath(zipPath, "drag");
      }
    });
  } catch (error) {
    console.warn("注册 ZIP 拖拽事件失败", error);
    setNotice("warning", "ZIP 拖拽功能不可用，请使用「选择 ZIP 文件」按钮。");
  }
}

function parseNxmLink(link: string): ParsedNxmRequest {
  const empty: ParsedNxmRequest = {
    raw: link,
    gameDomain: "",
    modId: "",
    fileId: "",
    key: "",
    expires: "",
    userId: "",
    nexusPageUrl: "",
  };

  const trimmed = link.trim();

  if (!trimmed) {
    return empty;
  }

  try {
    const withoutScheme = trimmed.replace(/^nxm:\/\//i, "");
    const [pathPart, queryPart = ""] = withoutScheme.split("?");
    const parts = pathPart.split("/").filter(Boolean);
    const params = new URLSearchParams(queryPart);

    const gameDomain = parts[0] ?? "";
    const modsIndex = parts.findIndex((part) => part.toLowerCase() === "mods");
    const filesIndex = parts.findIndex((part) => part.toLowerCase() === "files");

    const modId = modsIndex >= 0 ? parts[modsIndex + 1] ?? "" : "";
    const fileId = filesIndex >= 0 ? parts[filesIndex + 1] ?? "" : "";

    let nexusPageUrl = "";

    if (gameDomain && modId) {
      nexusPageUrl = `https://www.nexusmods.com/${encodeURIComponent(gameDomain)}/mods/${encodeURIComponent(modId)}`;

      if (fileId) {
        nexusPageUrl += `?tab=files&file_id=${encodeURIComponent(fileId)}`;
      }
    }

    return {
      raw: trimmed,
      gameDomain,
      modId,
      fileId,
      key: params.get("key") || "",
      expires: params.get("expires") || "",
      userId: params.get("user_id") || "",
      nexusPageUrl,
    };
  } catch {
    return empty;
  }
}

function showNxmRequest(link: string, autoDownload = false) {
  const trimmed = link.trim();

  if (!trimmed) {
    return;
  }

  if (!trimmed.toLowerCase().startsWith("nxm:")) {
    message.value = "这不是有效的 nxm:// 链接。";
    return;
  }

  nxmRequestLink.value = trimmed;
  nxmManualInput.value = "";
  nxmDownloadMessage.value = "";
  activeView.value = "tools";
  setNotice("info", "已接收到 Nexus NXM 下载请求。");

  if (autoDownload) {
    void handleDownloadNxmRequest();
  }
}

async function handleRegisterNxmProtocol() {
  try {
    await invoke("register_nxm_protocol");
    nxmProtocolStatus.value = "已关联 NXM 协议。之后在 Nexus 点击 Mod Manager Download 时，Windows 会尝试用 Junimo Box 打开。";
    setNotice("success", "已关联 NXM 协议。");
  } catch (error) {
    nxmProtocolStatus.value = `关联 NXM 协议失败：${String(error)}`;
    setNotice("error", nxmProtocolStatus.value);
  }
}

function loadNexusApiKey() {
  const savedKey = localStorage.getItem(NEXUS_API_KEY_STORAGE_KEY) || "";
  nexusApiKey.value = savedKey;
  nexusApiKeyDraft.value = savedKey;

  if (savedKey) {
    nexusApiStatus.value = "已保存，建议测试连接";
  }
}

function handleSaveNexusApiKey() {
  const trimmedKey = nexusApiKeyDraft.value.trim();

  if (!trimmedKey) {
    message.value = "请先填写 Nexus Personal API Key。";
    return;
  }

  nexusApiKey.value = trimmedKey;
  nexusApiKeyDraft.value = trimmedKey;
  localStorage.setItem(NEXUS_API_KEY_STORAGE_KEY, trimmedKey);
  nexusApiStatus.value = "已保存，建议测试连接";
  setNotice("success", "已保存 Nexus API Key。NXM 自动下载会使用这个 Key 进行认证。");
}

function handleClearNexusApiKey() {
  nexusApiKey.value = "";
  nexusApiKeyDraft.value = "";
  nexusApiUserName.value = "";
  nexusApiIsPremium.value = false;
  nexusApiStatus.value = "未配置";
  localStorage.removeItem(NEXUS_API_KEY_STORAGE_KEY);
  setNotice("info", "已清除 Nexus API Key。NXM 自动下载将不可用。");
}

async function handleTestNexusApiKey() {
  const trimmedKey = nexusApiKeyDraft.value.trim();

  if (!trimmedKey) {
    message.value = "请先填写 Nexus Personal API Key。";
    return;
  }

  isTestingNexusApiKey.value = true;
  nexusApiStatus.value = "正在测试连接...";

  try {
    const result = await invoke<NexusUserInfo>("test_nexus_api_key", {
      apiKey: trimmedKey,
    });

    nexusApiKey.value = trimmedKey;
    nexusApiKeyDraft.value = trimmedKey;
    localStorage.setItem(NEXUS_API_KEY_STORAGE_KEY, trimmedKey);

    nexusApiUserName.value = result.name || "Nexus 用户";
    nexusApiIsPremium.value = Boolean(result.is_premium);
    nexusApiStatus.value = `已连接：${nexusApiUserName.value}`;
    setNotice("success", `Nexus Mods 连接成功：${nexusApiUserName.value}`);
  } catch (error) {
    nexusApiStatus.value = `连接失败：${String(error)}`;
    setNotice("error", nexusApiStatus.value);
  } finally {
    isTestingNexusApiKey.value = false;
  }
}

function handleParseManualNxm() {
  showNxmRequest(nxmManualInput.value);
}

function closeNxmRequest() {
  if (isNxmDownloading.value) {
    return;
  }

  nxmRequestLink.value = "";
  nxmDownloadMessage.value = "";
}

async function handleOpenNxmNexusPage() {
  const url = parsedNxmRequest.value.nexusPageUrl;

  if (!url) {
    message.value = "无法从这个 NXM 链接解析 Nexus 页面。";
    return;
  }

  try {
    await invoke("open_url_in_browser", { url });
    setNotice("info", "已打开 Nexus 页面。如果自动下载失败，可以在网页中完成下载后回到 Junimo Box 选择 ZIP 安装。");
  } catch (error) {
    message.value = `打开 Nexus 页面失败：${String(error)}`;
  }
}


function formatBytes(bytes: number): string {
  if (bytes === 0) return "0 B";
  const units = ["B", "KB", "MB", "GB"];
  const i = Math.floor(Math.log(bytes) / Math.log(1024));
  return (bytes / Math.pow(1024, i)).toFixed(1) + " " + units[i];
}

function formatSpeed(bytesPerSec: number): string {
  if (bytesPerSec === 0) return "";
  return formatBytes(bytesPerSec) + "/s";
}

function mapStageToStatus(stage: string): DownloadQueueStatus {
  switch (stage) {
    case "connecting": return "connecting";
    case "downloading": return "downloading";
    case "merging": return "merging";
    case "completed": return "completed";
    case "failed": return "failed";
    case "cancelled": return "cancelled";
    default: return "queued";
  }
}

function compareVersions(a: string, b: string): "newer" | "downgrade" | "same" | "unknown" {
  if (!a || !b) return "unknown";
  const pa = a.split(".").map(Number);
  const pb = b.split(".").map(Number);
  for (let i = 0; i < Math.max(pa.length, pb.length); i++) {
    const na = pa[i] || 0;
    const nb = pb[i] || 0;
    if (na > nb) return "newer";
    if (na < nb) return "downgrade";
  }
  return "same";
}

function getVersionDiffLabel(diff: "newer" | "downgrade" | "same" | "unknown"): string {
  switch (diff) {
    case "newer": return "更新";
    case "downgrade": return "降级";
    case "same": return "版本相同";
    default: return "版本未知";
  }
}

function getVersionDiffClass(diff: "newer" | "downgrade" | "same" | "unknown"): string {
  switch (diff) {
    case "newer": return "diff-newer";
    case "downgrade": return "diff-downgrade";
    case "same": return "diff-same";
    default: return "diff-unknown";
  }
}

function addToQueue(id: string, fileName: string, source: "nxm" | "url", sourceUrl: string) {
  downloadQueue.value.push({
    id,
    fileName,
    source,
    sourceUrl,
    status: "queued",
    downloadedBytes: 0,
    totalBytes: 0,
    speedBytesPerSec: 0,
    message: "等待下载...",
    createdAt: new Date().toISOString(),
  });
  saveDownloadQueue();
  isDownloadQueueOpen.value = true;
  void processNextInQueue();
}

async function processNextInQueue() {
  const nextItem = downloadQueue.value.find((item) => item.status === "queued");

  if (!nextItem) return;

  nextItem.status = "connecting";
  nextItem.message = "正在连接...";
  saveDownloadQueue();

  try {
    if (nextItem.source === "url") {
      await invoke<UrlZipDownloadResult>("download_zip_from_url", {
        url: nextItem.sourceUrl,
        gamePath: gamePath.value,
        downloadId: nextItem.id,
      });
    } else if (nextItem.source === "nxm") {
      const savedNexusApiKey = nexusApiKey.value.trim() || nexusApiKeyDraft.value.trim();
      await invoke<UrlZipDownloadResult>("download_nxm_file", {
        nxmLink: nextItem.sourceUrl,
        gamePath: gamePath.value,
        apiKey: savedNexusApiKey || null,
        downloadId: nextItem.id,
      });
    }
  } catch (error) {
    const idx = downloadQueue.value.findIndex((item) => item.id === nextItem.id);
    if (idx !== -1) {
      downloadQueue.value[idx].status = "failed";
      downloadQueue.value[idx].errorMessage = String(error);
      downloadQueue.value[idx].message = String(error);
      saveDownloadQueue();
    }
  }
}

function handleRemoveFromQueue(itemId: string) {
  const idx = downloadQueue.value.findIndex((item) => item.id === itemId);
  if (idx === -1) return;

  if (downloadQueue.value[idx].status === "downloading" || downloadQueue.value[idx].status === "connecting") {
    void invoke("cancel_download", { downloadId: itemId });
  }

  downloadQueue.value.splice(idx, 1);
  saveDownloadQueue();
}

function handleClearCompletedQueue() {
  downloadQueue.value = downloadQueue.value.filter(
    (item) => item.status !== "completed" && item.status !== "failed" && item.status !== "cancelled"
  );
  saveDownloadQueue();
}

function handleRetryDownload(itemId: string) {
  const idx = downloadQueue.value.findIndex((item) => item.id === itemId);
  if (idx === -1) return;

  const item = downloadQueue.value[idx];
  if (item.status !== "failed" && item.status !== "cancelled") return;

  item.status = "queued";
  item.errorMessage = "";
  item.message = "准备重试...";
  item.downloadedBytes = 0;
  saveDownloadQueue();
  void processNextInQueue();
}

async function handleDownloadNxmRequest() {
  if (!gamePath.value) {
    message.value = "请先选择 Stardew Valley 游戏目录，再处理 NXM 下载。";
    return;
  }

  if (!nxmRequestLink.value) {
    message.value = "没有可处理的 NXM 链接。";
    return;
  }

  if (!parsedNxmRequest.value.key || !parsedNxmRequest.value.expires) {
    message.value = "这个 NXM 链接缺少 key 或 expires 参数。请重新从 Nexus 点击 Mod Manager Download。";
    return;
  }

  const savedNexusApiKey = nexusApiKey.value.trim() || nexusApiKeyDraft.value.trim();

  if (!savedNexusApiKey) {
    message.value = "NXM 自动下载需要 Nexus API Key。请先到 设置 → Nexus Mods 保存并测试 API Key。";
    return;
  }

  const downloadId = `nxm-${Date.now()}-${Math.random().toString(36).slice(2, 8)}`;
  const file_name = `nexus-${parsedNxmRequest.value.modId}-${parsedNxmRequest.value.fileId}.zip`;

  addToQueue(downloadId, file_name, "nxm", nxmRequestLink.value);
  nxmRequestLink.value = "";
  closeNxmRequest();
  setNotice("info", `NXM 下载已加入队列`);
}

async function handleChooseDownloadedZipForNxm() {
  closeNxmRequest();
  await handlePreviewZipMod();
}

async function checkStartupNxmLink() {
  try {
    const link = await invoke<string | null>("read_startup_nxm_link");

    if (link) {
      showNxmRequest(link, true);
    }
  } catch (error) {
    console.warn("读取启动 NXM 链接失败", error);
  }
}

async function checkPendingNxmLink() {
  try {
    const link = await invoke<string | null>("read_pending_nxm_link");

    if (link) {
      showNxmRequest(link, true);
    }
  } catch (error) {
    console.warn("读取待处理 NXM 链接失败", error);
  }
}

function handleGlobalKeydown(event: KeyboardEvent) {
  if (event.key === "Escape") {
    if (modalState.value.visible) {
      handleModalCancel();
      return;
    }
    if (selectedModKey.value) {
      closeModDetail();
      return;
    }
    if (isProfileEditorOpen.value) {
      closeProfileEditor();
      return;
    }
    if (zipModPreviews.value.length > 0) {
      closeZipPreview();
      return;
    }
    if (isDownloadQueueOpen.value) {
      isDownloadQueueOpen.value = false;
      return;
    }
  }

  if ((event.ctrlKey || event.metaKey) && event.key === "f") {
    const searchInput = document.querySelector<HTMLInputElement>(".search-box input");
    if (searchInput && activeView.value === "mods") {
      searchInput.focus();
      event.preventDefault();
    }
  }

  if ((event.ctrlKey || event.metaKey) && event.key === "r") {
    if (gamePath.value && !isScanning.value) {
      void scanMods();
      event.preventDefault();
    }
  }

  if ((event.ctrlKey || event.metaKey) && event.key === "a") {
    if (activeView.value === "mods") {
      selectedModKeys.value = new Set(filteredMods.value.map((m) => getModKey(m)));
      event.preventDefault();
    }
  }
}

async function handleDownloadZipFromUrl() {
  if (!gamePath.value) {
    message.value = "请先选择游戏目录。";
    return;
  }

  const url = urlZipInput.value.trim();

  if (!url) {
    message.value = "请先输入 ZIP 下载链接。";
    return;
  }

  if (!url.toLowerCase().startsWith("http://") && !url.toLowerCase().startsWith("https://")) {
    message.value = "请输入以 http:// 或 https:// 开头的 ZIP 下载链接。";
    return;
  }

  const file_name = url.split("/").pop() || "download.zip";
  const downloadId = `url-${Date.now()}-${Math.random().toString(36).slice(2, 8)}`;

  addToQueue(downloadId, file_name, "url", url);
  urlZipInput.value = "";
  setNotice("info", `URL 下载已加入队列`);
}

async function handleListDeletedMods() {
  if (!gamePath.value) return;
  isRecycleBinLoading.value = true;
  try {
    deletedMods.value = await invoke<DeletedModInfo[]>("list_deleted_mods", {
      gamePath: gamePath.value,
    });
  } catch (error) {
    message.value = `扫描回收站失败：${String(error)}`;
  } finally {
    isRecycleBinLoading.value = false;
  }
}

async function handleRestoreDeletedMod(folderName: string) {
  isRestoringMap.value[folderName] = true;
  try {
    await invoke<string>("restore_deleted_mod", {
      gamePath: gamePath.value,
      folderName,
    });
    deletedMods.value = deletedMods.value.filter((item) => item.folder_name !== folderName);
    setNotice("info", `Mod 已还原`);
    await scanMods();
  } catch (error) {
    message.value = `还原 Mod 失败：${String(error)}`;
  } finally {
    isRestoringMap.value[folderName] = false;
  }
}

async function handlePermanentlyDeleteMod(folderName: string) {
  const confirmed = await showConfirmModal("永久删除", `确定要永久删除 " ${folderName} " 吗？此操作不可撤销。`);
  if (!confirmed) return;
  try {
    await invoke("permanently_delete_mod", {
      gamePath: gamePath.value,
      folderName,
    });
    deletedMods.value = deletedMods.value.filter((item) => item.folder_name !== folderName);
    setNotice("info", `Mod 已永久删除`);
  } catch (error) {
    message.value = `永久删除失败：${String(error)}`;
  }
}

async function handleEmptyRecycleBin() {
  const confirmed = await showConfirmModal("清空回收站", "确定要清空回收站吗？所有已删除的 Mod 将被永久删除。");
  if (!confirmed) return;
  try {
    await invoke("empty_recycle_bin", { gamePath: gamePath.value });
    deletedMods.value = [];
    setNotice("info", "回收站已清空");
  } catch (error) {
    message.value = `清空回收站失败：${String(error)}`;
  }
}

async function handleExportBackup() {
  if (!gamePath.value) return;
  const backupPath = await save({
    defaultPath: `junimo-box-mods-backup-${new Date().toISOString().slice(0, 10)}.json`,
    filters: [{ name: "JSON 备份文件", extensions: ["json"] }],
  });
  if (!backupPath) return;
  try {
    const result = await invoke<string>("export_mods_backup", {
      gamePath: gamePath.value,
      backupPath,
    });
    setNotice("info", result);
  } catch (error) {
    message.value = `导出备份失败：${String(error)}`;
  }
}

async function handleImportBackup() {
  if (!gamePath.value) return;
  const selected = await open({
    directory: false,
    multiple: false,
    title: "选择备份文件",
    filters: [{ name: "JSON 备份文件", extensions: ["json"] }],
  });
  if (typeof selected !== "string") return;
  try {
    const result = await invoke<string>("import_mods_backup", {
      gamePath: gamePath.value,
      backupPath: selected,
    });
    setNotice("info", result);
    await scanMods();
  } catch (error) {
    message.value = `还原备份失败：${String(error)}`;
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

  await previewZipPath(selected, "local");
}

async function previewZipPath(zipPath: string, source: ZipInstallSource = selectedZipInstallSource.value || "local") {
  if (!zipPath.toLowerCase().endsWith(".zip")) {
    selectedZipPath.value = zipPath;
    zipModPreviews.value = [];
    activeView.value = "tools";
    message.value = "请选择 .zip 格式的 Mod 压缩包。";
    return;
  }

  selectedZipPath.value = zipPath;
  selectedZipInstallSource.value = source;
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

function closeZipPreview() {
  selectedZipPath.value = "";
  selectedZipInstallSource.value = "local";
  zipModPreviews.value = [];
}


async function handleInstallZipMod(conflictMode: ZipInstallConflictMode = "cancel") {
  if (!gamePath.value) {
    message.value = "请先选择游戏目录。";
    return;
  }

  if (!selectedZipPath.value || zipModPreviews.value.length === 0) {
    message.value = "请先预览 ZIP Mod。";
    return;
  }

  if (hasZipInstallConflicts.value && conflictMode === "cancel") {
    message.value = `检测到 ${zipInstallConflicts.value.length} 个已安装 Mod。请选择"跳过已有"或"替换 / 更新"。`;
    return;
  }

  const installingZipPath = selectedZipPath.value;
  const installingSource = selectedZipInstallSource.value;

  try {
    const installedMods = await invoke<ZipModPreview[]>("install_zip_mods", {
      zipPath: installingZipPath,
      gamePath: gamePath.value,
      conflictMode,
    });

    lastInstalledZipMods.value = installedMods;
    selectedZipPath.value = "";
    selectedZipInstallSource.value = "local";
    zipModPreviews.value = [];

    await checkGameFiles(gamePath.value);
    await scanMods();

    const modeNoteMap: Record<ZipInstallConflictMode, string> = {
      cancel: "正常安装",
      skip: "已跳过目标文件夹已存在的 Mod",
      replace: "已替换 / 更新目标文件夹已存在的 Mod",
    };

    addInstallHistory(installedMods, installingSource, conflictMode, installingZipPath, modeNoteMap[conflictMode]);

    message.value =
      missingDependencies.value.length > 0
        ? `安装完成：已处理 ${installedMods.length} 个 Mod，但发现 ${missingDependencies.value.length} 项缺失依赖。`
        : `安装完成：已处理 ${installedMods.length} 个 Mod，依赖检查正常。`;

    activeView.value = "mods";
  } catch (error) {
    message.value = `安装 ZIP Mod 失败：${String(error)}`;
  }
}

async function refreshSmapiVersionFromLatestLog() {
  try {
    const result = await invoke<string[]>("read_latest_smapi_log");
    const content = result[1] || "";
    const analysis = analyzeSmapiLog(content);

    smapiDetectedVersion.value = analysis.smapiVersion || "";
  } catch {
    smapiDetectedVersion.value = "";
  }
}

async function handleReadLatestSmapiLog() {
  try {
    const result = await invoke<string[]>("read_latest_smapi_log");

    smapiLogFileName.value = result[0] || "未知日志文件";
    smapiLogContent.value = result[1] || "";
    smapiLogAnalysis.value = analyzeSmapiLog(smapiLogContent.value);
    smapiDetectedVersion.value = smapiLogAnalysis.value.smapiVersion || smapiDetectedVersion.value;
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

function getZipConflictForPreview(mod: ZipModPreview): ZipInstallConflictRow | undefined {
  return zipInstallConflicts.value.find(
    (row) => row.preview.manifest_path === mod.manifest_path || row.preview.suggested_folder === mod.suggested_folder
  );
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

function escapeHtml(text: string): string {
  return text.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;");
}

function highlightText(text: string, query: string): string {
  if (!query || !text) return escapeHtml(text || "");
  const escaped = escapeHtml(text);
  const needle = query.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
  const regex = new RegExp(`(${needle})`, "gi");
  return escaped.replace(regex, "<mark class='hl'>$1</mark>");
}

function getModKey(mod: DisplayModInfo): string {
  return `${mod.isDisabled ? "disabled" : "enabled"}-${mod.uniqueId || mod.folderName}`;
}

function selectMod(mod: DisplayModInfo, event?: MouseEvent) {
  const key = getModKey(mod);
  if (event?.ctrlKey || event?.metaKey) {
    const keys = new Set(selectedModKeys.value);
    if (keys.has(key)) {
      keys.delete(key);
    } else {
      keys.add(key);
    }
    selectedModKeys.value = keys;
    selectedModKey.value = "";
    return;
  }
  if (event?.shiftKey && lastSelectedModIndex.value >= 0) {
    const keys = new Set(selectedModKeys.value);
    const currentIdx = filteredMods.value.findIndex((m) => getModKey(m) === key);
    const start = Math.min(lastSelectedModIndex.value, currentIdx);
    const end = Math.max(lastSelectedModIndex.value, currentIdx);
    for (let i = start; i <= end; i++) {
      keys.add(getModKey(filteredMods.value[i]));
    }
    selectedModKeys.value = keys;
    selectedModKey.value = "";
    return;
  }
  selectedModKey.value = key;
  selectedModKeys.value = new Set([key]);
  lastSelectedModIndex.value = filteredMods.value.findIndex((m) => getModKey(m) === key);
}

function closeModDetail() {
  selectedModKey.value = "";
  selectedModKeys.value = new Set();
}

function isSelectedMod(mod: DisplayModInfo): boolean {
  return selectedModKeys.value.has(getModKey(mod));
}

function getSelectedMods(): DisplayModInfo[] {
  return filteredMods.value.filter((m) => selectedModKeys.value.has(getModKey(m)));
}

async function handleBatchEnable() {
  const toEnable = getSelectedMods().filter((m) => m.isDisabled);
  if (toEnable.length === 0) {
    addToast("info", "所选 Mod 中没有已禁用的 Mod。");
    return;
  }
  for (const mod of toEnable) {
    await handleEnableMod(mod.folderName);
  }
  selectedModKeys.value = new Set();
}

async function handleBatchDisable() {
  const toDisable = getSelectedMods().filter((m) => !m.isDisabled);
  if (toDisable.length === 0) {
    addToast("info", "所选 Mod 中没有已启用的 Mod。");
    return;
  }
  for (const mod of toDisable) {
    await handleDisableMod(mod.folderName);
  }
  selectedModKeys.value = new Set();
}

async function handleBatchDelete() {
  const selected = getSelectedMods();
  if (selected.length === 0) return;

  const confirmed = await showConfirmModal(
    "批量删除 Mod",
    `确定要删除选中的 ${selected.length} 个 Mod 吗？\n\nJunimo Box 会把它们移动到游戏目录里的回收站文件夹，不会直接永久删除。`
  );

  if (!confirmed) return;

  for (const mod of selected) {
    void handleDeleteDisplayedMod(mod);
  }
  selectedModKeys.value = new Set();
}

function handleWizardNext() {
  wizardStep.value = Math.min(wizardStep.value + 1, 3);
}

async function handleWizardSelectPath() {
  await handleSelectPath();
  if (gamePath.value) {
    wizardStep.value = Math.max(wizardStep.value, 1);
  }
}

async function handleWizardInstallSmapi() {
  await handleInstallSmapi();
  await checkGameFiles(gamePath.value);
  if (smapiExists.value) {
    wizardStep.value = Math.max(wizardStep.value, 2);
  }
}

async function handleWizardScanMods() {
  await scanMods();
  wizardStep.value = Math.max(wizardStep.value, 3);
}

function handleWizardFinish() {
  localStorage.setItem("junimo-box-setup-complete", "true");
  showWizard.value = false;
}

function handleWizardSkip() {
  localStorage.setItem("junimo-box-setup-complete", "true");
  showWizard.value = false;
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

function normalizeUpdateKeys(rawUpdateKeys: unknown): string[] {
  if (!Array.isArray(rawUpdateKeys)) {
    return [];
  }

  return rawUpdateKeys
    .map((key) => String(key || "").trim())
    .filter(Boolean);
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
  color: var(--text-primary, #2d241b);
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
  background: var(--bg-surface, #fffaf0);
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
  background: var(--bg-surface, #fffaf0);
  color: #3f2b1d;
}

.sidebar-footer {
  margin-top: auto;
  padding: 12px;
}

.sidebar-launch-button {
  width: 100%;
  padding: 10px 16px;
  border: none;
  border-radius: var(--radius-button, 8px);
  background: linear-gradient(180deg, var(--green-bg, #6fa85f), #5b914e);
  color: #fff;
  font-size: 14px;
  font-weight: 700;
  cursor: pointer;
  transition: background 0.15s, transform 0.12s;
  text-shadow: 0 1px 2px rgba(0, 0, 0, 0.2);
}

.sidebar-launch-button:hover:not(:disabled) {
  background: linear-gradient(180deg, #5b914e, #4a7d3e);
  transform: scale(1.02);
}

.sidebar-launch-button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.content {
  min-width: 0;
  height: 100%;
  min-height: 0;
  overflow: hidden;
  padding: 24px 28px;
  box-sizing: border-box;
  display: flex;
  flex-direction: column;
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
  color: var(--text-secondary, #7a6652);
  line-height: 1.45;
}

.eyebrow {
  color: var(--text-gold, #8b6f47) !important;
  font-weight: 800;
  letter-spacing: 0.08em;
  text-transform: uppercase;
  font-size: 12px;
}

.view-stack {
  display: flex;
  flex-direction: column;
  gap: 16px;
  min-height: 0;
}

.notice,
.panel,
.empty-state {
  border-radius: var(--radius-panel, 12px);
  background: rgba(255, 250, 240, 0.92);
  box-shadow: 0 10px 28px rgba(67, 47, 27, 0.09);
}

.notice {
  margin-bottom: 16px;
  padding: 13px 18px;
  color: var(--warning-text, #7a4f22);
  font-weight: 800;
  display: flex;
  align-items: center;
  gap: 9px;
  border: 1px solid rgba(122, 79, 34, 0.1);
}

.notice-icon {
  flex-shrink: 0;
}

.notice-success {
  background: var(--green-light, #e8f3df);
  color: #2f6f3c;
}

.notice-info {
  background: rgba(255, 250, 240, 0.92);
  color: var(--warning-text, #7a4f22);
}

.notice-warning {
  background: var(--warning-bg, #f8e7c8);
  color: var(--warning-text, #7a4f22);
}

.notice-error {
  background: var(--danger-light, #f7dfd8);
  color: var(--danger-text, #8f2f22);
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
  color: var(--text-secondary, #7a6652);
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
  color: var(--text-secondary, #7a6652);
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
  background: var(--bg-card, #f6ead8);
}

.status-card span,
.summary-row span,
.setting-block span {
  display: block;
  color: var(--text-secondary, #7a6652);
  font-size: 13px;
  margin-bottom: 6px;
}

.status-card strong,
.summary-row strong,
.setting-block strong {
  word-break: break-all;
}

.status-card small {
  display: block;
  margin-top: 5px;
  color: var(--text-secondary, #7a6652);
  font-size: 12px;
  font-weight: 700;
}

.summary-row {
  grid-template-columns: repeat(4, minmax(0, 1fr));
}

.filter-panel {
  padding: 16px;
}

.batch-action-bar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  flex-wrap: wrap;
  gap: 10px;
  padding: 12px 16px;
  margin-bottom: 12px;
  border-radius: var(--radius-panel, 12px);
  background: rgba(111, 168, 95, 0.1);
  border: 2px solid rgba(111, 168, 95, 0.3);
}

.batch-count {
  font-weight: 600;
  font-size: 14px;
  color: var(--green-text, #2f7d3e);
}

.batch-actions {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
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
  background: var(--bg-card, #f6ead8);
}

.search-box input {
  width: 100%;
  border: none;
  outline: none;
  background: transparent;
  color: var(--text-primary, #2d241b);
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
  color: var(--text-secondary, #7a6652);
  font-size: 13px;
  font-weight: 800;
}

.filter-chip {
  padding: 7px 11px;
  border-radius: 999px;
  background: #eadcc8;
  color: var(--text-tertiary, #5c4630);
  font-size: 13px;
}

.filter-chip:hover,
.filter-chip.active {
  background: var(--text-gold, #8b6f47);
  color: var(--bg-surface, #fffaf0);
}

.filter-result-text {
  margin: 12px 0 0;
  color: var(--text-secondary, #7a6652);
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
  background: var(--bg-card, #f6ead8);
}

.mod-item.disabled {
  opacity: 0.74;
}

.mod-item.warning {
  background: var(--warning-bg, #f8e7c8);
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
  color: var(--green-text, #2f8f46);
}

.disabled-badge {
  background: #e5d6c2;
  color: var(--text-secondary, #7a6652);
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
  color: var(--text-secondary, #7a6652);
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
  color: var(--text-secondary, #7a6652);
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
  color: var(--text-tertiary, #5c4630);
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
  color: var(--text-tertiary, #5c4630);
}

.missing-list {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.missing-item {
  padding: 12px;
  border-radius: 12px;
  background: var(--danger-light, #f7dfd8);
  color: #6f2d20;
}

.missing-item p {
  margin: 6px 0 0;
  font-size: 14px;
}

.tool-grid {
  grid-template-columns: repeat(2, minmax(0, 1fr));
}

.toolbox-grid {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 16px;
}

.tool-section-card {
  padding: 18px;
}

.tool-section-header {
  display: flex;
  align-items: flex-start;
  gap: 12px;
  margin-bottom: 16px;
}

.tool-section-icon {
  width: 44px;
  height: 44px;
  flex: 0 0 auto;
  display: grid;
  place-items: center;
  border-radius: 15px;
  background: var(--bg-card, #f6ead8);
  font-size: 22px;
}

.tool-section-header h3 {
  margin: 0 0 5px;
  font-size: 21px;
}

.tool-section-header p {
  margin: 0;
  color: var(--text-secondary, #7a6652);
  font-size: 14px;
  line-height: 1.45;
}

.tool-section-actions {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 10px;
}

.tool-status-row {
  display: flex;
  justify-content: space-between;
  gap: 12px;
  margin: 12px 0;
  padding: 12px 14px;
  border-radius: 14px;
  background: var(--bg-card, #f6ead8);
}

.tool-status-row span {
  color: var(--text-secondary, #7a6652);
}

.tool-section-note {
  margin: 12px 0 0;
  color: var(--text-secondary, #7a6652);
  font-size: 13px;
  line-height: 1.5;
}

.tool-action-button {
  min-height: 42px;
}

.zip-tool-card {
  display: flex;
  flex-direction: column;
  gap: 14px;
}

.zip-tool-card .tool-section-header {
  margin-bottom: 0;
}

.zip-tool-actions {
  display: flex;
  align-items: center;
  gap: 12px;
  flex-wrap: wrap;
}

.zip-tool-hint {
  color: var(--text-secondary, #7a6652);
  font-size: 13px;
  line-height: 1.45;
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
  background: var(--text-primary, #2d241b);
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
  background: var(--bg-card, #f6ead8);
}

.diagnosis-card span {
  display: block;
  margin-bottom: 6px;
  color: var(--text-secondary, #7a6652);
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
  color: var(--text-tertiary, #5c4630);
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
  background: var(--warning-bg, #f8e7c8);
}

.error-box {
  padding: 14px;
  border: none;
  border-radius: 14px;
  background: var(--danger-light, #f7dfd8);
}

.code-text {
  padding: 10px 12px;
  border-radius: 10px;
  background: var(--bg-card, #f6ead8);
  color: var(--text-tertiary, #5c4630);
  font-family: Consolas, "Courier New", monospace;
  word-break: break-all;
}

.small-log {
  max-height: 220px;
  overflow: auto;
  margin: 0;
  padding: 12px;
  border-radius: 12px;
  background: var(--text-primary, #2d241b);
  color: #fff7e8;
  font-family: Consolas, "Courier New", monospace;
  font-size: 12px;
  line-height: 1.5;
  white-space: pre-wrap;
}

.muted-text {
  margin: 0;
  color: var(--text-secondary, #7a6652);
  line-height: 1.5;
}

.path-text {
  word-break: break-all;
}


.zip-preview-overlay {
  position: fixed;
  inset: 0;
  z-index: 56;
  display: grid;
  place-items: center;
  padding: 24px;
  background: rgba(45, 36, 27, 0.3);
  backdrop-filter: blur(2px);
}

.zip-preview-card {
  width: min(880px, calc(100vw - 48px));
  max-height: calc(100vh - 64px);
  overflow: hidden;
  box-sizing: border-box;
  padding: 20px;
  border-radius: 24px;
  background: var(--bg-surface, #fffaf0);
  box-shadow: 0 24px 70px rgba(45, 36, 27, 0.24);
  border: 1px solid rgba(111, 168, 95, 0.35);
  display: flex;
  flex-direction: column;
}

.zip-preview-card-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  gap: 16px;
  margin-bottom: 14px;
}

.zip-preview-card-header h3 {
  margin: 4px 0 6px;
  font-size: 24px;
}

.zip-preview-card-header p {
  margin: 0;
  color: var(--text-secondary, #7a6652);
  line-height: 1.45;
}

.zip-card-path {
  padding: 10px 12px;
  border-radius: 12px;
  background: var(--bg-card, #f6ead8);
}

.zip-card-scroll {
  overflow: auto;
  margin-top: 14px;
  padding-right: 4px;
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.zip-preview-item {
  padding: 15px;
  border-radius: 18px;
  background: var(--bg-card, #f6ead8);
}

.zip-preview-title-row {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  gap: 12px;
}

.zip-preview-title-row h4 {
  margin: 0 0 6px;
  font-size: 18px;
}

.zip-preview-meta-grid {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 10px;
  margin-top: 12px;
}

.zip-preview-meta-grid > div {
  padding: 10px;
  border-radius: 12px;
  background: rgba(255, 250, 240, 0.72);
}

.zip-preview-meta-grid .wide {
  grid-column: 1 / -1;
}

.zip-preview-meta-grid span {
  display: block;
  margin-bottom: 4px;
  color: var(--text-secondary, #7a6652);
  font-size: 12px;
  font-weight: 800;
}

.zip-preview-meta-grid strong {
  display: block;
  color: #4b3a2a;
  font-size: 13px;
  word-break: break-all;
}

.zip-preview-footer {
  display: flex;
  justify-content: flex-end;
  gap: 10px;
  padding-top: 14px;
  margin-top: 14px;
  border-top: 1px solid rgba(92, 70, 48, 0.14);
}

.zip-dependency-summary {
  margin-top: 14px;
  padding: 14px;
  border-radius: 14px;
}

.zip-dependency-summary.is-ok {
  background: var(--green-light, #e8f3df);
  color: #2f6f3c;
}

.zip-dependency-summary.has-warning {
  background: var(--warning-bg, #f8e7c8);
  color: var(--warning-text, #7a4f22);
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
  color: var(--text-tertiary, #5c4630);
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
  border-color: var(--green-bg, #6fa85f);
  background: var(--green-light, #e8f3df);
  transform: translateY(-1px);
}

.zip-drop-icon {
  width: 46px;
  height: 46px;
  flex: 0 0 auto;
  display: grid;
  place-items: center;
  border-radius: 16px;
  background: var(--bg-surface, #fffaf0);
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
  background: var(--green-light, #e8f3df);
}

.install-warning,
.install-success {
  margin-top: 14px;
  padding: 14px;
  border-radius: 14px;
}

.install-warning {
  background: var(--warning-bg, #f8e7c8);
  color: var(--warning-text, #7a4f22);
}

.install-success {
  background: var(--green-light, #e8f3df);
  color: #2f6f3c;
}

.install-warning p,
.install-success p {
  margin: 6px 0 0;
  line-height: 1.5;
}


.mod-detail-panel {
  border: 1px solid rgba(111, 168, 95, 0.28);
}

.detail-subtitle {
  margin: 4px 0 0;
  color: var(--text-secondary, #7a6652);
  font-size: 14px;
}

.detail-layout {
  display: grid;
  grid-template-columns: minmax(0, 1fr) 180px;
  gap: 16px;
}

.detail-badges {
  margin-bottom: 10px;
}

.detail-description {
  margin: 0 0 14px;
  color: #4b3a2a;
  line-height: 1.55;
}

.detail-grid {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 10px;
}

.detail-grid > div {
  padding: 10px 12px;
  border-radius: 13px;
  background: var(--bg-card, #f6ead8);
  min-width: 0;
}

.detail-grid span {
  display: block;
  margin-bottom: 5px;
  color: var(--text-secondary, #7a6652);
  font-size: 12px;
}

.detail-grid strong {
  display: block;
  word-break: break-all;
  font-size: 13px;
}

.detail-side {
  padding: 14px;
  border-radius: 16px;
  background: var(--bg-card, #f6ead8);
  align-self: start;
}

.detail-side h4,
.detail-dependencies h4 {
  margin: 0 0 10px;
}

.detail-actions {
  display: grid;
  gap: 8px;
}

.detail-actions .tiny-button {
  width: 100%;
}

.detail-dependencies {
  margin-top: 16px;
  padding-top: 14px;
  border-top: 1px solid rgba(92, 70, 48, 0.14);
}

.dependency-detail-list {
  display: grid;
  gap: 8px;
}

.dependency-detail-item {
  display: flex;
  justify-content: space-between;
  gap: 12px;
  padding: 10px 12px;
  border-radius: 12px;
  background: var(--bg-card, #f6ead8);
}

.dependency-detail-item span {
  color: var(--text-secondary, #7a6652);
  flex-shrink: 0;
}

.dependency-detail-item strong {
  text-align: right;
  word-break: break-all;
}

.selectable-mod-item {
  cursor: pointer;
  transition: transform 0.15s ease, box-shadow 0.15s ease, outline-color 0.15s ease;
}

.selectable-mod-item:hover {
  transform: translateY(-1px);
  box-shadow: 0 10px 24px rgba(67, 47, 27, 0.1);
}

.selectable-mod-item.selected {
  outline: 2px solid rgba(111, 168, 95, 0.75);
  background: #edf5e4;
}

.right-panel {
  height: 100%;
  padding: 16px 14px;
  box-sizing: border-box;
  overflow: hidden;
  background: rgba(255, 250, 240, 0.62);
  border-left: 1px solid rgba(92, 70, 48, 0.14);
  display: flex;
  flex-direction: column;
  gap: 14px;
}

.launch-card,
.side-card {
  padding: 16px;
  border-radius: var(--radius-card, 10px);
  background: var(--bg-surface, #fffaf0);
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
  color: var(--text-secondary, #7a6652);
  font-size: 13px;
  line-height: 1.45;
  word-break: break-all;
}

.launch-button {
  width: 100%;
  margin-top: 14px;
  padding: 13px 16px;
  border-radius: 15px;
  background: var(--green-bg, #6fa85f);
  font-size: 16px;
  font-weight: 800;
}

.vanilla-button {
  margin-top: 8px;
  background: var(--text-gold, #8b6f47);
}

.smapi-install-button {
  margin-top: 8px;
  background: #9f7d4a;
}

.smapi-install-button:hover:not(:disabled) {
  background: #87693d;
}

.vanilla-button:hover:not(:disabled) {
  background: var(--gold-hover, #755d3c);
}

.info-line {
  display: flex;
  justify-content: space-between;
  gap: 10px;
  padding: 9px 0;
  border-bottom: 1px solid var(--border-subtle, rgba(92, 70, 48, 0.12));
}

.info-line:last-child {
  border-bottom: none;
}

.info-line span {
  color: var(--text-secondary, #7a6652);
}

button {
  padding: 11px 16px;
  border: none;
  border-radius: 13px;
  background: var(--green-bg, #6fa85f);
  color: white;
  font-size: 15px;
  font-weight: 800;
  cursor: pointer;
}

button:hover:not(:disabled) {
  background: var(--green-hover, #5d944f);
}

button:disabled {
  opacity: 0.45;
  cursor: not-allowed;
}

button.secondary {
  background: var(--text-gold, #8b6f47);
}

button.secondary:hover:not(:disabled) {
  background: var(--gold-hover, #755d3c);
}

.compact-header-button {
  white-space: nowrap;
  padding: 10px 15px;
}

.tiny-button {
  padding: 7px 11px;
  border-radius: 999px;
  font-size: 12px;
  background: var(--text-gold, #8b6f47);
}

.tiny-button:hover:not(:disabled) {
  background: var(--gold-hover, #755d3c);
}

.tiny-button.danger {
  background: #b65b4b;
}

.tiny-button.danger:hover:not(:disabled) {
  background: #9f493c;
}

.ok {
  color: var(--green-text, #2f8f46);
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
  color: var(--green-text, #2f8f46);
  font-weight: 800;
}


.mods-view {
  gap: 14px;
}

.filter-summary-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 12px;
  margin-top: 12px;
}

.filter-summary-row .filter-result-text {
  margin: 0;
}

.active-filter-text {
  margin: 0;
  color: var(--green-bg, #6fa85f);
  font-size: 13px;
  font-weight: 800;
}

.mods-alert-grid {
  display: grid;
  grid-template-columns: minmax(0, 1fr) minmax(0, 1fr);
  gap: 14px;
}

.recent-install-list {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}

.recent-install-list span {
  padding: 6px 9px;
  border-radius: 999px;
  background: #e2d1b8;
  color: var(--text-tertiary, #5c4630);
  font-size: 12px;
  font-weight: 800;
}

.compact-result-box {
  margin-top: 12px;
  padding: 12px;
}

.compact-missing-list {
  gap: 8px;
}

.compact-missing-list .missing-item {
  padding: 10px 11px;
}

.compact-missing-list .missing-item p {
  font-size: 13px;
}

.mods-workspace {
  display: grid;
  grid-template-columns: minmax(0, 1fr);
  gap: 16px;
  align-items: start;
}

.mods-list-panel,
.side-detail-panel {
  min-width: 0;
}

.mods-list-panel {
  padding: 18px;
}

.scrollable-mods-list {
  min-height: 0;
  overflow-y: auto;
  overflow-x: hidden;
  padding-right: 4px;
}

.scrollable-mods-list::-webkit-scrollbar,
.side-detail-panel::-webkit-scrollbar {
  width: 8px;
}

.scrollable-mods-list::-webkit-scrollbar-thumb,
.side-detail-panel::-webkit-scrollbar-thumb {
  border-radius: 999px;
  background: var(--border-strong, rgba(92, 70, 48, 0.22));
}

.compact-mod-card {
  padding: 14px;
  align-items: flex-start;
}

.compact-mod-card .mod-title-row {
  align-items: flex-start;
}

.compact-description {
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.compact-card-actions {
  min-width: 84px;
}

.compact-card-actions .tiny-button {
  width: 72px;
}

.mod-detail-overlay {
  position: fixed;
  inset: 0;
  z-index: 55;
  display: grid;
  place-items: center;
  padding: 24px;
  background: rgba(45, 36, 27, 0.3);
  backdrop-filter: blur(2px);
}

.mod-detail-card {
  width: min(780px, calc(100vw - 48px));
  max-height: calc(100vh - 64px);
  overflow: auto;
  box-sizing: border-box;
  padding: 20px;
  border-radius: 24px;
  background: var(--bg-surface, #fffaf0);
  box-shadow: 0 24px 70px rgba(45, 36, 27, 0.24);
  border: 1px solid rgba(111, 168, 95, 0.35);
}

.mod-detail-card-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  gap: 16px;
  margin-bottom: 16px;
}

.mod-detail-card-header h3 {
  margin: 4px 0 6px;
  font-size: 24px;
}

.mod-detail-card-header p {
  margin: 0;
  color: var(--text-secondary, #7a6652);
  line-height: 1.45;
}

.detail-card-grid {
  margin-top: 14px;
}

.side-detail-panel {
  position: sticky;
  top: 0;
  max-height: calc(100vh - 190px);
  overflow-y: auto;
  padding: 18px;
}

.side-detail-panel .panel-header {
  align-items: flex-start;
}

.side-detail-panel .detail-description {
  margin-bottom: 12px;
}

.detail-actions-inline {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  margin-bottom: 14px;
}

.detail-actions-inline .tiny-button {
  width: auto;
}

.side-detail-grid {
  grid-template-columns: 1fr;
}

.side-dependencies {
  margin-top: 14px;
}

.empty-detail-state {
  min-height: 360px;
  display: grid;
  place-items: center;
  align-content: center;
  text-align: center;
  color: var(--text-secondary, #7a6652);
}

.empty-detail-icon {
  width: 58px;
  height: 58px;
  display: grid;
  place-items: center;
  margin-bottom: 14px;
  border-radius: 20px;
  background: var(--bg-card, #f6ead8);
  font-size: 30px;
}

.empty-detail-state h3 {
  margin: 0 0 8px;
  color: var(--text-primary, #2d241b);
}

.empty-detail-state p {
  max-width: 280px;
  margin: 0;
  line-height: 1.55;
}

@media (max-width: 760px) {
  .detail-layout,
  .detail-grid {
    grid-template-columns: 1fr;
  }
}


.profile-create-row {
  display: grid;
  grid-template-columns: minmax(0, 1fr) auto;
  gap: 12px;
  align-items: center;
  margin-top: 16px;
}

.profile-input {
  width: 100%;
  box-sizing: border-box;
  padding: 12px 14px;
  border: 1px solid rgba(92, 70, 48, 0.18);
  border-radius: 14px;
  background: var(--bg-surface, #fffaf0);
  color: var(--text-primary, #2d241b);
  font-size: 14px;
  outline: none;
}

.profile-input:focus {
  border-color: rgba(111, 168, 95, 0.75);
  box-shadow: 0 0 0 3px rgba(111, 168, 95, 0.14);
}

.profile-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.profile-card {
  display: flex;
  justify-content: space-between;
  gap: 16px;
  padding: 16px;
  border-radius: 18px;
  background: var(--bg-card, #f6ead8);
}

.profile-main {
  min-width: 0;
}

.profile-main h4 {
  margin: 0 0 6px;
  font-size: 18px;
}

.profile-main p {
  margin: 0 0 10px;
  color: var(--text-secondary, #7a6652);
  font-size: 14px;
}

.profile-mod-preview {
  display: flex;
  flex-wrap: wrap;
  gap: 7px;
}

.profile-mod-preview span {
  padding: 5px 8px;
  border-radius: 999px;
  background: #e2d1b8;
  color: var(--text-tertiary, #5c4630);
  font-size: 12px;
  font-weight: 800;
}

.profile-actions {
  flex-shrink: 0;
  display: flex;
  flex-direction: column;
  gap: 8px;
  align-items: flex-end;
}


@media (max-width: 1320px) {
  .mods-workspace {
    grid-template-columns: 1fr;
  }

  .side-detail-panel {
    position: static;
    max-height: none;
  }

  .side-detail-grid {
    grid-template-columns: repeat(2, minmax(0, 1fr));
  }
}

@media (max-width: 980px) {
  .mods-alert-grid {
    grid-template-columns: 1fr;
  }
}


.profile-intro-panel .panel-header {
  align-items: flex-start;
}

.profile-toolbar {
  display: flex;
  flex-wrap: wrap;
  gap: 10px;
  margin-top: 16px;
}

.profile-card-overlay {
  position: fixed;
  inset: 0;
  z-index: 50;
  display: grid;
  place-items: center;
  padding: 24px;
  background: rgba(45, 36, 27, 0.28);
  backdrop-filter: blur(2px);
}

.profile-editor-card {
  width: min(860px, calc(100vw - 48px));
  max-height: calc(100vh - 64px);
  overflow: auto;
  box-sizing: border-box;
  padding: 20px;
  border-radius: 24px;
  background: var(--bg-surface, #fffaf0);
  box-shadow: 0 24px 70px rgba(45, 36, 27, 0.24);
  border: 1px solid rgba(111, 168, 95, 0.35);
}

.profile-card-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  gap: 16px;
  margin-bottom: 16px;
}

.profile-card-header h3 {
  margin: 4px 0 6px;
  font-size: 22px;
}

.profile-card-header p {
  margin: 0;
  color: var(--text-secondary, #7a6652);
  line-height: 1.45;
}

.profile-editor-grid {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 12px;
  margin-bottom: 14px;
}

.profile-field span {
  display: block;
  margin-bottom: 6px;
  color: var(--text-secondary, #7a6652);
  font-size: 13px;
  font-weight: 800;
}

.profile-editor-summary {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 12px;
  padding: 12px 0;
  color: var(--text-secondary, #7a6652);
  font-weight: 800;
}

.profile-editor-actions {
  display: flex;
  gap: 8px;
}

.profile-select-list {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 10px;
  max-height: 360px;
  overflow: auto;
  padding-right: 4px;
}

.profile-select-item {
  display: flex;
  gap: 10px;
  align-items: flex-start;
  padding: 12px;
  border-radius: 15px;
  background: var(--bg-card, #f6ead8);
  border: 1px solid transparent;
  cursor: pointer;
}

.profile-select-item.selected {
  border-color: rgba(111, 168, 95, 0.8);
  background: var(--green-light, #e8f3df);
}

.profile-select-item input {
  margin-top: 4px;
  accent-color: var(--green-bg, #6fa85f);
}

.profile-select-main {
  min-width: 0;
  flex: 1;
}

.profile-select-main strong {
  display: block;
  margin-bottom: 4px;
}

.profile-select-main p,
.profile-select-main span {
  display: block;
  margin: 0;
  color: var(--text-secondary, #7a6652);
  font-size: 12px;
  line-height: 1.35;
  word-break: break-all;
}

.profile-select-tags {
  display: flex;
  flex-direction: column;
  align-items: flex-end;
  gap: 6px;
}

.profile-editor-footer {
  display: flex;
  gap: 10px;
  margin-top: 16px;
}

.profile-link-button {
  margin-top: 8px;
  padding: 0;
  border-radius: 0;
  background: transparent;
  color: var(--warning-text, #7a4f22);
  font-size: 13px;
  font-weight: 800;
}

.profile-link-button:hover:not(:disabled) {
  background: transparent;
  color: #5d3a18;
}

@media (max-width: 1180px) {
  .profile-editor-grid,
  .profile-select-list {
    grid-template-columns: 1fr;
  }
}

@media (max-width: 720px) {
  .profile-card-overlay {
    padding: 12px;
    align-items: stretch;
  }

  .profile-editor-card {
    width: 100%;
    max-height: calc(100vh - 24px);
    border-radius: 20px;
  }

  .profile-card-header {
    flex-direction: column;
  }
}

@media (max-width: 760px) {
  .zip-preview-card {
    width: calc(100vw - 28px);
    max-height: calc(100vh - 28px);
    padding: 16px;
  }

  .zip-preview-card-header,
  .zip-preview-title-row,
  .zip-preview-footer {
    flex-direction: column;
    align-items: stretch;
  }

  .zip-preview-meta-grid {
    grid-template-columns: 1fr;
  }
}



/* v0.2：Mod 列表卡片密度优化 */
.compact-mod-card {
  padding: 12px 13px;
  border-radius: 16px;
  display: block;
  border: 1px solid rgba(92, 70, 48, 0.08);
}

.compact-mod-card.warning {
  border-color: rgba(159, 73, 60, 0.28);
  background: #f9e8d3;
}

.compact-mod-card.disabled {
  opacity: 0.7;
}

.mod-card-content {
  min-width: 0;
}

.mod-card-main-row {
  display: flex;
  justify-content: space-between;
  gap: 12px;
  align-items: flex-start;
}

.mod-title-block {
  min-width: 0;
}

.mod-title-block h4 {
  margin: 0 0 4px;
  font-size: 16px;
  line-height: 1.25;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.compact-mod-card .mod-meta {
  margin: 0;
  font-size: 12px;
}

.compact-badges {
  max-width: 260px;
  gap: 5px;
}

.compact-badges .status-badge {
  padding: 3px 7px;
  font-size: 11px;
}

.compact-mod-card .compact-description {
  margin-top: 8px;
  font-size: 13px;
  line-height: 1.42;
  color: var(--text-tertiary, #5c4630);
  -webkit-line-clamp: 2;
}

.mod-card-footer {
  margin-top: 10px;
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 12px;
}

.folder-chip {
  min-width: 0;
  max-width: 62%;
  padding: 4px 8px;
  border-radius: 999px;
  background: rgba(226, 209, 184, 0.75);
  color: var(--text-folder-chip, #6b5238);
  font-size: 11px;
  font-weight: 800;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.compact-card-actions {
  min-width: auto;
  flex-direction: row;
  align-items: center;
  gap: 6px;
}

.compact-card-actions .tiny-button {
  width: auto;
  min-width: 48px;
  padding: 6px 9px;
  font-size: 11px;
}

.ghost-button {
  background: #a58a68;
}

.ghost-button:hover:not(:disabled) {
  background: var(--text-gold, #8b6f47);
}

.scrollable-mods-list {
  gap: 8px;
}

@media (max-width: 760px) {
  .mod-card-main-row,
  .mod-card-footer {
    flex-direction: column;
    align-items: stretch;
  }

  .folder-chip {
    max-width: 100%;
  }

  .compact-card-actions {
    justify-content: flex-start;
  }
}


.side-check-button {
  width: 100%;
  margin-top: 10px;
  padding: 9px 10px;
  border-radius: 12px;
  background: var(--text-gold, #8b6f47);
  font-size: 13px;
}

.side-check-button:hover:not(:disabled) {
  background: var(--gold-hover, #755d3c);
}

@media (max-width: 1200px) {
  .app-shell {
    grid-template-columns: 205px minmax(0, 1fr);
  }

  .right-panel {
    display: none;
  }
}

@media (max-width: 800px) {
  .sidebar {
    width: 64px;
    padding: 12px 8px;
    align-items: center;
  }

  .sidebar .brand h1,
  .sidebar .brand p,
  .sidebar .nav-button span:last-child,
  .sidebar .sidebar-footer p,
  .sidebar .sidebar-footer strong {
    display: none;
  }

  .sidebar .brand {
    justify-content: center;
    padding: 8px;
  }

  .sidebar .brand-icon {
    margin: 0;
  }

  .sidebar .nav-button {
    justify-content: center;
    padding: 10px;
    font-size: 0;
  }

  .sidebar .nav-button span:first-child {
    font-size: 18px;
  }

  .sidebar .sidebar-footer {
    padding: 4px;
  }

  .sidebar .sidebar-launch-button {
    padding: 8px;
    font-size: 0;
  }

  .sidebar .sidebar-launch-button::after {
    content: "▶";
    font-size: 16px;
  }

  .app-shell {
    grid-template-columns: 64px minmax(0, 1fr);
  }
}

@media (max-width: 820px) {
  .overview-grid,
  .status-grid,
  .summary-row,
  .tool-grid,
  .toolbox-grid,
  .diagnosis-grid {
    grid-template-columns: 1fr;
  }

  .tool-section-actions {
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


/* v0.2.1：统一滚动结构，避免双滚动
   - app-shell / content 不滚动
   - 普通页面在 view-stack 内部滚动
   - Mods 页面只让 Mod 列表内部滚动
   - 右侧栏自己内部滚动
   - 弹出小卡片自己滚动
*/
.content {
  overflow: hidden !important;
  display: flex !important;
  flex-direction: column !important;
  min-height: 0 !important;
}

.content-header,
.notice {
  flex-shrink: 0 !important;
}

.content > .view-stack {
  flex: 1 !important;
  min-height: 0 !important;
}

.content > .view-stack:not(.mods-page-fixed) {
  overflow-y: auto !important;
  overflow-x: hidden !important;
  padding-right: 6px;
}

.content > .mods-page-fixed {
  overflow: hidden !important;
  display: flex !important;
  flex-direction: column !important;
}

.mods-page-fixed > .filter-panel,
.mods-page-fixed > .mods-alert-grid,
.mods-page-fixed > .empty-state {
  flex-shrink: 0 !important;
}

.mods-page-fixed > .mods-workspace {
  flex: 1 !important;
  min-height: 0 !important;
  overflow: hidden !important;
  display: flex !important;
  flex-direction: column !important;
}

.mods-page-fixed .mods-list-panel {
  flex: 1 !important;
  min-height: 0 !important;
  overflow: hidden !important;
  display: flex !important;
  flex-direction: column !important;
}

.mods-page-fixed .mods-list-panel .panel-header {
  flex-shrink: 0 !important;
}

.mods-page-fixed .scrollable-mods-list {
  flex: 1 !important;
  min-height: 0 !important;
  max-height: none !important;
  overflow-y: auto !important;
  overflow-x: hidden !important;
  padding-right: 6px !important;
}

.right-panel {
  min-height: 0 !important;
  overflow-y: auto !important;
  overflow-x: hidden !important;
  overscroll-behavior: contain;
  padding-right: 10px !important;
}

.right-panel::-webkit-scrollbar {
  width: 8px;
}

.right-panel::-webkit-scrollbar-thumb {
  background: rgba(92, 70, 48, 0.28);
  border-radius: 999px;
}

.right-panel::-webkit-scrollbar-track {
  background: transparent;
}

.right-panel .launch-card,
.right-panel .side-card {
  flex-shrink: 0;
}

.right-panel .launch-button,
.right-panel .secondary,
.right-panel button {
  min-height: 42px;
}

/* v0.2.0：Profiles 页面轻量化 */
.profiles-page {
  gap: 14px;
}

.profile-hero-panel {
  padding: 18px 20px;
}

.profile-hero-main {
  display: flex;
  justify-content: space-between;
  gap: 18px;
  align-items: flex-start;
}

.profile-hero-main h3 {
  margin: 4px 0 6px;
  font-size: 22px;
}

.profile-hero-stats {
  flex-shrink: 0;
  display: flex;
  flex-direction: column;
  gap: 8px;
  align-items: flex-end;
}

.profile-hero-stats span,
.experiment-chip {
  padding: 5px 8px;
  border-radius: 999px;
  background: var(--green-light, #e8f3df);
  color: #2f6f3c;
  font-size: 12px;
  font-weight: 800;
  white-space: nowrap;
}

.profile-action-cards {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 12px;
  margin-top: 16px;
}

.profile-action-card {
  width: 100%;
  padding: 14px 16px;
  border-radius: 18px;
  background: var(--bg-card, #f6ead8);
  color: var(--text-primary, #2d241b);
  text-align: left;
  box-shadow: none;
}

.profile-action-card.primary {
  background: var(--green-bg, #6fa85f);
  color: var(--bg-surface, #fffaf0);
}

.profile-action-card strong,
.profile-action-card span {
  display: block;
}

.profile-action-card strong {
  margin-bottom: 4px;
  font-size: 15px;
}

.profile-action-card span {
  color: inherit;
  opacity: 0.82;
  font-size: 12px;
  line-height: 1.4;
}

.profile-list-light {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.light-profile-card {
  display: block;
  padding: 14px 16px;
  border-radius: 18px;
  background: var(--bg-surface, #fffaf0);
}

.profile-card-top {
  display: flex;
  justify-content: space-between;
  gap: 14px;
  align-items: flex-start;
}

.profile-title-row {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 4px;
}

.profile-title-row h4 {
  margin: 0;
  font-size: 17px;
}

.profile-main p {
  margin: 0;
  font-size: 13px;
  color: var(--text-secondary, #7a6652);
}

.compact-profile-actions {
  flex-direction: row;
  align-items: center;
  gap: 6px;
}

.compact-profile-actions .tiny-button {
  min-width: 44px;
  padding: 6px 9px;
  font-size: 11px;
}

.profile-card-bottom {
  margin-top: 10px;
  padding-top: 10px;
  border-top: 1px solid var(--border-subtle, rgba(92, 70, 48, 0.12));
}

.profile-preview-inline {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
  margin-top: 8px;
}

.profile-preview-inline span,
.expanded-profile-preview span {
  padding: 4px 7px;
  border-radius: 999px;
  background: #f0dfc7;
  color: var(--text-tertiary, #5c4630);
  font-size: 11px;
  font-weight: 800;
  max-width: 180px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.expanded-profile-preview {
  max-height: 180px;
  overflow: auto;
  padding-right: 4px;
}

.compact-profile-editor {
  width: min(760px, calc(100vw - 48px));
  max-height: calc(100vh - 72px);
  padding: 18px;
}

.compact-editor-summary {
  padding: 10px 0;
}

.compact-profile-select-list {
  max-height: 320px;
}

.compact-profile-select-item {
  padding: 10px;
}

.profile-empty-state {
  padding: 28px;
}

@media (max-width: 820px) {
  .profile-hero-main,
  .profile-card-top {
    flex-direction: column;
  }

  .profile-hero-stats,
  .compact-profile-actions {
    align-items: flex-start;
  }

  .profile-action-cards {
    grid-template-columns: 1fr;
  }
}


.secondary-action {
  background: var(--text-gold, #8b6f47);
}

.secondary-action:hover:not(:disabled) {
  background: var(--gold-hover, #755d3c);
}

.smapi-recheck-button {
  background: var(--text-gold, #8b6f47);
}

.smapi-recheck-button:hover:not(:disabled) {
  background: var(--gold-hover, #755d3c);
}


/* v0.3.0：Profiles 实用操作按钮 */
.profile-actions.compact-profile-actions {
  flex-wrap: wrap;
  justify-content: flex-end;
  max-width: 360px;
}

.profile-actions.compact-profile-actions .tiny-button {
  min-width: 52px;
}

/* v0.4.0：从链接安装 ZIP */
.url-zip-box {
  margin-top: 14px;
  padding: 14px;
  border-radius: 16px;
  background: var(--bg-card, #f6ead8);
  border: 1px solid var(--border-subtle, rgba(92, 70, 48, 0.12));
}

.url-zip-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 12px;
  margin-bottom: 10px;
}

.url-zip-header strong {
  font-size: 15px;
}

.url-zip-header span {
  color: var(--text-secondary, #7a6652);
  font-size: 12px;
  font-weight: 800;
}

.url-zip-form {
  display: grid;
  grid-template-columns: minmax(0, 1fr) auto;
  gap: 10px;
  align-items: center;
}

.url-zip-input {
  width: 100%;
  min-width: 0;
  box-sizing: border-box;
  border: 1px solid var(--border-strong, rgba(92, 70, 48, 0.22));
  border-radius: 13px;
  padding: 11px 13px;
  background: var(--bg-surface, #fffaf0);
  color: var(--text-primary, #2d241b);
  font-size: 14px;
}

.url-zip-input:disabled {
  opacity: 0.65;
}

.url-zip-button {
  white-space: nowrap;
}

.url-zip-status {
  margin-top: 10px !important;
}

@media (max-width: 760px) {
  .url-zip-form {
    grid-template-columns: 1fr;
  }
}

/* v0.3.0：修复 Profile 编辑小卡片双滚动 */
.profile-card-overlay {
  overflow: hidden !important;
}

.profile-editor-card,
.compact-profile-editor {
  max-height: calc(100vh - 48px) !important;
  overflow: hidden !important;
  display: flex !important;
  flex-direction: column !important;
  min-height: 0 !important;
}

.profile-card-header,
.profile-editor-grid,
.profile-editor-summary,
.profile-editor-footer {
  flex-shrink: 0 !important;
}

.profile-select-list,
.compact-profile-select-list {
  flex: 1 !important;
  min-height: 0 !important;
  max-height: none !important;
  overflow-y: auto !important;
  overflow-x: hidden !important;
  padding-right: 6px !important;
}

.profile-editor-footer {
  margin-top: 12px !important;
  padding-top: 12px !important;
  border-top: 1px solid var(--border-subtle, rgba(92, 70, 48, 0.12)) !important;
  background: var(--bg-surface, #fffaf0) !important;
}

@media (max-width: 820px) {
  .profile-editor-card,
  .compact-profile-editor {
    width: calc(100vw - 24px) !important;
    max-height: calc(100vh - 24px) !important;
  }

  .profile-editor-grid {
    grid-template-columns: 1fr !important;
  }
}


/* v0.4.0：安装结果改为轻量行内提示，避免顶开 Mod 列表 */
.inline-install-summary,
.inline-dependency-summary {
  margin-top: 10px;
  padding: 9px 12px;
  border-radius: 14px;
  font-size: 13px;
  font-weight: 700;
}

.inline-install-summary {
  background: rgba(229, 245, 219, 0.9);
  border: 1px solid rgba(111, 168, 95, 0.22);
  color: var(--green-text, #2f7d3e);
}

.inline-install-summary span {
  color: #4f7d48;
}

.inline-install-summary strong {
  color: #1f6d34;
}

.inline-dependency-summary {
  background: rgba(255, 243, 205, 0.9);
  border: 1px solid rgba(161, 119, 55, 0.22);
  color: #8a5a18;
}

.inline-dependency-summary span {
  color: #8a6a3d;
}

.inline-dependency-summary strong {
  color: #7a4b11;
}


/* v0.5.0：NXM 协议入口 */
.nxm-box {
  margin-top: 14px;
  padding: 14px;
  border-radius: 18px;
  background: rgba(255, 250, 240, 0.78);
  border: 1px solid var(--border-subtle, rgba(92, 70, 48, 0.12));
}

.nxm-action-row {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  margin-top: 10px;
}

.nxm-manual-form {
  margin-top: 10px;
}

.nxm-request-card {
  max-width: 680px;
}

.nxm-detail-grid {
  display: grid;
  grid-template-columns: repeat(3, minmax(0, 1fr));
  gap: 10px;
  margin: 14px 0;
}

.nxm-detail-grid > div {
  padding: 12px;
  border-radius: 16px;
  background: rgba(255, 250, 240, 0.9);
  border: 1px solid var(--border-subtle, rgba(92, 70, 48, 0.12));
}

.nxm-detail-grid span {
  display: block;
  margin-bottom: 4px;
  color: rgba(92, 70, 48, 0.62);
  font-size: 12px;
  font-weight: 800;
}

.nxm-detail-grid strong {
  color: #4a3222;
  font-size: 14px;
}

@media (max-width: 820px) {
  .nxm-detail-grid {
    grid-template-columns: 1fr;
  }
}


/* v0.5.2：Nexus API Key 设置 */
.nexus-key-form {
  display: grid;
  grid-template-columns: minmax(0, 1fr) auto;
  gap: 10px;
  align-items: center;
  margin-top: 12px;
}

.nexus-setting-actions {
  display: flex;
  flex-wrap: wrap;
  gap: 10px;
}

.nexus-status-block {
  margin-top: 14px;
}

.nexus-status-block small {
  display: block;
  margin-top: 6px;
  color: var(--text-secondary, #7a6652);
  font-size: 12px;
  font-weight: 800;
}



/* v0.6.0: 安装更新安全、历史、当前 Profile、更新来源 */
.zip-conflict-summary {
  margin: 12px 0;
  padding: 12px 14px;
  border-radius: 16px;
  border: 1px solid rgba(198, 126, 44, 0.25);
  background: rgba(255, 239, 205, 0.92);
  color: #7a4a16;
}

.zip-conflict-summary p {
  margin: 4px 0 0;
  font-size: 13px;
}

.zip-conflict-line {
  margin: 8px 0;
  padding: 7px 9px;
  border-radius: 12px;
  background: rgba(255, 239, 205, 0.85);
  color: #8a5317;
  font-size: 12px;
  font-weight: 700;
}

.current-profile-chip {
  background: rgba(111, 168, 95, 0.18);
  color: var(--green-text, #2f7d3e);
}

.two-column-tool-grid {
  align-items: stretch;
}

.history-list,
.update-check-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
  margin: 12px 0;
}

.history-item,
.update-check-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  padding: 10px 12px;
  border-radius: 14px;
  background: rgba(255, 250, 240, 0.78);
  border: 1px solid rgba(92, 70, 48, 0.1);
}

.history-item p,
.update-check-item p {
  margin: 4px 0 0;
  color: var(--text-muted, #7a634a);
  font-size: 12px;
}

.current-profile-side-card .side-check-button {
  margin-top: 10px;
}



/* v0.6.1：更新检测结果改为卡片内部滚动，避免撑开工具箱布局 */
.two-column-tool-grid {
  align-items: start !important;
}

.install-history-card,
.update-check-card {
  min-height: 0 !important;
  align-self: start !important;
}

.update-check-summary {
  margin-bottom: 8px !important;
}

.history-list.compact-scroll-list,
.update-check-list.compact-scroll-list,
.update-check-card .update-check-list,
.install-history-card .history-list {
  max-height: 260px !important;
  overflow-y: auto !important;
  overflow-x: hidden !important;
  padding-right: 6px !important;
}

.update-check-card .update-check-list::-webkit-scrollbar,
.install-history-card .history-list::-webkit-scrollbar {
  width: 8px;
}

.update-check-card .update-check-list::-webkit-scrollbar-thumb,
.install-history-card .history-list::-webkit-scrollbar-thumb {
  border-radius: 999px;
  background: var(--border-strong, rgba(92, 70, 48, 0.22));
}

.update-check-item,
.history-item {
  align-items: flex-start !important;
}

.update-check-item > div,
.history-item > div {
  min-width: 0;
}

.update-check-item strong,
.history-item strong {
  display: block;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.update-check-item .tiny-button {
  flex-shrink: 0;
}

.delete-mod-button {
  background: var(--danger-bg, #b9574f) !important;
  color: var(--bg-surface, #fffaf0) !important;
}

.delete-mod-button:hover {
  filter: brightness(0.96);
}

.compact-card-actions {
  flex-wrap: wrap;
  justify-content: flex-end;
}

.compact-card-actions .tiny-button {
  width: auto !important;
  min-width: 52px;
}



/* v0.6.1：Profiles 操作区重排，区分"实际切换"和"仅标记" */
.profile-action-groups {
  flex-shrink: 0;
  display: flex;
  flex-direction: column;
  align-items: flex-end;
  gap: 7px;
  max-width: 420px;
}

.profile-primary-actions,
.profile-secondary-actions {
  display: flex;
  flex-wrap: wrap;
  justify-content: flex-end;
  gap: 6px;
}

.profile-apply-action {
  min-width: 112px !important;
  background: var(--green-bg, #6fa85f) !important;
  color: #fff !important;
  box-shadow: 0 5px 12px rgba(83, 134, 71, 0.18);
}

.profile-apply-action:hover:not(:disabled) {
  background: #5c994e !important;
}

.profile-mark-action {
  min-width: 86px !important;
  background: #eadbc2 !important;
  color: #6d5435 !important;
}

.profile-mark-action:hover:not(:disabled) {
  background: #decaab !important;
}

.profile-current-note {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  min-height: 28px;
  padding: 6px 10px;
  border-radius: 999px;
  background: rgba(111, 168, 95, 0.16);
  color: var(--green-text, #2f7d3e);
  font-size: 11px;
  font-weight: 900;
}

.profile-action-note {
  max-width: 390px;
  margin: 0 !important;
  color: #8b7155 !important;
  font-size: 11px !important;
  line-height: 1.45;
  text-align: right;
}

.profile-secondary-actions .tiny-button {
  min-width: 48px !important;
}

@media (max-width: 980px) {
  .profile-card-top {
    flex-direction: column;
  }

  .profile-action-groups,
  .profile-primary-actions,
  .profile-secondary-actions {
    align-items: flex-start;
    justify-content: flex-start;
    width: 100%;
    max-width: none;
  }

  .profile-action-note {
    max-width: none;
    text-align: left;
  }
}

/* v0.7.0：从 block2 合并——安装阶段文字 */
.side-install-stage,
.smapi-install-stage-text {
  margin-top: 10px;
  color: var(--text-secondary, #7a6652);
  font-size: 12px;
  line-height: 1.45;
  word-break: break-word;
}

/* v0.6.2：工具箱重排——从 block2 合并 */
.toolbox-workspace {
  gap: 16px !important;
}

.toolbox-section-block {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.toolbox-section-title-row {
  display: flex;
  align-items: flex-end;
  justify-content: space-between;
  gap: 16px;
  padding: 0 2px;
}

.toolbox-section-title-row h3 {
  margin: 0 0 4px;
  font-size: 22px;
  color: var(--text-primary, #2d241b);
}

.toolbox-section-title-row p {
  margin: 0;
  color: var(--text-secondary, #7a6652);
  font-size: 14px;
}

.toolbox-full-card,
.toolbox-result-panel {
  width: 100% !important;
  box-sizing: border-box !important;
}

.compact-tool-card {
  min-height: 0 !important;
}

.toolbox-install-actions {
  display: grid;
  grid-template-columns: minmax(0, 1fr) auto;
  gap: 14px;
  align-items: stretch;
}

.compact-drop-zone {
  min-height: 92px !important;
  margin: 0 !important;
}

.compact-zip-actions {
  align-self: stretch;
  min-width: 190px;
  margin: 0 !important;
  padding: 12px;
  border-radius: 18px;
  background: rgba(246, 234, 216, 0.72);
  border: 1px solid rgba(92, 70, 48, 0.1);
  display: flex !important;
  flex-direction: column;
  align-items: stretch !important;
  justify-content: center;
  gap: 9px;
}

.compact-zip-actions button {
  width: 100%;
}

.tool-result-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 16px;
  margin-bottom: 12px;
}

.compact-result-header {
  margin-bottom: 0 !important;
  min-width: 0;
}

.result-action-button {
  flex: 0 0 auto;
  width: auto !important;
  min-width: 132px;
}

.update-result-table,
.history-result-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
  max-height: 330px;
  overflow-y: auto;
  overflow-x: hidden;
  padding-right: 6px;
  margin-top: 10px;
}

.update-result-row,
.history-result-row {
  display: grid;
  grid-template-columns: minmax(0, 1fr) auto;
  align-items: center;
  gap: 12px;
  padding: 11px 12px;
  border-radius: 14px;
  background: rgba(255, 250, 240, 0.78);
  border: 1px solid rgba(92, 70, 48, 0.1);
}

.update-result-main {
  min-width: 0;
  display: flex;
  align-items: baseline;
  gap: 10px;
}

.update-result-main strong,
.history-result-row strong {
  min-width: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  color: var(--text-primary, #2d241b);
}

.update-result-main span,
.history-result-row p {
  margin: 0;
  color: var(--text-muted, #7a634a);
  font-size: 12px;
  line-height: 1.35;
}

.tool-inline-state {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 14px;
  padding: 12px 14px;
  border-radius: 18px;
  background: rgba(255, 250, 240, 0.7);
  border: 1px dashed rgba(92, 70, 48, 0.18);
  color: var(--text-secondary, #7a6652);
}

.tool-inline-state > div {
  min-width: 0;
  display: flex;
  align-items: baseline;
  gap: 10px;
  flex-wrap: wrap;
}

.tool-inline-state strong {
  color: var(--text-primary, #2d241b);
}

.tool-inline-state span {
  font-size: 13px;
}

.history-header-actions {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  justify-content: flex-end;
}

@media (max-width: 980px) {
  .toolbox-install-actions,
  .update-result-row,
  .history-result-row {
    grid-template-columns: 1fr;
  }

  .compact-zip-actions {
    min-width: 0;
  }

  .tool-result-header {
    align-items: flex-start;
    flex-direction: column;
  }

  .result-action-button,
  .tool-inline-state .tiny-button {
    width: 100% !important;
  }

  .update-result-main {
    align-items: flex-start;
    flex-direction: column;
    gap: 4px;
  }

  .tool-inline-state {
    align-items: flex-start;
    flex-direction: column;
  }
}
.smapi-install-stage-text {
  margin-top: 10px;
  color: var(--text-secondary, #7a6652);
  font-size: 12px;
  line-height: 1.45;
  word-break: break-word;
}

.secondary-action {
  background: var(--text-gold, #8b6f47);
}

.secondary-action:hover:not(:disabled) {
  background: var(--gold-hover, #755d3c);
}

.smapi-recheck-button {
  background: var(--text-gold, #8b6f47);
}

.smapi-recheck-button:hover:not(:disabled) {
  background: var(--gold-hover, #755d3c);
}


/* v0.3.0：Profiles 实用操作按钮 */
.profile-actions.compact-profile-actions {
  flex-wrap: wrap;
  justify-content: flex-end;
  max-width: 360px;
}

.profile-actions.compact-profile-actions .tiny-button {
  min-width: 52px;
}

/* v0.4.0：从链接安装 ZIP */
.url-zip-box {
  margin-top: 14px;
  padding: 14px;
  border-radius: 16px;
  background: var(--bg-card, #f6ead8);
  border: 1px solid var(--border-subtle, rgba(92, 70, 48, 0.12));
}

.url-zip-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 12px;
  margin-bottom: 10px;
}

.url-zip-header strong {
  font-size: 15px;
}

.url-zip-header span {
  color: var(--text-secondary, #7a6652);
  font-size: 12px;
  font-weight: 800;
}

.url-zip-form {
  display: grid;
  grid-template-columns: minmax(0, 1fr) auto;
  gap: 10px;
  align-items: center;
}

.url-zip-input {
  width: 100%;
  min-width: 0;
  box-sizing: border-box;
  border: 1px solid var(--border-strong, rgba(92, 70, 48, 0.22));
  border-radius: 13px;
  padding: 11px 13px;
  background: var(--bg-surface, #fffaf0);
  color: var(--text-primary, #2d241b);
  font-size: 14px;
}

.url-zip-input:disabled {
  opacity: 0.65;
}

.url-zip-button {
  white-space: nowrap;
}

.url-zip-status {
  margin-top: 10px !important;
}

@media (max-width: 760px) {
  .url-zip-form {
    grid-template-columns: 1fr;
  }
}

/* v0.3.0：修复 Profile 编辑小卡片双滚动 */
.profile-card-overlay {
  overflow: hidden !important;
}

.profile-editor-card,
.compact-profile-editor {
  max-height: calc(100vh - 48px) !important;
  overflow: hidden !important;
  display: flex !important;
  flex-direction: column !important;
  min-height: 0 !important;
}

.profile-card-header,
.profile-editor-grid,
.profile-editor-summary,
.profile-editor-footer {
  flex-shrink: 0 !important;
}

.profile-select-list,
.compact-profile-select-list {
  flex: 1 !important;
  min-height: 0 !important;
  max-height: none !important;
  overflow-y: auto !important;
  overflow-x: hidden !important;
  padding-right: 6px !important;
}

.profile-editor-footer {
  margin-top: 12px !important;
  padding-top: 12px !important;
  border-top: 1px solid var(--border-subtle, rgba(92, 70, 48, 0.12)) !important;
  background: var(--bg-surface, #fffaf0) !important;
}

@media (max-width: 820px) {
  .profile-editor-card,
  .compact-profile-editor {
    width: calc(100vw - 24px) !important;
    max-height: calc(100vh - 24px) !important;
  }

  .profile-editor-grid {
    grid-template-columns: 1fr !important;
  }
}


/* v0.4.0：安装结果改为轻量行内提示，避免顶开 Mod 列表 */
.inline-install-summary,
.inline-dependency-summary {
  margin-top: 10px;
  padding: 9px 12px;
  border-radius: 14px;
  font-size: 13px;
  font-weight: 700;
}

.inline-install-summary {
  background: rgba(229, 245, 219, 0.9);
  border: 1px solid rgba(111, 168, 95, 0.22);
  color: var(--green-text, #2f7d3e);
}

.inline-install-summary span {
  color: #4f7d48;
}

.inline-install-summary strong {
  color: #1f6d34;
}

.inline-dependency-summary {
  background: rgba(255, 243, 205, 0.9);
  border: 1px solid rgba(161, 119, 55, 0.22);
  color: #8a5a18;
}

.inline-dependency-summary span {
  color: #8a6a3d;
}

.inline-dependency-summary strong {
  color: #7a4b11;
}


/* v0.5.0：NXM 协议入口 */
.nxm-box {
  margin-top: 14px;
  padding: 14px;
  border-radius: 18px;
  background: rgba(255, 250, 240, 0.78);
  border: 1px solid var(--border-subtle, rgba(92, 70, 48, 0.12));
}

.nxm-action-row {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  margin-top: 10px;
}

.nxm-manual-form {
  margin-top: 10px;
}

.nxm-request-card {
  max-width: 680px;
}

.nxm-detail-grid {
  display: grid;
  grid-template-columns: repeat(3, minmax(0, 1fr));
  gap: 10px;
  margin: 14px 0;
}

.nxm-detail-grid > div {
  padding: 12px;
  border-radius: 16px;
  background: rgba(255, 250, 240, 0.9);
  border: 1px solid var(--border-subtle, rgba(92, 70, 48, 0.12));
}

.nxm-detail-grid span {
  display: block;
  margin-bottom: 4px;
  color: rgba(92, 70, 48, 0.62);
  font-size: 12px;
  font-weight: 800;
}

.nxm-detail-grid strong {
  color: #4a3222;
  font-size: 14px;
}

@media (max-width: 820px) {
  .nxm-detail-grid {
    grid-template-columns: 1fr;
  }
}


/* v0.5.2：Nexus API Key 设置 */
.nexus-key-form {
  display: grid;
  grid-template-columns: minmax(0, 1fr) auto;
  gap: 10px;
  align-items: center;
  margin-top: 12px;
}

.nexus-setting-actions {
  display: flex;
  flex-wrap: wrap;
  gap: 10px;
}

.nexus-status-block {
  margin-top: 14px;
}

.nexus-status-block small {
  display: block;
  margin-top: 6px;
  color: var(--text-secondary, #7a6652);
  font-size: 12px;
  font-weight: 800;
}



/* v0.6.0: 安装更新安全、历史、当前 Profile、更新来源 */
.zip-conflict-summary {
  margin: 12px 0;
  padding: 12px 14px;
  border-radius: 16px;
  border: 1px solid rgba(198, 126, 44, 0.25);
  background: rgba(255, 239, 205, 0.92);
  color: #7a4a16;
}

.zip-conflict-summary p {
  margin: 4px 0 0;
  font-size: 13px;
}

.zip-conflict-line {
  margin: 8px 0;
  padding: 7px 9px;
  border-radius: 12px;
  background: rgba(255, 239, 205, 0.85);
  color: #8a5317;
  font-size: 12px;
  font-weight: 700;
}

.current-profile-chip {
  background: rgba(111, 168, 95, 0.18);
  color: var(--green-text, #2f7d3e);
}

.two-column-tool-grid {
  align-items: stretch;
}

.history-list,
.update-check-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
  margin: 12px 0;
}

.history-item,
.update-check-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  padding: 10px 12px;
  border-radius: 14px;
  background: rgba(255, 250, 240, 0.78);
  border: 1px solid rgba(92, 70, 48, 0.1);
}

.history-item p,
.update-check-item p {
  margin: 4px 0 0;
  color: var(--text-muted, #7a634a);
  font-size: 12px;
}

.current-profile-side-card .side-check-button {
  margin-top: 10px;
}



/* v0.6.1：更新检测结果改为卡片内部滚动，避免撑开工具箱布局 */
.two-column-tool-grid {
  align-items: start !important;
}

.install-history-card,
.update-check-card {
  min-height: 0 !important;
  align-self: start !important;
}

.update-check-summary {
  margin-bottom: 8px !important;
}

.history-list.compact-scroll-list,
.update-check-list.compact-scroll-list,
.update-check-card .update-check-list,
.install-history-card .history-list {
  max-height: 260px !important;
  overflow-y: auto !important;
  overflow-x: hidden !important;
  padding-right: 6px !important;
}

.update-check-card .update-check-list::-webkit-scrollbar,
.install-history-card .history-list::-webkit-scrollbar {
  width: 8px;
}

.update-check-card .update-check-list::-webkit-scrollbar-thumb,
.install-history-card .history-list::-webkit-scrollbar-thumb {
  border-radius: 999px;
  background: var(--border-strong, rgba(92, 70, 48, 0.22));
}

.update-check-item,
.history-item {
  align-items: flex-start !important;
}

.update-check-item > div,
.history-item > div {
  min-width: 0;
}

.update-check-item strong,
.history-item strong {
  display: block;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.update-check-item .tiny-button {
  flex-shrink: 0;
}

.delete-mod-button {
  background: var(--danger-bg, #b9574f) !important;
  color: var(--bg-surface, #fffaf0) !important;
}

.delete-mod-button:hover {
  filter: brightness(0.96);
}

.compact-card-actions {
  flex-wrap: wrap;
  justify-content: flex-end;
}

.compact-card-actions .tiny-button {
  width: auto !important;
  min-width: 52px;
}



/* v0.6.1：Profiles 操作区重排，区分"实际切换"和"仅标记" */
.profile-action-groups {
  flex-shrink: 0;
  display: flex;
  flex-direction: column;
  align-items: flex-end;
  gap: 7px;
  max-width: 420px;
}

.profile-primary-actions,
.profile-secondary-actions {
  display: flex;
  flex-wrap: wrap;
  justify-content: flex-end;
  gap: 6px;
}

.profile-apply-action {
  min-width: 112px !important;
  background: var(--green-bg, #6fa85f) !important;
  color: #fff !important;
  box-shadow: 0 5px 12px rgba(83, 134, 71, 0.18);
}

.profile-apply-action:hover:not(:disabled) {
  background: #5c994e !important;
}

.profile-mark-action {
  min-width: 86px !important;
  background: #eadbc2 !important;
  color: #6d5435 !important;
}

.profile-mark-action:hover:not(:disabled) {
  background: #decaab !important;
}

.profile-current-note {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  min-height: 28px;
  padding: 6px 10px;
  border-radius: 999px;
  background: rgba(111, 168, 95, 0.16);
  color: var(--green-text, #2f7d3e);
  font-size: 11px;
  font-weight: 900;
}

.profile-action-note {
  max-width: 390px;
  margin: 0 !important;
  color: #8b7155 !important;
  font-size: 11px !important;
  line-height: 1.45;
  text-align: right;
}

.profile-secondary-actions .tiny-button {
  min-width: 48px !important;
}

@media (max-width: 980px) {
  .profile-card-top {
    flex-direction: column;
  }

  .profile-action-groups,
  .profile-primary-actions,
  .profile-secondary-actions {
    align-items: flex-start;
    justify-content: flex-start;
    width: 100%;
    max-width: none;
  }

  .profile-action-note {
    max-width: none;
    text-align: left;
  }
}



/* v0.6.2：工具箱重排，入口在上、结果在下，避免半宽结果列表挤压布局 */
.toolbox-workspace {
  gap: 16px !important;
}

.toolbox-section-block {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.toolbox-section-title-row {
  display: flex;
  align-items: flex-end;
  justify-content: space-between;
  gap: 16px;
  padding: 0 2px;
}

.toolbox-section-title-row h3 {
  margin: 0 0 4px;
  font-size: 22px;
  color: var(--text-primary, #2d241b);
}

.toolbox-section-title-row p {
  margin: 0;
  color: var(--text-secondary, #7a6652);
  font-size: 14px;
}

.toolbox-full-card,
.toolbox-result-panel {
  width: 100% !important;
  box-sizing: border-box !important;
}

.compact-tool-card {
  min-height: 0 !important;
}

.toolbox-install-actions {
  display: grid;
  grid-template-columns: minmax(0, 1fr) auto;
  gap: 14px;
  align-items: stretch;
}

.compact-drop-zone {
  min-height: 92px !important;
  margin: 0 !important;
}

.compact-zip-actions {
  align-self: stretch;
  min-width: 190px;
  margin: 0 !important;
  padding: 12px;
  border-radius: 18px;
  background: rgba(246, 234, 216, 0.72);
  border: 1px solid rgba(92, 70, 48, 0.1);
  display: flex !important;
  flex-direction: column;
  align-items: stretch !important;
  justify-content: center;
  gap: 9px;
}

.compact-zip-actions button {
  width: 100%;
}

.tool-result-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 16px;
  margin-bottom: 12px;
}

.compact-result-header {
  margin-bottom: 0 !important;
  min-width: 0;
}

.result-action-button {
  flex: 0 0 auto;
  width: auto !important;
  min-width: 132px;
}

.update-result-table,
.history-result-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
  max-height: 330px;
  overflow-y: auto;
  overflow-x: hidden;
  padding-right: 6px;
  margin-top: 10px;
}

.update-result-row,
.history-result-row {
  display: grid;
  grid-template-columns: minmax(0, 1fr) auto;
  align-items: center;
  gap: 12px;
  padding: 11px 12px;
  border-radius: 14px;
  background: rgba(255, 250, 240, 0.78);
  border: 1px solid rgba(92, 70, 48, 0.1);
}

.update-result-main {
  min-width: 0;
  display: flex;
  align-items: baseline;
  gap: 10px;
}

.update-result-main strong,
.history-result-row strong {
  min-width: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  color: var(--text-primary, #2d241b);
}

.update-result-main span,
.history-result-row p {
  margin: 0;
  color: var(--text-muted, #7a634a);
  font-size: 12px;
  line-height: 1.35;
}

.tool-inline-state {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 14px;
  padding: 12px 14px;
  border-radius: 18px;
  background: rgba(255, 250, 240, 0.7);
  border: 1px dashed rgba(92, 70, 48, 0.18);
  color: var(--text-secondary, #7a6652);
}

.tool-inline-state > div {
  min-width: 0;
  display: flex;
  align-items: baseline;
  gap: 10px;
  flex-wrap: wrap;
}

.tool-inline-state strong {
  color: var(--text-primary, #2d241b);
}

.tool-inline-state span {
  font-size: 13px;
}

.history-header-actions {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  justify-content: flex-end;
}

@media (max-width: 980px) {
  .toolbox-install-actions,
  .update-result-row,
  .history-result-row {
    grid-template-columns: 1fr;
  }

  .compact-zip-actions {
    min-width: 0;
  }

  .tool-result-header {
    align-items: flex-start;
    flex-direction: column;
  }

  .result-action-button,
  .tool-inline-state .tiny-button {
    width: 100% !important;
  }

  .update-result-main {
    align-items: flex-start;
    flex-direction: column;
    gap: 4px;
  }

  .tool-inline-state {
    align-items: flex-start;
    flex-direction: column;
  }
}

/* v0.7.0：自定义模态对话框 */
.modal-overlay {
  position: fixed;
  inset: 0;
  z-index: 9999;
  background: rgba(45, 36, 27, 0.5);
  display: grid;
  place-items: center;
}

.modal-box {
  min-width: 340px;
  max-width: 440px;
  padding: 24px;
  border-radius: var(--radius-panel, 12px);
  background: var(--bg-surface, #fffaf0);
  box-shadow: 0 20px 48px rgba(45, 36, 27, 0.25);
}

.modal-box h3 {
  margin: 0 0 12px;
  font-size: 20px;
  color: var(--text-primary, #2d241b);
}

.modal-box p {
  margin: 0 0 16px;
  color: var(--text-secondary, #7a6652);
  line-height: 1.5;
  white-space: pre-wrap;
}

.modal-input {
  width: 100%;
  box-sizing: border-box;
  border: 1px solid var(--border-strong, rgba(92, 70, 48, 0.22));
  border-radius: 13px;
  padding: 11px 13px;
  background: var(--bg-surface, #fffaf0);
  color: var(--text-primary, #2d241b);
  font-size: 14px;
  margin-bottom: 16px;
}

.modal-input:focus {
  outline: none;
  border-color: var(--green-bg, #6fa85f);
}

.modal-actions {
  display: flex;
  justify-content: flex-end;
  gap: 10px;
}

.modal-actions .tiny-button {
  min-width: 80px;
}

/* Search highlight */
mark.hl {
  background: #f0d89f;
  color: #5c3f1a;
  border-radius: 3px;
  padding: 0 2px;
  font-weight: 700;
}

/* Toast notification system */
.toast-container {
  position: fixed;
  bottom: 24px;
  right: 24px;
  z-index: 200;
  display: flex;
  flex-direction: column;
  gap: 8px;
  pointer-events: none;
}

.toast-item {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 12px 16px;
  border-radius: 10px;
  border: 2px solid rgba(92, 70, 48, 0.15);
  background: #fffaf0;
  box-shadow: 0 8px 24px rgba(61, 40, 21, 0.18);
  font-size: 14px;
  pointer-events: auto;
  animation: toast-in 0.25s ease;
}

@keyframes toast-in {
  from { opacity: 0; transform: translateY(12px); }
  to { opacity: 1; transform: translateY(0); }
}

.toast-success { border-color: rgba(111, 168, 95, 0.4); background: #f0f7ea; }
.toast-error { border-color: rgba(185, 87, 79, 0.4); background: #fdf0ee; }
.toast-warning { border-color: rgba(250, 230, 189, 0.8); background: #fef9ed; }

.toast-text { flex: 1; color: var(--text-primary, #2d241b); }

.toast-undo {
  padding: 4px 10px;
  border: 1px solid var(--green-bg, #6fa85f);
  border-radius: 8px;
  background: transparent;
  color: var(--green-bg, #6fa85f);
  font-size: 12px;
  font-weight: 800;
  cursor: pointer;
}

.toast-undo:hover {
  background: var(--green-bg, #6fa85f);
  color: #fff;
}

.toast-close {
  padding: 2px 6px;
  border: none;
  background: transparent;
  color: var(--text-secondary, #7a6652);
  font-size: 12px;
  cursor: pointer;
  opacity: 0.6;
}

.toast-close:hover { opacity: 1; }

</style>
<style scoped>
.app-shell {
  position: relative;
  background:
    linear-gradient(180deg, rgba(255, 255, 255, 0.56), rgba(255, 255, 255, 0) 24%),
    linear-gradient(135deg, rgba(162, 103, 44, 0.08), transparent 32%),
    var(--bg-page, #f5eddd);
}

.sidebar {
  padding: 18px 16px;
  background:
    linear-gradient(180deg, rgba(255, 255, 255, 0.06), transparent 18%),
    linear-gradient(180deg, var(--bg-sidebar-start, #7c4b22), var(--bg-sidebar-end, #3c2412));
  border-right: 2px solid rgba(36, 21, 10, 0.22);
}

.brand {
  background: rgba(255, 246, 229, 0.08);
  border: 1px solid rgba(255, 246, 229, 0.16);
  box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.45);
}

.nav-button {
  border: 1px solid transparent;
  transition: transform 0.16s ease, background 0.16s ease, color 0.16s ease;
}

.nav-button:hover,
.nav-button.active {
  transform: translateX(2px);
  border-color: rgba(255, 247, 232, 0.45);
  background: linear-gradient(180deg, #f9f0da, #ecdfc3);
  color: #3f2b1d;
}

.content {
  padding: 20px;
  background:
    radial-gradient(circle at top left, rgba(127, 179, 107, 0.14), transparent 28%),
    linear-gradient(180deg, rgba(255, 255, 255, 0.34), transparent 18%),
    linear-gradient(180deg, rgba(255, 250, 240, 0.9), rgba(247, 234, 210, 0.94));
}

.content-header {
  padding: 18px 20px;
  border-radius: var(--radius-panel, 12px);
  border: 1px solid rgba(92, 70, 48, 0.12);
  background: rgba(255, 249, 236, 0.8);
  box-shadow: 0 14px 34px rgba(61, 40, 21, 0.14);
}

.overview-hero {
  display: grid;
  grid-template-columns: minmax(0, 1.1fr) minmax(280px, 0.9fr);
  gap: 18px;
  align-items: stretch;
  padding: 22px;
  overflow: hidden;
  background: linear-gradient(180deg, #f5e7c7, #e7cda4);
  border: 1px solid rgba(92, 70, 48, 0.12);
}

.overview-hero-copy h3 {
  margin: 4px 0 8px;
  font-size: 30px;
  line-height: 1.08;
}

.overview-hero-copy p {
  margin: 0;
  max-width: 46ch;
  color: var(--text-secondary, #755f48);
  line-height: 1.55;
}

.hero-chip-row {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  margin-top: 16px;
}

.hero-chip-row span {
  padding: 6px 10px;
  border-radius: 999px;
  background: rgba(111, 168, 95, 0.16);
  color: var(--green-text, #2f7d3e);
  font-size: 12px;
  font-weight: 800;
}

.hero-action-row {
  display: flex;
  flex-wrap: wrap;
  gap: 10px;
  margin-top: 18px;
}

.hero-action-button {
  min-height: 44px;
  padding: 10px 16px;
  border: 1px solid rgba(92, 70, 48, 0.18);
  border-radius: var(--radius-button, 8px);
  background: var(--green-bg, #6fa85f);
  color: #fffaf0;
  font-weight: 800;
}

.hero-action-button.secondary {
  background: var(--text-gold, #8d693c);
}

.overview-hero-scene {
  position: relative;
  min-height: 250px;
  border-radius: 18px;
  overflow: hidden;
  border: 4px solid rgba(92, 70, 48, 0.2);
  background: linear-gradient(180deg, #8fd0f5 0%, #cbeafe 42%, #e2f3ad 42%, #b0d66d 100%);
}

.scene-mountains,
.scene-field,
.scene-house,
.scene-tree,
.scene-crate,
.scene-junimo {
  position: absolute;
}

.scene-mountains {
  left: -10%;
  right: -10%;
  top: 16%;
  height: 30%;
  background:
    linear-gradient(135deg, transparent 46%, rgba(72, 125, 80, 0.95) 46% 58%, transparent 58%) left bottom/34% 100% no-repeat,
    linear-gradient(135deg, transparent 44%, rgba(56, 106, 70, 0.95) 44% 59%, transparent 59%) center bottom/36% 100% no-repeat,
    linear-gradient(135deg, transparent 45%, rgba(88, 141, 90, 0.95) 45% 60%, transparent 60%) right bottom/34% 100% no-repeat;
}

.scene-field {
  inset: auto 0 0;
  height: 44%;
  background: linear-gradient(180deg, #84bc58, #5f9338 70%, #49712b);
}

.scene-house {
  left: 50%;
  bottom: 16%;
  width: 124px;
  height: 88px;
  transform: translateX(-50%);
}

.scene-roof {
  position: absolute;
  inset: 0 6px auto;
  height: 34px;
  background: linear-gradient(180deg, #5e9a4f, #47773a);
  clip-path: polygon(50% 0, 100% 60%, 100% 100%, 0 100%, 0 60%);
}

.scene-wall {
  position: absolute;
  left: 18px;
  right: 18px;
  bottom: 0;
  height: 58px;
  border-radius: 8px 8px 10px 10px;
  background: linear-gradient(180deg, #bb8b53, #8c5a2c);
  border: 2px solid rgba(52, 30, 16, 0.22);
}

.scene-door {
  position: absolute;
  left: 52px;
  bottom: 4px;
  width: 20px;
  height: 32px;
  border-radius: 6px 6px 3px 3px;
  background: linear-gradient(180deg, #6c4420, #4b2e17);
}

.scene-window {
  position: absolute;
  bottom: 20px;
  width: 14px;
  height: 14px;
  border-radius: 3px;
  background: #f8e8b2;
  border: 2px solid rgba(70, 43, 22, 0.22);
}

.scene-window.left { left: 33px; }
.scene-window.right { right: 33px; }

.scene-tree {
  bottom: 15%;
  width: 32px;
  height: 76px;
  border-radius: 16px 16px 10px 10px;
  background:
    radial-gradient(circle at 50% 18%, #7dc264 0 36%, transparent 38%),
    radial-gradient(circle at 35% 40%, #5f9b49 0 28%, transparent 30%),
    radial-gradient(circle at 65% 42%, #4b8338 0 26%, transparent 28%),
    linear-gradient(180deg, #6e4d2b, #4a2f19 72%, transparent 72%);
}

.scene-tree-left { left: 12px; }
.scene-tree-right { right: 12px; }

.scene-crate {
  left: 18%;
  bottom: 19%;
  width: 28px;
  height: 22px;
  border-radius: 4px;
  background: linear-gradient(180deg, #b57b3e, #885429);
  border: 2px solid rgba(50, 29, 15, 0.2);
}

.scene-junimo {
  right: 12px;
  bottom: 10px;
  width: 38px;
  height: 38px;
  border-radius: 50%;
  background:
    radial-gradient(circle at 50% 38%, #f6f2c0 0 2px, transparent 3px),
    radial-gradient(circle at 38% 44%, #5d903f 0 3px, transparent 4px),
    radial-gradient(circle at 62% 44%, #5d903f 0 3px, transparent 4px),
    linear-gradient(180deg, #a9df74, #6ea84f);
}

.panel,
.notice,
.empty-state,
.launch-card,
.side-card,
.modal-box,
.zip-preview-card,
.mod-detail-card,
.profile-editor-card,
.profile-action-card,
.tool-section-card,
.profile-card,
.mod-item,
.zip-preview-item,
.url-zip-box,
.nxm-box {
  border: var(--border-pixel, 2px solid rgba(92, 70, 48, 0.15));
  background: rgba(255, 249, 236, 0.9);
  box-shadow: 0 10px 24px rgba(61, 40, 21, 0.10);
}

.panel,
.empty-state {
  border-radius: var(--radius-panel, 12px);
}

.launch-card {
  border-radius: var(--radius-card, 10px);
  background: linear-gradient(180deg, #f6e4be, #e8c78d);
}

.right-panel {
  padding: 18px 16px;
  background: rgba(255, 249, 236, 0.66);
  border-left: 1px solid rgba(92, 70, 48, 0.1);
}

button,
.launch-button,
.hero-action-button {
  box-shadow: 0 8px 18px rgba(95, 67, 32, 0.15);
}

button.secondary,
.secondary-action,
.smapi-recheck-button,
.hero-action-button.secondary {
  background: var(--text-gold, #8d693c);
}

button.secondary:hover:not(:disabled),
.secondary-action:hover:not(:disabled),
.smapi-recheck-button:hover:not(:disabled),
.hero-action-button.secondary:hover:not(:disabled) {
  background: var(--gold-hover, #735331);
}

.url-zip-input,
.profile-input,
.modal-input {
  border: 1px solid rgba(92, 70, 48, 0.16);
  background: rgba(255, 252, 245, 0.96);
}

.mod-item,
.profile-card,
.zip-preview-item,
.update-result-row,
.history-result-row,
.tool-inline-state,
.setting-block,
.diagnosis-card {
  border-radius: 16px;
}

.folder-chip,
.profile-preview-inline span,
.expanded-profile-preview span,
.hero-chip-row span {
  background: rgba(233, 217, 188, 0.9);
}

.mod-item h4,
.profile-main h4,
.zip-preview-title-row h4 {
  font-size: 17px;
}

@media (max-width: 1320px) {
  .overview-hero {
    grid-template-columns: 1fr;
  }
}

@media (max-width: 1200px) {
  .right-panel {
    display: none;
  }
}

@media (max-width: 820px) {
  .overview-hero-copy h3 {
    font-size: 24px;
  }

  .hero-action-row,
  .hero-chip-row {
    gap: 8px;
  }
}

/* 下载队列浮动按钮 */
.download-queue-fab {
  position: fixed;
  bottom: 24px;
  right: 24px;
  z-index: 1000;
  width: 52px;
  height: 52px;
  border-radius: 50%;
  border: 2px solid var(--border-strong);
  background: linear-gradient(180deg, #f9f0da, #ecdfc3);
  color: var(--text-primary);
  font-size: 20px;
  cursor: pointer;
  box-shadow: 0 4px 16px rgba(61, 40, 21, 0.25);
  transition: transform 0.15s, box-shadow 0.15s;
  display: flex;
  align-items: center;
  justify-content: center;
}

.download-queue-fab:hover {
  transform: scale(1.08);
  box-shadow: 0 6px 24px rgba(61, 40, 21, 0.35);
}

.download-queue-fab.has-active {
  border-color: var(--green-bg);
  animation: dq-pulse 2s ease-in-out infinite;
}

@keyframes dq-pulse {
  0%, 100% { box-shadow: 0 4px 16px rgba(61, 40, 21, 0.25); }
  50% { box-shadow: 0 4px 24px rgba(111, 168, 95, 0.5); }
}

.dq-fab-count {
  position: absolute;
  top: -6px;
  right: -6px;
  min-width: 20px;
  height: 20px;
  padding: 0 5px;
  border-radius: 10px;
  background: var(--danger-bg);
  color: #fff;
  font-size: 11px;
  font-weight: 700;
  line-height: 20px;
  text-align: center;
}

/* 下载队列面板 */
.download-queue-panel {
  position: fixed;
  bottom: 84px;
  right: 24px;
  z-index: 999;
  width: 380px;
  max-height: 460px;
  border-radius: 16px;
  border: 1px solid var(--border-strong);
  background: var(--bg-surface);
  box-shadow: 0 12px 40px rgba(61, 40, 21, 0.3);
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.dq-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 16px;
  border-bottom: 1px solid var(--border-subtle);
  background: var(--bg-card);
}

.dq-header h3 {
  margin: 0;
  font-size: 15px;
  font-weight: 700;
  color: var(--text-primary);
}

.dq-header-actions {
  display: flex;
  gap: 6px;
}

.dq-list {
  flex: 1;
  overflow-y: auto;
  padding: 8px;
}

.dq-item {
  padding: 10px 12px;
  margin-bottom: 6px;
  border-radius: 10px;
  border: 1px solid var(--border-subtle);
  background: var(--bg-page);
  font-size: 13px;
}

.dq-item:last-child {
  margin-bottom: 0;
}

.dq-item.dq-completed {
  border-color: var(--green-bg);
}

.dq-item.dq-failed {
  border-color: var(--danger-bg);
}

.dq-item.dq-cancelled {
  border-color: var(--border-subtle);
  opacity: 0.6;
}

.dq-item.dq-downloading,
.dq-item.dq-merging {
  border-color: #d4b87a;
  background: #fef9ed;
}

.dq-item-top {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 4px;
}

.dq-file-name {
  flex: 1;
  font-weight: 600;
  color: var(--text-primary);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.dq-source-tag {
  font-size: 10px;
  padding: 1px 6px;
  border-radius: 4px;
  background: var(--border-subtle);
  color: var(--text-secondary);
}

.dq-status-badge {
  font-size: 11px;
  padding: 1px 8px;
  border-radius: 8px;
  font-weight: 600;
  white-space: nowrap;
}

.dq-badge-queued { background: #e8e0d0; color: #7a6652; }
.dq-badge-connecting { background: #f8e7c8; color: #8a7540; }
.dq-badge-downloading { background: #f8e7c8; color: #8a7540; }
.dq-badge-merging { background: #d4e8c8; color: #4a7a3a; }
.dq-badge-completed { background: #d4e8c8; color: #3a6a2a; }
.dq-badge-failed { background: #f4d0cc; color: #a04030; }
.dq-badge-cancelled { background: #e0d8d0; color: #8a7a6a; }

.dq-progress-bar {
  height: 4px;
  border-radius: 2px;
  background: var(--border-subtle);
  margin: 6px 0;
  overflow: hidden;
}

.dq-progress-fill {
  height: 100%;
  border-radius: 2px;
  background: linear-gradient(90deg, #b8d4a0, var(--green-bg));
  transition: width 0.3s ease;
}

.dq-item-bottom {
  display: flex;
  align-items: center;
  gap: 8px;
  flex-wrap: wrap;
  font-size: 12px;
  color: var(--text-secondary);
}

.dq-message {
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  min-width: 0;
}

.dq-speed {
  font-variant-numeric: tabular-nums;
}

.dq-bytes {
  font-variant-numeric: tabular-nums;
}

.dq-item-actions {
  display: flex;
  gap: 6px;
  margin-top: 6px;
  justify-content: flex-end;
}

.dq-empty {
  text-align: center;
  padding: 40px 16px;
  color: var(--text-secondary);
  font-size: 14px;
}

/* 版本对比标签 */
.version-diff-tag {
  display: inline-block;
  font-size: 11px;
  padding: 0 6px;
  border-radius: 4px;
  font-weight: 600;
  margin-left: 4px;
}

.version-diff-tag.diff-newer {
  background: #d4e8c8;
  color: #3a6a2a;
}

.version-diff-tag.diff-downgrade {
  background: #f4d0cc;
  color: #a04030;
}

.version-diff-tag.diff-same {
  background: #e0d8d0;
  color: #8a7a6a;
}

.version-diff-tag.diff-unknown {
  background: var(--border-subtle);
  color: var(--text-secondary);
}

/* 回收站 */
.recycle-bin-list {
  max-height: 240px;
  overflow-y: auto;
  margin-top: 8px;
  border-top: 1px solid var(--border-subtle);
  padding-top: 6px;
}

.recycle-bin-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 6px 4px;
  border-bottom: 1px solid var(--border-subtle);
  gap: 8px;
}

.recycle-bin-row:last-child {
  border-bottom: none;
}

.rb-info {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.rb-info strong {
  font-size: 13px;
  color: var(--text-primary);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.rb-time {
  font-size: 11px;
  color: var(--text-secondary);
}

.rb-actions {
  display: flex;
  gap: 4px;
  flex-shrink: 0;
}

.rb-empty {
  text-align: center;
  padding: 16px 8px;
  color: var(--text-secondary);
  font-size: 13px;
}

.tool-action-button.danger {
  color: var(--danger-bg);
  border-color: var(--danger-bg);
}

.danger-button {
  padding: 6px 16px;
  border-radius: 8px;
  border: 1px solid var(--danger-bg, #b9574f);
  background: transparent;
  color: var(--danger-bg, #b9574f);
  font-size: 13px;
  cursor: pointer;
  transition: background 0.15s;
}

.danger-button:hover {
  background: var(--danger-bg, #b9574f);
  color: #fff;
}

/* Onboarding wizard */
.wizard-overlay {
  position: fixed;
  inset: 0;
  z-index: 9998;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(42, 28, 16, 0.7);
  backdrop-filter: blur(4px);
}

.wizard-card {
  width: 100%;
  max-width: 480px;
  padding: 40px 36px 32px;
  border-radius: var(--radius-panel, 12px);
  background: var(--bg-surface, #fff8e9);
  border: 2px solid rgba(92, 70, 48, 0.15);
  box-shadow: 0 20px 54px rgba(42, 28, 16, 0.35);
  text-align: center;
}

.wizard-steps {
  display: flex;
  justify-content: center;
  gap: 8px;
  margin-bottom: 28px;
}

.wizard-dot {
  width: 10px;
  height: 10px;
  border-radius: 50%;
  background: rgba(92, 70, 48, 0.15);
  transition: background 0.2s;
}

.wizard-dot.active {
  background: var(--green-bg, #6fa85f);
}

.wizard-body {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 12px;
}

.wizard-icon {
  font-size: 48px;
  line-height: 1;
  margin-bottom: 4px;
}

.wizard-body h3 {
  margin: 0;
  font-size: 22px;
  color: var(--text-primary, #2d241b);
}

.wizard-body > p {
  margin: 0;
  max-width: 36ch;
  color: var(--text-secondary, #755f48);
  line-height: 1.5;
}

.wizard-status {
  padding: 10px 18px;
  border-radius: var(--radius-button, 8px);
  background: rgba(92, 70, 48, 0.06);
  font-size: 14px;
  font-weight: 600;
}

.wizard-status-ok {
  color: var(--green-text, #2f7d3e);
}

.wizard-status-missing {
  color: var(--danger-text, #8f2f22);
}

.wizard-actions {
  display: flex;
  flex-wrap: wrap;
  gap: 10px;
  justify-content: center;
  margin-top: 8px;
}

.wizard-primary {
  padding: 10px 28px !important;
  font-size: 15px !important;
}
</style>
