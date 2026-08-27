# Web 认证安全方案设计

## 目标与威胁边界

JWT 只负责证明令牌由可信服务签发，并携带用户、角色和有效期等声明。它本身不能阻止浏览器自动携带 cookie 造成的 CSRF，不能阻止页面脚本读取可访问的令牌，也不能表达一条已签发令牌是否已经被服务端撤销，更不能替代后端权限判断。

本次实现覆盖以下边界：

- XSS 后不让脚本直接读取访问令牌，并限制脚本来源；动态 HTML 在服务端净化，Vue 页面继续使用自动转义插值，不引入 `v-html`。
- cookie 认证的非安全方法必须同时通过 Origin 白名单和双提交 CSRF token 校验。
- 权限由后端依据 JWT 与会话中一致的角色强制执行；前端隐藏按钮只改善体验，不作为安全边界。
- 每个 JWT 的 `jti` 在 SQLite 会话表中留痕；登出立即标记撤销，之后即使 JWT 签名和 `exp` 仍有效也会被拒绝。
- 请求日志不输出 `Authorization`、`Cookie`、`Set-Cookie` 或密码。

## 方案对比

| 方案 | 优点 | 缺点 | 结论 |
| --- | --- | --- | --- |
| JWT 存 `localStorage`，每次发 Bearer | 实现简单，天然不受 cookie CSRF 影响 | 任意同源 XSS 都能读取并带走令牌，在过期前可异地重放；撤销仍需服务端状态 | 不采用 |
| 纯服务端随机 Session cookie | 撤销和续期直观，cookie 内容无身份信息 | 每次请求必须查存储；与现有 JWT/API 客户端兼容性较弱 | 可行，但不是本项目渐进改造的最优选择 |
| HttpOnly JWT cookie + CSRF + 服务端 `jti` 会话表 | 浏览器脚本读不到 JWT；支持立即撤销、RBAC 和 Bearer API；能沿用现有 JWT | 不再是完全无状态；cookie 请求需要 CSRF 防护 | **采用** |
| JWT + Redis 会话 | 高吞吐、TTL 自然 | 运行时依赖 Redis 服务 | 已支持，通过 `session.backend = "redis"` 启用 |

## 最终设计

### 登录与令牌

登录成功后签发 HS256 access token。claims 必须包含 `sub`、`role`、`iat`、`exp`、`jti`、`iss`、`aud`；校验固定算法，并同时校验 issuer、audience 和过期时间，避免令牌跨系统误用。

JWT 只写入 `access_token` cookie：`HttpOnly; SameSite=Lax; Path=/`，生产配置启用 `Secure`。响应体不再返回 JWT。另写入可由前端读取的 `csrf_token` cookie；数据库只保存它的 SHA-256 摘要。

### CSRF

使用 cookie 身份认证且方法为 POST/PUT/PATCH/DELETE 时，中间件要求：

1. 若存在 `Origin`，它必须精确命中 `security.allowed_origins`；
2. `csrf_token` cookie 必须与 `X-CSRF-Token` 请求头相同；
3. 该值的摘要必须与当前 `jti` 在 SQLite 中保存的摘要一致。

Bearer 客户端的凭据不会被浏览器自动发送，因此不要求 CSRF token。CORS 只允许明确来源、明确方法和明确请求头，并允许 cookie。

### 撤销与会话一致性

会话存储通过 `session.backend` 在 `database` 和 `redis` 之间选择。数据库模式由 `auth_sessions` 保存 `jti`、用户名、角色、CSRF 摘要、签发/过期/撤销时间；Redis 模式将相同结构序列化到带前缀的键，并按 JWT 剩余有效期设置 TTL。每次认证在验证 JWT 后查询所选存储，并确认用户名、角色和过期时间与 JWT 一致。登出在数据库设置 `revoked_at`，或直接删除 Redis 键，同时清除两个 cookie。

这让“签名正确”与“当前仍获授权”成为两步独立判断。代价是每次请求都要访问一次所选会话存储；数据库模式适合本地独立运行，Redis 模式则利用原生 TTL 管理过期会话。

### RBAC 与 XSS

当前角色为 `admin` 和 `viewer`。查询/报表允许两者，`insert`、`update`、`delete`、上传和同步类路径只允许 `admin`。判定发生在后端认证中间件中。

XSS 采用多层约束：令牌 HttpOnly、CSP `script-src 'self'`、禁止 framing、禁止 MIME sniffing、收紧 referrer/permissions policy；Rust 返回动态 HTML 时继续用 `ammonia` 净化，Vue 只用文本插值。需要强调：HttpOnly 不能让正在运行的恶意同源脚本失去“代用户发请求”的能力，所以 CSP、依赖审计和输出转义仍然必须保留。

## 部署要求

- 生产环境必须通过 `APP_JWT__SECRET` 覆盖默认 JWT secret，通过 `APP_AUTH__PASSWORD` 覆盖开发密码，并将 `APP_SECURITY__COOKIE_SECURE=true`。环境变量中的双下划线表示配置层级。
- `security.allowed_origins` 只填写实际前端源（协议、主机、端口完整匹配），不要使用 `*`。
- 服务必须通过 HTTPS 暴露；SQLite 文件需要限制系统权限并纳入备份策略。
- 角色变化或账号禁用时，应撤销该用户的现有 `auth_sessions`；目前登出实现单会话撤销。
