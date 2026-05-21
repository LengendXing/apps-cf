<div align="center">

# Apps Startpage

**A sleek, self-hosted app dashboard & bookmark manager powered by Rust + Cloudflare Workers**

[![Deploy](https://github.com/LengendXing/apps-cf/actions/workflows/deploy.yml/badge.svg)](https://github.com/LengendXing/apps-cf/actions/workflows/deploy.yml)
[![Rust](https://img.shields.io/badge/Rust-1.77%2B-orange?logo=rust)](https://www.rust-lang.org/)
[![Vue](https://img.shields.io/badge/Vue-3.5-42b883?logo=vue.js)](https://vuejs.org/)
[![Cloudflare Workers](https://img.shields.io/badge/CF-Workers-F38020?logo=cloudflare)](https://workers.cloudflare.com/)

[🌐 English](#english) · [🇨🇳 中文](#中文) · [🇯🇵 日本語](#日本語)

[Live Demo](https://apps.sential.eu.cc) · [Report Bug](https://github.com/LengendXing/apps-cf/issues) · [Request Feature](https://github.com/LengendXing/apps-cf/issues)

</div>

---

# English

## What is this?

Apps Startpage is a personal app dashboard that sits on your domain. Think of it as a **prettier, faster, self-hosted alternative** to static bookmark pages — with a built-in admin panel, config manager, and notes system. All running on Cloudflare's edge network for free.

```
┌─────────────────────────────────────────────┐
│  C-end (Public)        │  Admin Panel       │
│  ┌──────────────────┐  │  ┌───────────────┐ │
│  │ Password Gate    │  │  │ Dashboard      │ │
│  │ App Grid         │  │  │ Tools CRUD     │ │
│  │ Config Viewer    │  │  │ Scripts CRUD   │ │
│  │ Notes Editor     │  │  │ Configs CRUD   │ │
│  │ Dark/Light Mode  │  │  │ Users Mgmt    │ │
│  │ i18n zh/en       │  │  │ Audit Logs    │ │
│  └──────────────────┘  │  │ Notes Mgmt    │ │
│                        │  │ System Config  │ │
│                        │  │ Import/Export  │ │
│                        │  └───────────────┘ │
└─────────────────────────────────────────────┘
```

---

## Features

### Client-facing (C-end)

| Feature | Description |
|---------|-------------|
| **Access Password** | Password gate before showing any content |
| **App Grid** | 7-column responsive grid with brand-color icons (via [Simple Icons](https://simpleicons.org)) |
| **Category Sidebar** | Filter apps by category with left sidebar |
| **Search** | Real-time fuzzy search across app names and descriptions |
| **Info Drawer** | macOS-style slide-in drawer showing app details, tags, platforms, versions |
| **Script Viewer** | View installation/usage scripts per tool in the info drawer |
| **Config Viewer** | Browse and copy configuration files (properties/yaml/toml/json/text) |
| **Notes Editor** | CotEditor-style note editor with line numbers, auto-save (5s debounce), manual save button |
| **Folder Tree** | Collapsible folder tree in sidebar, create/delete folders and notes |
| **Dark Mode** | Pure B&W gray theme, toggle sun/moon icon in header |
| **i18n** | Chinese / English, persisted in localStorage |
| **Skeleton Loading** | Shimmer skeleton screens while data loads |
| **macOS Dialogs** | All prompts/confirms use soft macOS-style dialogs, zero browser popups |

### Admin Panel

| Feature | Description |
|---------|-------------|
| **GitHub OAuth Login** | Sign in with GitHub, JWT-based auth |
| **Dashboard** | Stats overview (users, categories, tools) + access password management |
| **Tools CRUD** | Modal-based add/edit with icon, tags, platforms, versions, featured flag |
| **Scripts CRUD** | Per-tool script management with platform and tags |
| **Configs CRUD** | Paginated config list with 5 format types (properties/yaml/toml/json/text) |
| **Notes CRUD** | Folder + note management, inline folder creation, drawer detail view |
| **Users Management** | Enable/disable/delete users, role assignment |
| **Audit Logs** | Filterable operation history (action, target type, target ID) |
| **System Config** | Menu layout toggle (left sidebar / top navigation) |
| **Import/Export** | One-click JSON export, JSON file import with deduplication |
| **macOS Dialogs** | All confirmations use macOS-style dialogs, zero browser popups |

---

## Tech Stack

### Backend — Rust on Cloudflare Workers

| Component | Tech |
|-----------|------|
| Runtime | [Cloudflare Workers](https://developers.cloudflare.com/workers/) (WASM) |
| Language | Rust (edition 2021) |
| HTTP Framework | [`worker`](https://docs.rs/worker) crate v0.8 |
| Storage | Cloudflare KV (`APPS_DATA` namespace) |
| Auth | JWT (custom SHA256 signing), GitHub OAuth |
| Build | [`worker-build`](https://github.com/cloudflare/workers-rs) → WASM |
| Size | ~24KB compiled WASM |

**Backend structure** (`src/lib.rs`, ~1000 lines):

- 8 data models: `User`, `Category`, `Tool`, `Script`, `Config`, `AuditLog`, `NoteFolder`, `Note`
- 25+ API routes covering all CRUD operations
- Auto-seeding with default admin user and categories
- `check_access_or_admin()` — dual auth: admin JWT *or* `X-Access-Password` header for C-end
- `log_action()` — audit trail for all admin operations
- Pagination on tools, configs, notes, audit logs

### Frontend — Vue 3 SPA

| Component | Tech |
|-----------|------|
| Framework | Vue 3.5 + Composition API (`<script setup>`) |
| Router | Vue Router 4 (history mode) |
| State | Pinia |
| HTTP | Axios (auto-unwrapping `ApiResponse`) |
| i18n | vue-i18n 10 (zh/en) |
| CSS | Tailwind CSS 3 + CSS custom properties |
| Build | Vite 6 |

**Frontend structure:**

```
frontend/src/
├── api/index.js          # Axios instance + all API modules
├── assets/main.css       # Global styles, macOS animations, skeleton, dialogs
├── components/
│   └── MainLayout.vue    # Admin sidebar layout
├── i18n/
│   ├── index.js          # vue-i18n setup
│   ├── zh.js             # Chinese translations
│   └── en.js             # English translations
├── router/index.js       # Route definitions + auth guards
├── stores/
│   └── auth.js           # Pinia auth store (token, user)
├── views/
│   ├── Home.vue          # C-end: password gate, app grid, notes editor
│   ├── Login.vue         # Admin: GitHub OAuth login
│   ├── Dashboard.vue     # Admin: stats + settings
│   ├── AdminTools.vue    # Admin: tools + scripts + versions CRUD
│   ├── AdminConfigs.vue  # Admin: configs CRUD + import/export
│   ├── AdminNotes.vue    # Admin: folders + notes CRUD
│   ├── AdminUsers.vue    # Admin: user management
│   ├── AdminAudit.vue    # Admin: audit log viewer
│   └── SystemConfig.vue  # Admin: layout settings
├── App.vue
└── main.js
```

---

## API Reference

All endpoints return `{ code: 0, message: "ok", data: ... }` on success.

### Public (no auth)

| Method | Path | Description |
|--------|------|-------------|
| `GET` | `/api/categories` | List all categories |
| `GET` | `/api/tools` | List tools (paginated) |
| `GET` | `/api/scripts` | List scripts (filterable by tool_id) |
| `GET` | `/api/configs` | List configs (paginated) |
| `GET` | `/api/note-folders` | List all note folders |
| `GET` | `/api/notes` | List notes (paginated, filterable) |
| `GET` | `/api/notes/:id` | Get note detail |
| `POST` | `/api/settings/access-password/verify` | Verify access password |

### C-end Auth (X-Access-Password header)

| Method | Path | Description |
|--------|------|-------------|
| `POST` | `/api/note-folders` | Create folder |
| `PUT` | `/api/note-folders/:id` | Update folder |
| `DELETE` | `/api/note-folders/:id` | Delete folder |
| `POST` | `/api/notes` | Create note |
| `PUT` | `/api/notes/:id` | Update note (auto-save) |
| `DELETE` | `/api/notes/:id` | Delete note |

### Admin Auth (JWT Bearer token)

| Method | Path | Description |
|--------|------|-------------|
| `POST` | `/api/auth/login` | Admin login |
| `GET` | `/api/auth/me` | Current user info |
| `GET` | `/api/auth/github` | GitHub OAuth redirect |
| `GET` | `/api/auth/github/callback` | GitHub OAuth callback |
| `POST` | `/api/categories` | Create category |
| `PUT` | `/api/categories/:id` | Update category |
| `DELETE` | `/api/categories/:id` | Delete category |
| `POST` | `/api/tools` | Create tool |
| `PUT` | `/api/tools/:id` | Update tool |
| `DELETE` | `/api/tools/:id` | Delete tool |
| `POST` | `/api/scripts` | Create script |
| `PUT` | `/api/scripts/:id` | Update script |
| `DELETE` | `/api/scripts/:id` | Delete script |
| `POST` | `/api/configs` | Create config |
| `PUT` | `/api/configs/:id` | Update config |
| `DELETE` | `/api/configs/:id` | Delete config |
| `POST` | `/api/configs/:id/copy` | Increment copy count |
| `PUT` | `/api/users/:id` | Update user |
| `DELETE` | `/api/users/:id` | Delete user |
| `GET` | `/api/users` | List users |
| `GET` | `/api/audit-logs` | List audit logs (paginated) |
| `GET` | `/api/stats` | Dashboard stats |
| `GET` | `/api/settings/access-password` | Get access password |
| `PUT` | `/api/settings/access-password` | Update access password |
| `GET` | `/api/settings/system-config` | Get system config |
| `PUT` | `/api/settings/system-config` | Update system config |
| `GET` | `/api/export` | Export all data (categories + tools + configs) |
| `POST` | `/api/import` | Import data (deduplicates by name) |

---

## Deployment

### Prerequisites

- [Cloudflare account](https://dash.cloudflare.com/) with Workers enabled
- GitHub account (for OAuth login)
- Rust toolchain with `wasm32-unknown-unknown` target

### 1. Clone & Configure

```bash
git clone https://github.com/LengendXing/apps-cf.git
cd apps-cf
```

Edit `wrangler.toml`:

```toml
# Your domain
routes = [{ pattern = "your-domain.com", custom_domain = true }]

# Create a KV namespace: wrangler kv:namespace create APPS_DATA
[[kv_namespaces]]
binding = "APPS_DATA"
id = "your-kv-namespace-id"

# GitHub OAuth app credentials
[vars]
APP_URL = "https://your-domain.com"
GITHUB_CLIENT_ID = "your-github-client-id"
```

### 2. Set Secrets

```bash
# Cloudflare API token (for CI/CD)
wrangler secret put GITHUB_CLIENT_SECRET
```

### 3. GitHub Actions (Auto Deploy)

Push to `main` branch triggers auto-deployment via `.github/workflows/deploy.yml`.

Required GitHub Secrets:

| Secret | Description |
|--------|-------------|
| `CF_API_TOKEN` | Cloudflare API token |
| `CF_ACCOUNT_ID` | Cloudflare account ID |
| `GH_OAUTH_CLIENT_SECRET` | GitHub OAuth app client secret |

### 4. Manual Deploy

```bash
# Install tools
rustup target add wasm32-unknown-unknown
cargo install worker-build

# Build frontend
cd frontend && npm install && npm run build && cd ..

# Build & deploy
CLOUDFLARE_API_TOKEN=your-token npx wrangler deploy
```

### 5. First Login

- Default admin: `admin` / `Admin@123`
- Change password immediately after first login
- Access password for C-end: set via Dashboard > Access Settings

---

## Architecture

```
                    ┌──────────────┐
                    │   Browser    │
                    └──────┬───────┘
                           │
                    ┌──────▼───────┐
                    │  CF Workers   │  ← Rust (WASM, ~24KB)
                    │  + KV Store   │  ← All data in KV
                    │  + Static     │  ← Serves Vue SPA
                    └──────┬───────┘
                           │
              ┌────────────┼────────────┐
              │            │            │
        ┌─────▼─────┐ ┌───▼────┐ ┌────▼─────┐
        │  C-end SPA │ │ Admin  │ │ GitHub   │
        │  (Home)    │ │ Panel  │ │ OAuth    │
        └────────────┘ └────────┘ └──────────┘
```

**Key design decisions:**

- **Zero database** — Everything in Cloudflare KV. No D1, no Postgres. Perfect for personal use.
- **Single binary** — Backend is one Rust file (`src/lib.rs`) compiled to WASM.
- **Dual auth** — Admin gets JWT via GitHub OAuth. C-end gets access password verified + sent as `X-Access-Password` header for note creation.
- **SPA with SSR fallback** — Workers serves `index.html` for all routes, Vue Router handles client-side routing.
- **Auto-seeding** — First run creates admin user, default categories, and sets access password.

---

## Data Format (Import/Export)

```json
{
  "categories": [
    { "name": "Development", "icon": "💻", "sort_order": 0 }
  ],
  "tools": [
    {
      "name": "VS Code",
      "url": "https://code.visualstudio.com",
      "category_id": 1,
      "icon": "https://cdn.simpleicons.org/visualstudiocode",
      "description": "Code editor",
      "tags": ["editor", "microsoft"],
      "platforms": ["mac", "windows", "linux"],
      "is_featured": false,
      "sort_order": 0,
      "versions": [
        { "version": "1.90", "url": "https://update.code.visualstudio.com/latest/darwin/stable" }
      ]
    }
  ],
  "configs": [
    {
      "name": "settings.json",
      "format": "json",
      "content": "{ \"theme\": \"dark\" }"
    }
  ]
}
```

Import deduplicates by name — existing items with the same name are skipped.

---

## Design Language

| Surface | Style |
|---------|-------|
| **C-end** | Pure black/white/gray — zero color accent. Minimal, content-first |
| **Admin** | Apple Human Interface — blue accent (#007AFF), SF-style rounded cards, 16px radius |
| **Animations** | macOS standard ease `cubic-bezier(0.25, 0.1, 0.25, 1)`, 400ms drawer, 350ms modal |
| **Icons** | SVG inline (SF Symbols style) — no emoji in UI |
| **Tool logos** | [Simple Icons CDN](https://simpleicons.org) with brand colors |
| **Dialogs** | Custom macOS-style soft dialogs everywhere — zero `alert()`/`confirm()`/`prompt()` |
| **Loading** | Skeleton shimmer screens (C-end) + macOS spinner in header |

---

## Error Codes

| Code | Meaning |
|------|---------|
| `1001` | Unauthorized / token expired |
| `1002` | Insufficient permissions |
| `1003` | Invalid parameters |
| `1004` | Resource not found |
| `500` | Internal server error |

---

## Development

```bash
# Frontend dev server
cd frontend && npm run dev

# Backend local (requires wrangler)
CLOUDFLARE_API_TOKEN=your-token npx wrangler dev
```

---

## License

MIT — do whatever you want with it.

---

<div align="center">

Built with Rust & Vue on Cloudflare Workers

**Special Thanks to [Visual Studio Code](https://code.visualstudio.com)** — The editor that makes everything possible

**Special Thanks to [[ClodHost](https://clodhost.com)** — Reliable hosting infrastructure

</div>

---

---

# 中文

## 这是什么？

Apps Startpage 是一个部署在你自己域名上的个人应用仪表盘。你可以把它理解为一个**更漂亮、更快速、自托管的静态书签页替代品**——内置管理后台、配置管理器和笔记系统。全部运行在 Cloudflare 边缘网络上，免费。

```
┌─────────────────────────────────────────────┐
│  用户端（公开）          │  管理后台         │
│  ┌──────────────────┐  │  ┌───────────────┐ │
│  │ 密码验证门       │  │  │ 仪表盘        │ │
│  │ 应用网格         │  │  │ 工具管理       │ │
│  │ 配置查看器       │  │  │ 脚本管理       │ │
│  │ 笔记编辑器       │  │  │ 配置管理       │ │
│  │ 深色/浅色模式    │  │  │ 用户管理       │ │
│  │ 中英文切换       │  │  │ 审计日志       │ │
│  └──────────────────┘  │  │ 笔记管理       │ │
│                        │  │ 系统配置       │ │
│                        │  │ 导入/导出      │ │
│                        │  └───────────────┘ │
└─────────────────────────────────────────────┘
```

---

## 功能特性

### 用户端功能

| 功能 | 说明 |
|------|------|
| **访问密码** | 进入前需验证密码，保护内容隐私 |
| **应用网格** | 7 列响应式网格，品牌色图标（通过 [Simple Icons](https://simpleicons.org)） |
| **分类侧栏** | 左侧分类筛选 |
| **实时搜索** | 模糊搜索应用名称和描述 |
| **详情抽屉** | macOS 风格滑入抽屉，展示应用详情、标签、平台、版本 |
| **脚本查看** | 在详情抽屉中查看安装/使用脚本 |
| **配置查看** | 浏览和复制配置文件（properties/yaml/toml/json/text） |
| **笔记编辑器** | CotEditor 风格编辑器，带行号、自动保存（5秒防抖）、手动保存按钮 |
| **文件夹树** | 可折叠文件夹树，创建/删除文件夹和笔记 |
| **深色模式** | 纯黑白灰主题，太阳/月亮切换图标 |
| **多语言** | 中文/英文，持久化到 localStorage |
| **骨架屏** | 数据加载时展示微光骨架屏 |
| **macOS 弹窗** | 所有确认/输入均使用 macOS 柔和弹窗，零浏览器原生弹窗 |

### 管理后台功能

| 功能 | 说明 |
|------|------|
| **GitHub OAuth 登录** | GitHub 登录，JWT 认证 |
| **仪表盘** | 统计概览（用户、分类、工具）+ 访问密码管理 |
| **工具 CRUD** | 弹窗式新增/编辑，支持图标、标签、平台、版本、推荐标记 |
| **脚本 CRUD** | 按工具管理脚本，支持平台和标签 |
| **配置 CRUD** | 分页配置列表，5 种格式（properties/yaml/toml/json/text） |
| **笔记 CRUD** | 文件夹 + 笔记管理，内联创建文件夹，抽屉详情 |
| **用户管理** | 启用/禁用/删除用户，角色分配 |
| **审计日志** | 可筛选的操作记录（操作、目标类型、目标ID） |
| **系统配置** | 菜单布局切换（左侧栏/顶部导航） |
| **导入/导出** | 一键 JSON 导出，JSON 文件导入（自动去重） |
| **macOS 弹窗** | 所有确认均使用 macOS 风格弹窗，零浏览器原生弹窗 |

---

## 技术栈

### 后端 — Rust + Cloudflare Workers

| 组件 | 技术 |
|------|------|
| 运行时 | [Cloudflare Workers](https://developers.cloudflare.com/workers/) (WASM) |
| 语言 | Rust (edition 2021) |
| HTTP 框架 | [`worker`](https://docs.rs/worker) crate v0.8 |
| 存储 | Cloudflare KV (`APPS_DATA` 命名空间) |
| 认证 | JWT (自定义 SHA256 签名)，GitHub OAuth |
| 构建 | [`worker-build`](https://github.com/cloudflare/workers-rs) → WASM |
| 体积 | 编译后 WASM 约 24KB |

**后端结构** (`src/lib.rs`，约 1000 行)：

- 8 个数据模型：`User`、`Category`、`Tool`、`Script`、`Config`、`AuditLog`、`NoteFolder`、`Note`
- 25+ API 路由，覆盖所有 CRUD 操作
- 自动初始化默认管理员和分类
- `check_access_or_admin()` — 双重认证：管理员 JWT 或 `X-Access-Password` 请求头
- `log_action()` — 所有管理员操作的审计记录
- 工具、配置、笔记、审计日志均支持分页

### 前端 — Vue 3 SPA

| 组件 | 技术 |
|------|------|
| 框架 | Vue 3.5 + Composition API (`<script setup>`) |
| 路由 | Vue Router 4 (history 模式) |
| 状态 | Pinia |
| HTTP | Axios (自动解包 `ApiResponse`) |
| 国际化 | vue-i18n 10 (中文/英文) |
| CSS | Tailwind CSS 3 + CSS 自定义属性 |
| 构建 | Vite 6 |

---

## 部署

### 前置条件

- [Cloudflare 账号](https://dash.cloudflare.com/)，已启用 Workers
- GitHub 账号（用于 OAuth 登录）
- Rust 工具链 + `wasm32-unknown-unknown` target

### 1. 克隆 & 配置

```bash
git clone https://github.com/LengendXing/apps-cf.git
cd apps-cf
```

修改 `wrangler.toml`：

```toml
routes = [{ pattern = "your-domain.com", custom_domain = true }]

[[kv_namespaces]]
binding = "APPS_DATA"
id = "your-kv-namespace-id"

[vars]
APP_URL = "https://your-domain.com"
GITHUB_CLIENT_ID = "your-github-client-id"
```

### 2. 设置密钥

```bash
wrangler secret put GITHUB_CLIENT_SECRET
```

### 3. GitHub Actions 自动部署

推送 `main` 分支自动触发部署。需要在 GitHub 仓库设置以下 Secrets：

| Secret | 说明 |
|--------|------|
| `CF_API_TOKEN` | Cloudflare API Token |
| `CF_ACCOUNT_ID` | Cloudflare 账号 ID |
| `GH_OAUTH_CLIENT_SECRET` | GitHub OAuth 应用密钥 |

### 4. 手动部署

```bash
rustup target add wasm32-unknown-unknown
cargo install worker-build
cd frontend && npm install && npm run build && cd ..
CLOUDFLARE_API_TOKEN=your-token npx wrangler deploy
```

### 5. 首次登录

- 默认管理员：`admin` / `Admin@123`
- 首次登录后请立即修改密码
- 用户端访问密码：通过 仪表盘 > 访问设置 配置

---

## 架构

```
                    ┌──────────────┐
                    │    浏览器     │
                    └──────┬───────┘
                           │
                    ┌──────▼───────┐
                    │  CF Workers   │  ← Rust (WASM, ~24KB)
                    │  + KV Store   │  ← 所有数据存储在 KV
                    │  + 静态资源   │  ← 托管 Vue SPA
                    └──────┬───────┘
                           │
              ┌────────────┼────────────┐
              │            │            │
        ┌─────▼─────┐ ┌───▼────┐ ┌────▼─────┐
        │  用户端 SPA │ │ 管理后台│ │ GitHub   │
        │  (Home)    │ │  面板   │ │  OAuth   │
        └────────────┘ └────────┘ └──────────┘
```

**核心设计决策：**

- **零数据库** — 所有数据存储在 Cloudflare KV，不需要 D1 或 Postgres，个人使用完美
- **单文件后端** — 后端仅一个 Rust 文件 (`src/lib.rs`)，编译为 WASM
- **双重认证** — 管理员通过 GitHub OAuth 获取 JWT；用户端通过访问密码验证，写入操作通过 `X-Access-Password` 请求头
- **SPA + 边缘渲染** — Workers 对所有路由返回 `index.html`，Vue Router 处理客户端路由
- **自动初始化** — 首次运行自动创建管理员、默认分类和访问密码

---

## 导入/导出数据格式

```json
{
  "categories": [{ "name": "开发工具", "icon": "💻", "sort_order": 0 }],
  "tools": [{
    "name": "VS Code", "url": "https://code.visualstudio.com", "category_id": 1,
    "icon": "https://cdn.simpleicons.org/visualstudiocode",
    "tags": ["编辑器"], "platforms": ["mac", "windows", "linux"],
    "versions": [{ "version": "1.90", "url": "https://..." }]
  }],
  "configs": [{ "name": "settings.json", "format": "json", "content": "{ \"theme\": \"dark\" }" }]
}
```

导入时按名称去重——同名项目会被跳过。

---

## 错误码

| 错误码 | 含义 |
|--------|------|
| `1001` | 未授权 / Token 过期 |
| `1002` | 权限不足 |
| `1003` | 参数错误 |
| `1004` | 资源不存在 |
| `500` | 内部服务器错误 |

---

## 开发

```bash
# 前端开发服务器
cd frontend && npm run dev

# 后端本地运行（需要 wrangler）
CLOUDFLARE_API_TOKEN=your-token npx wrangler dev
```

---

## 许可证

MIT — 随便用。

---

<div align="center">

使用 Rust & Vue 构建于 Cloudflare Workers

**特别感谢 [Visual Studio Code](https://code.visualstudio.com)** — 让一切成为可能的编辑器

**特别感谢 [[ClodHost](https://clodhost.com)** — 可靠的托管基础设施

</div>

---

---

# 日本語

## これは何？

Apps Startpage は、あなたのドメインに配置する個人アプリダッシュボードです。静的なブックマークページの**より美しく、より高速な、セルフホスト型の代替**とお考えください——管理パネル、設定マネージャー、ノートシステムを内蔵。すべて Cloudflare のエッジネットワーク上で無料で動作します。

```
┌─────────────────────────────────────────────┐
│  ユーザー側（公開）      │  管理パネル        │
│  ┌──────────────────┐  │  ┌───────────────┐ │
│  │ パスワード認証    │  │  │ ダッシュボード │ │
│  │ アプリグリッド    │  │  │ ツール管理     │ │
│  │ 設定ビューア     │  │  │ スクリプト管理 │ │
│  │ ノートエディタ   │  │  │ 設定管理       │ │
│  │ ダーク/ライト    │  │  │ ユーザー管理   │ │
│  │ 多言語対応       │  │  │ 監査ログ       │ │
│  └──────────────────┘  │  │ ノート管理     │ │
│                        │  │ システム設定   │ │
│                        │  │ インポート/    │ │
│                        │  │ エクスポート   │ │
│                        │  └───────────────┘ │
└─────────────────────────────────────────────┘
```

---

## 機能

### ユーザー側機能

| 機能 | 説明 |
|------|------|
| **アクセスパスワード** | コンテンツ表示前のパスワード認証 |
| **アプリグリッド** | 7列レスポンシブグリッド、ブランドカラーアイコン（[Simple Icons](https://simpleicons.org) 経由） |
| **カテゴリサイドバー** | 左サイドバーによるカテゴリフィルタリング |
| **リアルタイム検索** | アプリ名と説明のあいまい検索 |
| **詳細ドロワー** | macOS スタイルのスライドインドロワー、アプリ詳細/タグ/プラットフォーム/バージョン表示 |
| **スクリプトビューア** | ツールごとのインストール/使用スクリプト表示 |
| **設定ビューア** | 設定ファイルの閲覧とコピー（properties/yaml/toml/json/text） |
| **ノートエディタ** | CotEditor スタイルのエディタ、行番号付き、自動保存（5秒デバウンス）、手動保存ボタン |
| **フォルダツリー** | 折りたたみ可能なフォルダツリー、フォルダ/ノートの作成・削除 |
| **ダークモード** | 純粋な白黒グレーテーマ、太陽/月の切り替えアイコン |
| **多言語** | 中国語/英語、localStorage に保存 |
| **スケルトンローディング** | データ読み込み中のシマースケルトン画面 |
| **macOS ダイアログ** | すべての確認/入力に macOS スタイルのソフトダイアログ、ブラウザネイティブポップアップゼロ |

### 管理パネル機能

| 機能 | 説明 |
|------|------|
| **GitHub OAuth ログイン** | GitHub ログイン、JWT 認証 |
| **ダッシュボード** | 統計概要（ユーザー、カテゴリ、ツール）+ アクセスパスワード管理 |
| **ツール CRUD** | モーダルベースの追加/編集、アイコン/タグ/プラットフォーム/バージョン/注目フラグ対応 |
| **スクリプト CRUD** | ツールごとのスクリプト管理、プラットフォームとタグ対応 |
| **設定 CRUD** | ページネーション付き設定リスト、5種類のフォーマット（properties/yaml/toml/json/text） |
| **ノート CRUD** | フォルダ + ノート管理、インラインフォルダ作成、ドロワー詳細表示 |
| **ユーザー管理** | ユーザーの有効化/無効化/削除、ロール割り当て |
| **監査ログ** | フィルタ可能な操作履歴（操作、対象タイプ、対象ID） |
| **システム設定** | メニューレイアウト切替（左サイドバー/トップナビゲーション） |
| **インポート/エクスポート** | ワンクリック JSON エクスポート、JSON ファイルインポート（重複排除付き） |
| **macOS ダイアログ** | すべての確認に macOS スタイルダイアログ、ブラウザネイティブポップアップゼロ |

---

## 技術スタック

### バックエンド — Rust + Cloudflare Workers

| コンポーネント | 技術 |
|---------------|------|
| ランタイム | [Cloudflare Workers](https://developers.cloudflare.com/workers/) (WASM) |
| 言語 | Rust (edition 2021) |
| HTTP フレームワーク | [`worker`](https://docs.rs/worker) crate v0.8 |
| ストレージ | Cloudflare KV (`APPS_DATA` ネームスペース) |
| 認証 | JWT (カスタム SHA256 署名)、GitHub OAuth |
| ビルド | [`worker-build`](https://github.com/cloudflare/workers-rs) → WASM |
| サイズ | コンパイル後 WASM 約 24KB |

### フロントエンド — Vue 3 SPA

| コンポーネント | 技術 |
|---------------|------|
| フレームワーク | Vue 3.5 + Composition API (`<script setup>`) |
| ルーター | Vue Router 4 (history モード) |
| 状態管理 | Pinia |
| HTTP | Axios (自動アンラップ `ApiResponse`) |
| 国際化 | vue-i18n 10 (中国語/英語) |
| CSS | Tailwind CSS 3 + CSS カスタムプロパティ |
| ビルド | Vite 6 |

---

## デプロイ

### 前提条件

- Workers が有効な [Cloudflare アカウント](https://dash.cloudflare.com/)
- GitHub アカウント（OAuth ログイン用）
- `wasm32-unknown-unknown` ターゲット付き Rust ツールチェーン

### 1. クローン & 設定

```bash
git clone https://github.com/LengendXing/apps-cf.git
cd apps-cf
```

`wrangler.toml` を編集：

```toml
routes = [{ pattern = "your-domain.com", custom_domain = true }]

[[kv_namespaces]]
binding = "APPS_DATA"
id = "your-kv-namespace-id"

[vars]
APP_URL = "https://your-domain.com"
GITHUB_CLIENT_ID = "your-github-client-id"
```

### 2. シークレット設定

```bash
wrangler secret put GITHUB_CLIENT_SECRET
```

### 3. GitHub Actions 自動デプロイ

`main` ブランチへのプッシュで自動デプロイがトリガーされます。

必要な GitHub Secrets：

| Secret | 説明 |
|--------|------|
| `CF_API_TOKEN` | Cloudflare API トークン |
| `CF_ACCOUNT_ID` | Cloudflare アカウント ID |
| `GH_OAUTH_CLIENT_SECRET` | GitHub OAuth アプリクライアントシークレット |

### 4. 手動デプロイ

```bash
rustup target add wasm32-unknown-unknown
cargo install worker-build
cd frontend && npm install && npm run build && cd ..
CLOUDFLARE_API_TOKEN=your-token npx wrangler deploy
```

### 5. 初回ログイン

- デフォルト管理者：`admin` / `Admin@123`
- 初回ログイン後、すぐにパスワードを変更してください
- ユーザー側アクセスパスワード：ダッシュボード > アクセス設定 で設定

---

## アーキテクチャ

```
                    ┌──────────────┐
                    │   ブラウザ   │
                    └──────┬───────┘
                           │
                    ┌──────▼───────┐
                    │  CF Workers   │  ← Rust (WASM, ~24KB)
                    │  + KV Store   │  ← 全データ KV 保存
                    │  + 静的資産   │  ← Vue SPA 配信
                    └──────┬───────┘
                           │
              ┌────────────┼────────────┐
              │            │            │
        ┌─────▼─────┐ ┌───▼────┐ ┌────▼─────┐
        │ ユーザーSPA│ │ 管理者  │ │ GitHub   │
        │  (Home)   │ │ パネル  │ │  OAuth   │
        └───────────┘ └────────┘ └──────────┘
```

**主要な設計決定：**

- **データベース不要** — すべて Cloudflare KV に保存。D1 も Postgres も不要。個人利用に最適
- **単一バイナリ** — バックエンドは1つの Rust ファイル (`src/lib.rs`) を WASM にコンパイル
- **デュアル認証** — 管理者は GitHub OAuth 経由で JWT を取得。ユーザー側はアクセスパスワードで認証、書き込み操作は `X-Access-Password` ヘッダーを使用
- **SPA + エッジフォールバック** — Workers は全ルートに `index.html` を返し、Vue Router がクライアントサイドルーティングを処理
- **自動シード** — 初回実行時に管理者ユーザー、デフォルトカテゴリ、アクセスパスワードを自動作成

---

## インポート/エクスポートデータ形式

```json
{
  "categories": [{ "name": "開発ツール", "icon": "💻", "sort_order": 0 }],
  "tools": [{
    "name": "VS Code", "url": "https://code.visualstudio.com", "category_id": 1,
    "icon": "https://cdn.simpleicons.org/visualstudiocode",
    "tags": ["エディタ"], "platforms": ["mac", "windows", "linux"],
    "versions": [{ "version": "1.90", "url": "https://..." }]
  }],
  "configs": [{ "name": "settings.json", "format": "json", "content": "{ \"theme\": \"dark\" }" }]
}
```

インポート時は名前で重複排除——同名項目はスキップされます。

---

## エラーコード

| コード | 意味 |
|--------|------|
| `1001` | 認証エラー / トークン期限切れ |
| `1002` | 権限不足 |
| `1003` | パラメータエラー |
| `1004` | リソース未検出 |
| `500` | 内部サーバーエラー |

---

## 開発

```bash
# フロントエンド開発サーバー
cd frontend && npm run dev

# バックエンドローカル実行（wrangler 必要）
CLOUDFLARE_API_TOKEN=your-token npx wrangler dev
```

---

## ライセンス

MIT — ご自由にどうぞ。

---

<div align="center">

Rust & Vue で Cloudflare Workers 上に構築

**[Visual Studio Code](https://code.visualstudio.com) に特別な感謝** — すべてを可能にするエディタ

**[[ClodHost](https://clodhost.com) に特別な感謝** — 信頼性の高いホスティングインフラ

</div>
