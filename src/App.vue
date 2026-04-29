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

      <div
        v-if="notice"
        class="notice"
        :class="`notice-${notice.type}`"
      >
        <span class="notice-icon">{{ noticeIcon }}</span>
        <span>{{ notice.text }}</span>
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
                @click="selectMod(mod)"
              >
                <div class="mod-card-content">
                  <div class="mod-card-main-row">
                    <div class="mod-title-block">
                      <h4>{{ mod.name }}</h4>
                      <p class="mod-meta">
                        {{ mod.author || "未知作者" }} · v{{ mod.version || "未知版本" }}
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
                    <span class="folder-chip">{{ mod.folderName }}</span>

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
        <div class="toolbox-grid">
          <article class="panel tool-section-card">
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

          <article class="panel tool-section-card">
            <div class="tool-section-header">
              <div class="tool-section-icon">📤</div>
              <div>
                <h3>导出</h3>
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
        </div>

        <article class="panel tool-section-card smapi-tool-card">
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
            SMAPI {{ smapiInstallerVersion || "" }} 安装器已打开。请按官方安装器提示完成安装，完成后点击“我已完成安装，重新检测”。
          </p>

          <p class="tool-section-note">
            Junimo Box 会下载 SMAPI 最新安装包，解压后运行官方 install on Windows.bat。
            安装过程仍以 SMAPI 官方安装器为准。
          </p>
        </article>

        <article class="panel tool-section-card zip-tool-card">
          <div class="tool-section-header">
            <div class="tool-section-icon">📦</div>
            <div>
              <h3>安装 ZIP Mod</h3>
              <p>拖拽或选择 ZIP 压缩包，先预览和检查依赖，再安装到 Mods 文件夹。</p>
            </div>
          </div>

          <div
            class="zip-drop-zone"
            :class="{ active: isZipDragOver }"
            @click="handlePreviewZipMod"
          >
            <div class="zip-drop-icon">＋</div>
            <div>
              <strong>拖拽 ZIP Mod 到这里</strong>
              <p>拖入 .zip 后会自动生成安装预览，不会直接安装。</p>
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
                :disabled="isUrlZipDownloading"
                @keydown.enter="handleDownloadZipFromUrl"
              />

              <button
                class="url-zip-button"
                :disabled="!gamePath || isUrlZipDownloading || !urlZipInput.trim()"
                @click="handleDownloadZipFromUrl"
              >
                {{ isUrlZipDownloading ? "下载中..." : "下载并预览" }}
              </button>
            </div>

            <p v-if="urlZipDownloadMessage" class="tool-section-note url-zip-status">
              {{ urlZipDownloadMessage }}
            </p>
          </div>

          <div class="zip-tool-actions">
            <button @click="handlePreviewZipMod">
              选择 ZIP 文件
            </button>

            <span class="zip-tool-hint">
              支持安装前依赖检查、链接下载和临时目录安全解压。
            </span>
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
                :disabled="!gamePath"
                @click="handleInstallZipMod"
              >
                安装到 Mods
              </button>
            </div>
          </section>
        </div>
      </section>


      <section v-if="activeView === 'profiles'" class="view-stack profiles-page">
        <div class="panel compact-panel profile-hero-panel">
          <div class="profile-hero-main">
            <div>
              <p class="eyebrow">Profiles</p>
              <h3>配置方案（实验）</h3>
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
                  <span class="experiment-chip">实验</span>
                </div>

                <p>
                  {{ profile.enabledFolderNames.length }} 个启用 Mod · 更新于 {{ formatDateTime(profile.updatedAt) }}
                </p>
              </div>

              <div class="profile-actions compact-profile-actions">
                <button
                  class="tiny-button"
                  :disabled="!gamePath"
                  @click="handleApplyProfile(profile)"
                >
                  应用
                </button>

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
                  重命名
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
          <p>点击“新建配置”，在小卡片里直接勾选要启用的 Mod。</p>
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

        <div v-if="smapiExists" class="info-line">
          <span>SMAPI 版本</span>
          <strong>{{ smapiDetectedVersion || "未识别" }}</strong>
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

        <div class="info-line">
          <span>启动前检查</span>
          <strong :class="launchHealthStatus.className">
            {{ launchHealthStatus.label }}
          </strong>
        </div>

        <button
          class="side-check-button"
          :disabled="!gamePath"
          @click="handleRunLaunchCheck"
        >
          检查环境
        </button>
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
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { getCurrentWebview } from "@tauri-apps/api/webview";
import { open, save } from "@tauri-apps/plugin-dialog";
import { exists, readDir, readTextFile } from "@tauri-apps/plugin-fs";
import JSON5 from "json5";

const STORAGE_KEY = "junimo-box-game-path";
const PROFILES_STORAGE_KEY = "junimo-box-profiles";

