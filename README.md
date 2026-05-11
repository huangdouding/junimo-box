# Junimo Box / 祝尼魔盒

> 本项目由 AI（Claude）辅助生成，是一个学习与实验性质的项目。
> This project is AI-assisted (Claude) and is built for learning and experimentation.

Junimo Box 是一个面向《星露谷物语》的第三方 Mod 管理器兼游戏启动器，帮助玩家更轻松地管理 Mod、启动游戏、切换配置和排查问题。

Junimo Box is a third-party mod manager and game launcher for **Stardew Valley**, designed to help players manage mods, launch the game, switch profiles, and troubleshoot — all in one cozy desktop app.

---

## 功能 / Features

| 中文 | English |
|---|---|
| **游戏目录管理** — 自动检测或手动选择 Stardew Valley 安装路径 | **Game Path Management** — Auto-detect or manually select your Stardew Valley installation |
| **SMAPI 管理** — 检测、安装、更新 SMAPI，启动 Mod 版与原版 | **SMAPI Management** — Detect, install, update SMAPI; launch modded or vanilla |
| **Mod 管理** — 扫描、启用/禁用、搜索筛选、依赖检测、类型标签 | **Mod Management** — Scan, enable/disable, search & filter, dependency detection, type labels |
| **批量操作** — Ctrl/Shift 多选，批量启用/禁用/删除 | **Batch Operations** — Multi-select with Ctrl/Shift, batch enable/disable/delete |
| **ZIP 安装** — 拖拽或选择 ZIP 文件预览安装，含冲突检测与依赖检查 | **ZIP Installation** — Drag & drop or pick ZIP files, preview before install, conflict & dependency checking |
| **配置方案 (Profiles)** — 创建、复制、导出/导入不同的 Mod 组合 | **Profiles** — Create, copy, export/import different mod combinations |
| **回收站** — 删除的 Mod 进入回收站，支持还原和清空 | **Recycle Bin** — Deleted mods go to a recycle bin, with restore and empty support |
| **下载队列** — 支持 NXM 协议和 URL ZIP 下载，含进度显示和取消 | **Download Queue** — NXM protocol & URL ZIP downloads with progress and cancellation |
| **安装历史** — 记录每次安装的来源和冲突处理方式 | **Install History** — Records every install source and conflict resolution |
| **SMAPI 日志** — 日志查看与基础诊断 | **SMAPI Logs** — Log viewer with basic diagnosis |
| **Toast 通知** — 操作结果轻量提示，支持撤销 | **Toast Notifications** — Lightweight operation feedback with undo support |
| **键盘快捷键** — Ctrl+F 搜索、Ctrl+R 扫描、Escape 关闭面板 | **Keyboard Shortcuts** — Ctrl+F search, Ctrl+R scan, Escape close panels |
| **新手引导** — 首次使用 4 步引导配置 | **Onboarding Wizard** — 4-step setup guide for first-time users |
| **备份与还原** — 导出/导入 Mod 启用状态 | **Backup & Restore** — Export/import mod enable states |

---

## 技术栈 / Tech Stack

| 层 / Layer | 技术 / Technology |
|---|---|
| 前端 / Frontend | Vue 3 + TypeScript + Vite |
| 后端 / Backend | Rust / Tauri 2 |
| 包管理 / Package Manager | pnpm |
| 国际化 / i18n | 内置翻译引擎，支持中英文切换 / Built-in translation engine with zh/en toggle |

---

## 开发 / Development

```bash
# 安装依赖 / Install dependencies
pnpm install

# 启动开发环境 / Start dev environment
pnpm tauri dev

# 构建 / Build
pnpm tauri build

# 仅前端构建 / Frontend-only build
pnpm build
```

---

## 下载 / Download

从 [Releases](https://github.com/huangdouding/junimo-box/releases) 页面下载最新安装包。

Download the latest installer from the [Releases](https://github.com/huangdouding/junimo-box/releases) page.

---

## 许可 / License

MIT
