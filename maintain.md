# Apps Startpage - 变更记录

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
