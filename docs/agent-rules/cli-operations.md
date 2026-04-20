# CLI 操作、通知系统与调试

## PowerShell 注意事项
- `&&` 不可用，用 `;` 连接命令
- `curl` 是 PowerShell 别名，需要用 `curl.exe`
- 文件读写只在项目目录内进行，不用系统 `%TEMP%`
- `Out-File` 写系统临时目录会被安全工具拦截，避免使用

## Git 操作
- 优先用 Git MCP 工具（add、commit、status、diff、log、branch）
- push/pull 等 MCP 不支持的才用命令行
- 有未提交更改时先 `git stash` 再 pull

## 通知系统
- 通知文件：`notifications/github-meta.json`（元数据）+ `github-activity.jsonl`（事件日志）
- Actions 每 30 分钟采集：上游 commits、PR 活动、issue 更新、评论
- Agent 消费行为：分析含义 → 汇报用户 → 社交类只建议不行动 → 技术类自主处理
- 读取后更新 `last_read_timestamp`

## 调试方法论
- 遇到 bug 先复现，再定位，最后修复
- GitHub Actions 调试：将日志写入仓库文件（会被提交推送），运行后读取
- API 调试：先在本地用 `curl.exe` 或 `mcp_fetch_fetch` 验证端点可用
- 变量展开问题：shell heredoc 不可靠，用 `jq -n` 构建 JSON
- 权限问题：`GITHUB_TOKEN` 只能访问当前仓库，上游公开仓库用 `curl` 无认证 API

## Fork 维护意识

这是 fork 仓库，提 PR 时需注意：Agent 专用文件不应出现在给上游的 PR diff 中。创建文件时先思考：这个文件是给上游用的吗？

---

## Agent 协作操作

### 读写 Inbox（消息收件箱）

**位置**：`tasks/shared/inbox/[角色].md`（planner / coordinator / worker / pr-manager / maintainer / housekeeper / coo）

**读取**：用 Read 工具读取自己的 inbox，检查"待处理消息"区
**写入**：用 SearchReplace 向目标 Agent 的 inbox 添加消息行：
```markdown
| 时间 | 来源 | 内容摘要 | 状态 |
|------|------|----------|------|
| YYYY-MM-DDTHH:MM:SSZ | 你的Agent名 | [消息内容] | 未读 |
```
**处理后**：将消息从"待处理"移到"已处理"区

### 更新 Agent Status

**位置**：`tasks/shared/agent-status.md`

更新自己的状态和等待唤醒的 Agent：
```markdown
| Agent | 最近活跃 | 当前状态 | 等待唤醒 |
|-------|----------|----------|----------|
| 你的Agent名 | 当前时间 | 沉睡 | - |
| 下一个Agent | - | 未启动 | 你的Agent名 |
```

### 日志记录

**位置**：`tasks/logs/[角色].log`

**格式**：
```
[YYYY-MM-DD HH:MM:SS] [角色] [级别] MESSAGE
  - detail: ...
  - data: { ... }
```

**级别**：INFO / WARN / ERROR / DECISION

**ValveOS 特有事件**（必须按需记录）：
| 事件 | 触发场景 | 使用Agent |
|------|----------|-----------|
| WAKEUP | 被用户唤醒，执行醒来协议 | 所有Agent |
| RESUME | 断点续传，发现上次进度 | 主要Planner |
| MESSAGE | Inbox消息读写 | 所有Agent |
| RESET | 系统重置（完全/选择性） | 主要Housekeeper |
| ITERATION | 迭代生命周期变更 | 主要Planner |
| LOOKUP | 查阅功能索引/文档 | 主要Maintainer |

详细格式示例见 `tasks/logs/README.md` 和各 Agent 的 instructions.md

### 完成后的标准输出

每个 Agent 完成工作后必须输出：

```markdown
请唤醒 [下一个Agent名称]。
```

所有上下文、策略、细节必须已写入目标 Agent 的 inbox。用户是阀门，不传话。

---

## 待机模式

