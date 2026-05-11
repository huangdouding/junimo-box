# Junimo Box / 祝尼魔盒

> 本项目由 AI（Claude）辅助生成，是一个学习与实验性质的项目。

Junimo Box 是一个面向《星露谷物语》的第三方 Mod 管理器兼游戏启动器，帮助玩家更轻松地管理 Mod、启动游戏、切换配置和排查问题。

---

## 功能

- **游戏目录管理** — 自动检测或手动选择 Stardew Valley 安装路径
- **SMAPI 管理** — 检测、安装、更新 SMAPI，启动 Mod 版与原版
- **Mod 管理** — 扫描、启用/禁用、搜索筛选、依赖检测、类型标签
- **批量操作** — Ctrl/Shift 多选，批量启用/禁用/删除
- **ZIP 安装** — 拖拽或选择 ZIP 文件预览安装，含冲突检测与依赖检查
- **配置方案 (Profiles)** — 创建、复制、导出/导入不同的 Mod 组合
- **回收站** — 删除的 Mod 进入回收站，支持还原和清空
- **下载队列** — 支持 NXM 协议和 URL ZIP 下载，含进度显示和取消
- **安装历史** — 记录每次安装的来源和冲突处理方式
- **SMAPI 日志** — 日志查看与基础诊断
- **Toast 通知** — 操作结果轻量提示，支持撤销
- **键盘快捷键** — Ctrl+F 搜索、Ctrl+R 扫描、Escape 关闭面板
- **新手引导** — 首次使用 4 步引导配置
- **备份与还原** — 导出/导入 Mod 启用状态

---

## 技术栈

| 层 | 技术 |
|---|---|
| 前端 | Vue 3 + TypeScript + Vite |
| 后端 | Rust / Tauri 2 |
| 包管理 | pnpm |

---

## 开发

```bash
# 安装依赖
pnpm install

# 启动开发环境
pnpm tauri dev

# 构建
pnpm tauri build
```

---

## 下载

从 [Releases](https://github.com/huangdouding/junimo-box/releases) 页面下载最新安装包。

---

## 许可

MIT