type ViewId = "overview" | "mods" | "logs" | "tools" | "profiles" | "settings";
type ModStatusFilter = "all" | "enabled" | "disabled";
type ModDependencyFilter = "all" | "missing";
type LaunchTarget = "smapi" | "vanilla";
type LaunchIssueLevel = "error" | "warning";

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

const notice = ref<NoticePayload | null>(null);

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
const isZipDragOver = ref(false);
const urlZipInput = ref("");
const isUrlZipDownloading = ref(false);
const urlZipDownloadMessage = ref("");

let unlistenDragDrop: (() => void) | null = null;
let unlistenSmapiInstallStage: UnlistenFn | null = null;

const modSearchQuery = ref("");
const modStatusFilter = ref<ModStatusFilter>("all");
const modDependencyFilter = ref<ModDependencyFilter>("all");
const selectedModKey = ref("");
const profiles = ref<ModProfile[]>([]);
const isProfileEditorOpen = ref(false);
const profileEditorMode = ref<"create" | "edit">("create");
const editingProfileId = ref("");
const profileDraftName = ref("");
const profileDraftSearchQuery = ref("");
const profileDraftEnabledFolders = ref<string[]>([]);
const expandedProfileId = ref("");

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

const selectedMod = computed<DisplayModInfo | null>(() => {
  if (!selectedModKey.value) {
    return null;
  }

  return allDisplayMods.value.find((mod) => getModKey(mod) === selectedModKey.value) || null;
});

const duplicateEnabledUniqueIds = computed(() => getDuplicateUniqueIds(mods.value));

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

  loadProfiles();
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

  if (unlistenSmapiInstallStage) {
    unlistenSmapiInstallStage();
    unlistenSmapiInstallStage = null;
  }
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
  } catch (error) {
    profiles.value = [];
    message.value = `读取配置方案失败：${String(error)}`;
  }
}

function saveProfiles() {
  localStorage.setItem(PROFILES_STORAGE_KEY, JSON.stringify(profiles.value));
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
    .map((folderName) => folderName.trim());

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

    setNotice("success", `已导入 ${normalizedProfiles.length} 个配置方案。`);
  } catch (error) {
    setNotice("error", `导入配置方案失败：${String(error)}`);
  }
}

