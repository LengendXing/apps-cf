# Apps Startpage - 变更记录

## v0.3.0 - 2026-05-20
### 变更内容
- 修复 GitHub OAuth 登录：GITHUB_CLIENT_ID 加入 wrangler.toml [vars]，GITHUB_CLIENT_SECRET 通过 deploy.yml 自动设置 CF Secret
- 重建前端源码（Vue3 + Vite + Tailwind），原仓库仅有编译产物无源码
- 登录页背景根据北京时间动态切换：6:00-18:00 白色，18:00-6:00 黑色
- 修复 OAuth 回调 token 处理：前端从 URL query 读取 token 并存入 localStorage
- 更新 deploy.yml：部署前自动执行 wrangler secret put GITHUB_CLIENT_SECRET
- 前端 API 层使用 axios 拦截器，1001 错误码自动跳转登录页

### 影响范围
- 后端: wrangler.toml (新增 GITHUB_CLIENT_ID var), deploy.yml (新增 secret 设置步骤)
- 前端: 全部重建 — Login.vue, Home.vue, Dashboard.vue, AdminTools.vue, AdminUsers.vue, AdminAudit.vue, AdminConfigs.vue, SystemConfig.vue, MainLayout.vue, API层, i18n, router, Pinia auth store
- 配置: .gitignore (新增 frontend/node_modules/)

## v0.2.0 - 2026-05-20
### 变更内容
- 配置集(Config)移除 tool_id 字段，成为独立实体
- 新增系统配置功能，支持菜单布局切换(左侧边栏/顶部导航)
- 新增 GitHub OAuth 登录，替换原账号密码登录
- 登录页改为苹果风格，仅 GitHub 授权按钮
- 管理员邮箱白名单：mysinsy@163.com, scx@polofox.com, fntp66@gmail.com
- 后台布局支持 left(左侧边栏)/top(顶部导航) 动态切换
- 新增 GitHub Actions 自动部署(push main 触发 wrangler deploy)
- 默认域名修改为 apps.sential.eu.cc
- wrangler.toml 添加 APP_URL 环境变量和自定义域名路由

### 影响范围
- 后端: lib.rs (Config结构体、路由、GitHub OAuth回调)
- 前端: Login.vue, AdminConfigs.vue, MainLayout.vue, SystemConfig.vue(新增), api/index.ts, main.ts, i18n, Home.vue
- 配置: Cargo.toml(新增reqwest), wrangler.toml(域名+env), .github/workflows/deploy.yml(新增)
