# Junimo Box / 祝尼魔盒

Junimo Box 是一个面向《星露谷物语》的第三方 Mod 管理器兼游戏启动器。

它的目标是让玩家可以更轻松地管理 Stardew Valley 的 Mod、启动游戏、切换配置、查看 SMAPI 日志，并尽可能减少手动折腾文件夹、依赖、版本和冲突的麻烦。

> 当前项目仍处于早期开发阶段。

---

## 项目目标

Junimo Box 最终希望成为一个温暖、可爱、实用的星露谷 Mod 启动中心。

长期目标包括：

- 识别 Stardew Valley 游戏目录
- 识别并管理 SMAPI
- 扫描本地 Mods 文件夹
- 启用 / 禁用 Mod
- 查看 Mod 信息、版本、作者和说明
- 查看 SMAPI 日志并辅助排查问题
- 支持本地 ZIP Mod 安装
- 支持多个 Mod 配置方案
- 支持存档、Mods 和配置备份
- 支持启动器内浏览、下载、安装和更新 Mod
- 打造温暖像素风 UI

---

## 当前版本目标

当前阶段优先完成 MVP：一个可以稳定使用的本地 Mod 管理器。

第一版的目标是：

> 打开 Junimo Box → 选择游戏目录 → 扫描 Mods 文件夹 → 查看 Mod 列表 → 启用或禁用 Mod → 启动游戏。

---

## 当前已实现 / 计划中的功能

### 已实现

- Stardew Valley 游戏目录选择
- Stardew Valley、SMAPI 和 Mods 文件夹检测
- 启动 SMAPI
- 启动原版 Stardew Valley
- 本地 Mod 扫描
- 递归扫描分类文件夹
- Mod 启用 / 禁用
- Mod 搜索和筛选
- Mod 类型标签
- Mod 详情面板
- 依赖检测
- SMAPI 日志查看和基础诊断
- Mod 列表导出
- 问题报告导出
- ZIP Mod 预览
- ZIP Mod 拖拽预览
- ZIP Mod 安装
- ZIP 安装依赖预览
- 使用临时目录进行更安全的 ZIP 安装
- Profiles v0.1

### 已知问题

- Profiles 交互仍然比较粗糙
- UI 还需要专门做一次交互优化
- 在线 Mod 下载暂未实现
- Mod 更新检测暂未实现

---

## 技术栈

Junimo Box 使用以下技术开发：

- Tauri
- Vue 3
- TypeScript
- Vite
- Rust

---

## 开发环境推荐

推荐使用：

- VS Code
- Vue - Official
- Tauri
- rust-analyzer

---

## 本地运行

安装依赖：

```bash
npm install
```

启动开发环境：

```bash
npm run tauri dev
```

如果 1420 端口被残留进程占用，可以先运行：

```bash
npm run dev:free-port
```

构建应用：

```bash
npm run tauri build
```

项目结构
junimo-box/
├─ src/              # Vue 前端代码
├─ src-tauri/        # Tauri / Rust 后端代码
├─ public/           # 静态资源
├─ package.json
├─ README.md
└─ LICENSE
版本路线图
MVP 第一版：本地 Mod 管理器

目标：能选择游戏目录、扫描 Mods 文件夹、启用 / 禁用 Mod，并启动游戏。

第二版：更完整的本地管理工具

目标：加入 Mod 详情、ZIP 安装、拖拽安装、基础依赖检测、日志查看和问题报告导出。

第三版：资源库与一键安装

目标：支持在启动器内搜索、下载、安装和更新 Mod，并处理依赖关系。

第四版：多配置方案

目标：支持为不同玩法保存不同 Mod 组合，例如美化整合、多人联机、剧情扩展等。

第五版：稳定性与完整体验

目标：强化冲突检测、日志分析、备份还原、新手引导和完整像素风 UI。

项目愿景

Junimo Box 希望让玩家感觉：

我不需要再手动折腾一堆文件夹了，打开祝尼魔盒就能管理我的星露谷。
