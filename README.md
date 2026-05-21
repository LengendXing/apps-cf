<div align="center">

# Apps Startpage

**A sleek, self-hosted app dashboard & bookmark manager powered by Rust + Cloudflare Workers**

[![Deploy](https://github.com/LengendXing/apps-cf/actions/workflows/deploy.yml/badge.svg)](https://github.com/LengendXing/apps-cf/actions/workflows/deploy.yml)
[![Rust](https://img.shields.io/badge/Rust-1.77%2B-orange?logo=rust)](https://www.rust-lang.org/)
[![Vue](https://img.shields.io/badge/Vue-3.5-42b883?logo=vue.js)](https://vuejs.org/)
[![Cloudflare Workers](https://img.shields.io/badge/CF-Workers-F38020?logo=cloudflare)](https://workers.cloudflare.com/)

[Live Demo](https://apps.sential.eu.cc) · [Report Bug](https://github.com/LengendXing/apps-cf/issues) · [Request Feature](https://github.com/LengendXing/apps-cf/issues)

</div>

---

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

# Run both simultaneously for local dev
```

---

## License

MIT — do whatever you want with it.

---

<div align="center">

Built with Rust & Vue on Cloudflare Workers

</div>