Agent 可以在用户唤醒后进入**待机轮询**，主动监听 inbox 而非等待手动唤醒。

### 待机模式类型总览

| 类型 | 使用者 | 等待内容 | Inbox 文件 | 用户触发命令 |
|------|--------|----------|------------|-------------|
| **Coordinator Inbox 待机** | Coordinator | Planner 完成消息 | `inbox/coordinator.md` | "待机模式，等 Planner 消息" |
| **Worker Dispatch 待机** | Worker | Coordinator 分配的任务 | `coordinator/assignments.md` | "待机模式，等任务分配" |
| **PR Manager Inbox 待机** | PR Manager | Worker 完成消息 | `inbox/pr-manager.md` | "待机模式，等 Worker 消息" |

### 触发

用户唤醒 Agent 时附加指令："待机模式，等 [来源Agent] 消息"

### 级联待机（推荐用法）

不需要一口气开所有 Agent。每步只开紧邻下游：

```
1. 开 Planner(工作) + Coordinator(待机)    ← 2个终端
   Planner 完成 → Coordinator 自动开始
2. 开 Worker(待机)                          ← 3个终端
   Coordinator 完成 → Worker 自动开始
3. 开 PR Manager(待机)                      ← 3个终端
   Worker 完成 → PR Manager 自动开始
```

Agent 完成工作后终端自动空出，所以同时最多占 2-3 个终端。

### 待机建议

Agent 完成后，在输出中附带待机建议，帮助用户决定下一步开谁待机：

```
请唤醒 Worker-001（待机）。预计需要 1 个 Worker。
```

### 工作流

```
1. 更新 agent-status → "待机(等XXX消息)"
2. 执行: Start-Sleep -Seconds 300
3. 醒来后检查: Select-String -Path $inboxPath -Pattern '未读' -Quiet
4. 有消息 → 标记已读 → 开始工作
5. 无消息 → 回到步骤 2
```

### 待机命令

```powershell
Start-Sleep -Seconds 300
```

### 检查命令

```powershell
$inboxPath = "项目路径/tasks/shared/inbox/[自己的角色].md"
Select-String -Path $inboxPath -Pattern '未读' -Quiet
```

返回 `True` = 有未读消息，返回 `False` = 无消息。

### 超时恢复

如果 Trae 超时杀掉了 sleep 命令：
- 用户重新唤醒 Agent
- Agent 读取 inbox → 有消息就工作，没消息就继续待机
- 天然幂等，无需特殊恢复逻辑

### ⚠️ 不要用 while 循环

```powershell
# ❌ 错误 — 被杀后恢复困难，上下文浪费
while ($true) { ... Start-Sleep ... }

# ✅ 正确 — 单次 sleep，Agent 自主决定是否重调用
Start-Sleep -Seconds 300
```

原因：
1. while 循环被超时杀掉后，Agent 会话可能异常
2. 循环日志持续消耗上下文窗口
3. Agent 在循环期间无 AI 控制权，无法做决策

### 安全规则

1. **聊天会话数量** — 取决于 Trae IDE 能同时开多少个聊天窗口，每个待机 Agent 占一个
2. **只待机紧邻下游** — 不需要全部待机
3. **待机前更新 agent-status** — 状态改为"待机(等XXX消息)"
4. **检测到消息后立即标记已读** — 防止重复触发

---

## 系统重置

当用户想要从头开始时，告诉任意Agent **"执行系统重置"**。

> ⚠️ **不可跳步**：以下步骤必须按顺序逐个执行，每完成一步标记 `[x]`。跳步会导致状态不一致（如 iteration-log 与 agent-status 迭代号不匹配）。

### 重置操作

Agent会执行以下操作：