function handleRenameProfile(profile: ModProfile) {
  const newName = window.prompt("输入新的配置方案名称：", profile.name)?.trim();

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
      `SMAPI ${result.version} 安装器已打开。请在安装器中完成安装，然后点击“我已完成安装，重新检测”。`
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

async function handleRunLaunchCheck() {
  await runLaunchEnvironmentCheck("smapi", true);
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
    selectedModKey.value = "";
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
    selectedModKey.value = "";
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

  isUrlZipDownloading.value = true;
  urlZipDownloadMessage.value = "正在下载 ZIP Mod，下载速度取决于链接来源和网络环境...";
  activeView.value = "tools";

  try {
    const result = await invoke<UrlZipDownloadResult>("download_zip_from_url", {
      url,
      gamePath: gamePath.value,
    });

    urlZipDownloadMessage.value = `下载完成：${result.file_name}，正在生成安装预览...`;
    urlZipInput.value = "";

    await previewZipPath(result.zip_path);
  } catch (error) {
    urlZipDownloadMessage.value = "";
    message.value = `下载 ZIP Mod 失败：${String(error)}`;
  } finally {
    isUrlZipDownloading.value = false;
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

function closeZipPreview() {
  selectedZipPath.value = "";
  zipModPreviews.value = [];
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

function getModKey(mod: DisplayModInfo): string {
  return `${mod.isDisabled ? "disabled" : "enabled"}-${mod.uniqueId || mod.folderName}`;
}

function selectMod(mod: DisplayModInfo) {
  selectedModKey.value = getModKey(mod);
}

function closeModDetail() {
  selectedModKey.value = "";
}

function isSelectedMod(mod: DisplayModInfo): boolean {
  return selectedModKey.value === getModKey(mod);
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
  min-height: 0;
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
  display: flex;
  align-items: center;
  gap: 9px;
  border: 1px solid rgba(122, 79, 34, 0.1);
}

.notice-icon {
  flex-shrink: 0;
}

.notice-success {
  background: #e8f3df;
  color: #2f6f3c;
}

.notice-info {
  background: rgba(255, 250, 240, 0.92);
  color: #7a4f22;
}

.notice-warning {
  background: #f8e7c8;
  color: #7a4f22;
}

.notice-error {
  background: #f7dfd8;
  color: #8f2f22;
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

.status-card small {
  display: block;
  margin-top: 5px;
  color: #7a6652;
  font-size: 12px;
  font-weight: 700;
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
  background: #f6ead8;
  font-size: 22px;
}

.tool-section-header h3 {
  margin: 0 0 5px;
  font-size: 21px;
}

.tool-section-header p {
  margin: 0;
  color: #7a6652;
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
  background: #f6ead8;
}

.tool-status-row span {
  color: #7a6652;
}

.tool-section-note {
  margin: 12px 0 0;
  color: #7a6652;
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
  color: #7a6652;
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
  background: #fffaf0;
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
  color: #7a6652;
  line-height: 1.45;
}

.zip-card-path {
  padding: 10px 12px;
  border-radius: 12px;
  background: #f6ead8;
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
  background: #f6ead8;
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
  color: #7a6652;
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


.mod-detail-panel {
  border: 1px solid rgba(111, 168, 95, 0.28);
}

.detail-subtitle {
  margin: 4px 0 0;
  color: #7a6652;
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
  background: #f6ead8;
  min-width: 0;
}

.detail-grid span {
  display: block;
  margin-bottom: 5px;
  color: #7a6652;
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
  background: #f6ead8;
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
  background: #f6ead8;
}

.dependency-detail-item span {
  color: #7a6652;
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

.smapi-install-button {
  margin-top: 8px;
  background: #9f7d4a;
}

.smapi-install-button:hover:not(:disabled) {
  background: #87693d;
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
  color: #6fa85f;
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
  color: #5c4630;
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
  background: rgba(92, 70, 48, 0.22);
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
  background: #fffaf0;
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
  color: #7a6652;
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
  color: #7a6652;
}

.empty-detail-icon {
  width: 58px;
  height: 58px;
  display: grid;
  place-items: center;
  margin-bottom: 14px;
  border-radius: 20px;
  background: #f6ead8;
  font-size: 30px;
}

.empty-detail-state h3 {
  margin: 0 0 8px;
  color: #2d241b;
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
  background: #fffaf0;
  color: #2d241b;
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
  background: #f6ead8;
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
  color: #7a6652;
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
  color: #5c4630;
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
  background: #fffaf0;
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
  color: #7a6652;
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
  color: #7a6652;
  font-size: 13px;
  font-weight: 800;
}

.profile-editor-summary {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 12px;
  padding: 12px 0;
  color: #7a6652;
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
  background: #f6ead8;
  border: 1px solid transparent;
  cursor: pointer;
}

.profile-select-item.selected {
  border-color: rgba(111, 168, 95, 0.8);
  background: #e8f3df;
}

.profile-select-item input {
  margin-top: 4px;
  accent-color: #6fa85f;
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
  color: #7a6652;
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
  color: #7a4f22;
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
  color: #5c4630;
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
  color: #6b5238;
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
  background: #8b6f47;
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
  background: #8b6f47;
  font-size: 13px;
}

.side-check-button:hover:not(:disabled) {
  background: #755d3c;
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
  background: #e8f3df;
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
  background: #f6ead8;
  color: #2d241b;
  text-align: left;
  box-shadow: none;
}

.profile-action-card.primary {
  background: #6fa85f;
  color: #fffaf0;
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
  background: #fffaf0;
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
  color: #7a6652;
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
  border-top: 1px solid rgba(92, 70, 48, 0.12);
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
  color: #5c4630;
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
  background: #8b6f47;
}

.secondary-action:hover:not(:disabled) {
  background: #755d3c;
}

.smapi-recheck-button {
  background: #8b6f47;
}

.smapi-recheck-button:hover:not(:disabled) {
  background: #755d3c;
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
  background: #f6ead8;
  border: 1px solid rgba(92, 70, 48, 0.12);
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
  color: #7a6652;
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
  border: 1px solid rgba(92, 70, 48, 0.22);
  border-radius: 13px;
  padding: 11px 13px;
  background: #fffaf0;
  color: #2d241b;
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
  border-top: 1px solid rgba(92, 70, 48, 0.12) !important;
  background: #fffaf0 !important;
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
  color: #2f7d3e;
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

</style>


<style scoped>
.side-install-stage,
.smapi-install-stage-text {
  margin-top: 10px;
  color: #7a6652;
  font-size: 12px;
  line-height: 1.45;
  word-break: break-word;
}

.secondary-action {
  background: #8b6f47;
}

.secondary-action:hover:not(:disabled) {
  background: #755d3c;
}

.smapi-recheck-button {
  background: #8b6f47;
}

.smapi-recheck-button:hover:not(:disabled) {
  background: #755d3c;
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
  background: #f6ead8;
  border: 1px solid rgba(92, 70, 48, 0.12);
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
  color: #7a6652;
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
  border: 1px solid rgba(92, 70, 48, 0.22);
  border-radius: 13px;
  padding: 11px 13px;
  background: #fffaf0;
  color: #2d241b;
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
  border-top: 1px solid rgba(92, 70, 48, 0.12) !important;
  background: #fffaf0 !important;
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
  color: #2f7d3e;
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

</style>