- [ ] **清空所有 inbox**（`tasks/shared/inbox/*.md`）→ 恢复为空模板
- [ ] **重置 agent-status.md** → 所有Agent回到"未启动"
- [ ] **归档当前 iteration-log 条目** → 标记为"已废弃"，新建递增迭代号条目（与 agent-status.md 迭代号一致）
- [ ] **新建空白迭代条目**
- [ ] **处理运行数据文件**：
   - **归档保留**（添加重置分隔线，不清空）：
     - `tasks/coo/audit-log.md`
     - `tasks/shared/session-reports/*.md`（所有 7 个 Agent 文件）
   - **清空恢复模板**：
     - `tasks/planner/observations.md`
     - `tasks/coordinator/queue.md` + `assignments.md`
     - `tasks/workers/status.md` + `branches.md`（清空分支记录）
     - `tasks/pr-manager/pr-queue.md`
     - `tasks/maintainer/improvements.md`（改进状态改为 proposed 或删除已完成项）
     - `tasks/housekeeper/cleanup-queue.md`（保留清理历史）
- [ ] **不触碰制度文件**：instructions.md（所有 Agent）、ARCHITECTURE.md、AGENTS.md、SYSTEM-MANIFEST.md、decisions.md、SKILL.md 等跨迭代制度文件保持不变
- [ ] 输出："✅ 系统已重置，可以重新唤醒 Planner 开始新迭代"

### 完成后校验（必须执行）

重置完成后，逐项校验以下内容，确保无遗漏：

- [ ] iteration-log.md 当前迭代号 = agent-status.md 当前迭代号
- [ ] 所有 inbox 已清空（7 个文件）
- [ ] observations.md 已恢复模板
- [ ] queue.md + assignments.md 已恢复模板
- [ ] audit-log.md 有重置分隔线且历史保留
- [ ] session-reports/*.md 有重置分隔线且历史保留
- [ ] 制度文件（instructions.md/ARCHITECTURE.md/AGENTS.md/SYSTEM-MANIFEST.md/decisions.md）未被修改
- [ ] git commit 成功

如有任何项未通过，立即补执行对应步骤。

### 选择性重置

| 命令 | 操作 |
|------|------|
| "执行系统重置" | 完全重置（默认） |
| "只重置任务看板" | 只重置 agent-status 的任务区 |
| "只归档当前迭代" | 只标记当前迭代为已废弃 |
| "只清空inbox" | 只清空所有收件箱 |

### 安全规则

1. **永远不删除Git历史** — 重置只是恢复文件内容到模板状态
2. **保留cleanup-queue的清理历史**
3. **保留logs/目录的日志文件**
4. **重置前必须告知用户将要做什么**
5. **保留audit-log的制度记忆** — 审计日志是跨迭代的制度记忆，重置时只添加分隔线标注，不清空
6. **不修改制度文件** — instructions.md、ARCHITECTURE.md、AGENTS.md、SYSTEM-MANIFEST.md、decisions.md 等是跨迭代的制度记忆，重置时不应修改

---

## .git 损坏应急协议

### 预防措施：使用 Worktree 隔离

多个 Worker 同时操作同一个 git 仓库会导致 .git 损坏。预防方法：

- Worker 使用 `git worktree add` 创建独立工作目录
- 主仓库永远保持在 main 分支
- 详见 `git-workflow.md#Worker Worktree 工作流`

当 `git` 命令报错（如 `fatal: not a git repository`、`corrupt`、`index.lock` 等）时：

### 修复步骤

1. **停止所有写操作** — 不要继续执行任务
2. **诊断损坏程度**：
   - `git status` — 是否能读？
   - `git log --oneline -1` — 历史是否完整？
   - `git fsck` — 检查损坏详情
3. **尝试修复**：
   - `index.lock` 残留 → `rm .git/index.lock`
   - 轻微损坏 → `git fsck --full` 按提示修复
   - HEAD 损坏 → `git reset --hard HEAD` 或 `git reset --hard origin/main`
4. **严重损坏**（无法修复）→ 告知用户："⚠️ .git 严重损坏，建议用户手动处理"
5. **修复后验证** → `git status` + `git log --oneline -3` 确认正常

### 安全规则

- **不要尝试 `git push --force`** — 可能覆盖远程数据
- **不要删除 .git 目录** — 除非用户明确指示
- **修复后立即 commit + push** — 确保当前工作不再次丢失
