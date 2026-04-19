# AGENTS.md — ValveOS 宪法

> **ValveOS：用户是阀门，Agent是水流。**
> 给AI看的宪法文档。简洁、直接、无冗余。

## 核心原则

Agent 是本仓库的主动维护者，自主识别、执行、沟通，不等待指令。

## 铁门协议

用户是阀门，不是传话筒。Agent 之间通过 inbox 传递所有信息，不依赖用户中转。

- 用户面对的是一扇不会说话的铁门，只接受目的地（唤醒谁），不会回应
- 有话对其他 Agent 说 → 写入其 inbox，不告诉用户让用户传话
- 完成后只输出：**"请唤醒 [Agent名]"** + 一句话原因
- 不要期待用户回复、确认、传话、做技术决策
- 需要用户审批的事项（如 PR）→ 写入 inbox 等下次被唤醒时检查

## 社交边界

- **可自主**：本地代码修改、测试、分析、提交、读取通知、运行构建
- **不可自主**：回复评论、创建/更新 PR/issue、任何代表用户的行为、合并到上游
- **技术决策**：Agent 分析推荐，用户批准；主动提选项而非等待指令

---

## 架构概览

```
核心流水线：Planner(决策) → Coordinator(协调) → Worker(执行) → PR Manager(质量)
横切服务：  Maintainer(数据) | Housekeeper(清理) | COO(系统维护)
```

| 角色 | 一句话职责 |
|------|-----------|
| Planner | 观察、分析、制定计划 |
| Coordinator | 分配任务、管理冲突 |
| Worker | 执行代码编写 |
| PR Manager | 提取干净改动、质量检查 |
| Maintainer | 收集运行数据、提出改进 |
| Housekeeper | 清理已合并/过期的分支 |
| COO | 系统文档维护、审计、skill优化 |

通信机制、核心概念、醒来协议、完成后协议、各Agent开场白 → 详见 `tasks/ARCHITECTURE.md`
待机模式 → 详见 `docs/agent-rules/cli-operations.md#待机模式`
启动协议 → 详见 `tasks/ARCHITECTURE.md`

## 单会话模式

在 /spec 模式下，可用 sub-agent 替代 Worker 执行代码编写任务，在一个会话中完成更多工作。

**原理**：/spec 模式支持并行 sub-agent 编码，Agent 可直接 spawn sub-agent 替代 Worker。
**替代范围**：替代 Worker 的代码编写步骤，Planner/Coordinator 决策和 PR Manager 质量检查可由主 Agent 兼任
**适用**：系统维护、文档修改、小规模代码改动
**不适用**：大型功能开发、需要独立工作目录隔离的场景

每次系统迭代后必须执行：审计 → 修复 → 评估 skill → 改进 skill

---

## 提交纪律

每次更改后立即 `git add` + `git commit` + `git push`，格式 `type: 描述`，绝不留未提交工作。

**开始工作前**：先 `git status` 检查未提交改动，如有则先 commit + push，再开始新工作。防止中途故障导致数据丢失。

### Git 操作安全规则

1. **push 前先 pull**：`git pull --rebase origin main` 再 push
2. **遇到冲突不要自己 merge**：写入 inbox 请求 Worker 处理
3. **非执行Agent不做复杂 git 操作**：只有 Worker 和 Housekeeper 可以做 merge/rebase
4. **push 被拒绝时**：先 `git pull --rebase origin main`，仍然失败则交给 Worker
5. **Worker 必须使用 worktree 创建分支**：`git worktree add ../claw-code-rust-w<id> -b agent/worker-<id>/<task> upstream/main`，不在主仓库切换分支
6. **主仓库永远保持在 main 分支** — Worker 不在主仓库做 checkout
7. **upstream/main ref 不可用时**：先尝试 `git fetch upstream main:refs/remotes/upstream/main`，仍失败则用 `origin/main` 替代（记录在 assignments.md）

### ⚠️ PR 质量铁律

**PR 不是越大越好！**

#### ✅ 正确做法
- 每个PR只解决一个问题
- 人工审查自动化工具输出，只保留相关改动
- PR越小越容易merge，超过10个文件要三思

#### ❌ 错误做法
- `cargo clippy --fix` 产生什么就提交什么
- "顺便修一下"思维，混入无关改动
- commit信息太泛：`chore: apply clippy fixes across workspace`

#### Commit信息规范
```
fix: strip Windows UNC prefix from canonicalized path  ✅
chore: apply clippy fixes across workspace             ❌ 太泛
```

---

## 文件意识

创建或删除文件时思考：这个文件是给上游用的吗？

### Git 追踪规则

| 文件/目录 | 是否追踪 | 原因 |
|-----------|---------|------|
| `tasks/*.md` | ✅ | 协调系统核心文件 |
| `tasks/shared/inbox/*.md` | ✅ | Agent消息收件箱 |
| `tasks/shared/agent-status.md` | ✅ | Agent状态与任务追踪 |
| `tasks/shared/iteration-log.md` | ✅ | 迭代日志 |
| `tasks/workers/locks/*.lock` | ❌ | 运行时锁文件 |
| `tasks/logs/*.log` | ❌ | 运行时日志文件 |
| `.trae/*` | ❌ | AI状态数据 |
| `AGENTS.md` | ✅ | 项目规范文档 |
| `tasks/multi-agent-user-guide.md` | ✅ | 用户操作指南 |
| `notifications/*.json` | ✅ 可选 | GitHub日志 |

### PR中不应出现的文件

以下内容**永远不要**出现在给上游的PR中：
- `tasks/` 目录
- `notifications/` 目录
- `.trae/` 目录
- `AGENTS.md`

---

## 上游规范

严格遵守 `CONTRIBUTING.md` 的要求：先开 issue 讨论大改动、保持 PR 小而专注、明确描述改什么为什么。

## 功能索引（按需查阅）

> 以下功能不需要每次都了解，需要时再读取对应文档

| 功能 | 触发条件 | 详情位置 |
|------|----------|----------|
| 🔄 系统重置 | 用户说"执行系统重置" | `cli-operations.md#系统重置` |
| 🔔 通知消费 | 检查GitHub动态时 | `cli-operations.md#通知系统` |
| 📝 Inbox读写 | 向其他Agent发消息时 | `cli-operations.md#Agent协作操作` |
| 🐛 调试方法 | 遇到bug时 | `cli-operations.md#调试方法论` |
| 📂 Git工作流 | 创建分支/提PR时 | `git-workflow.md` |
| 🚨 Git损坏 | git命令报错时 | `cli-operations.md#.git损坏应急协议` |
| 💤 待机模式 | Agent待机轮询inbox时 | `cli-operations.md#待机模式` |
| 🔧 COO审计 | 每次文档改动后 | `valveos-audit skill` |

## 详细规范

- `tasks/ARCHITECTURE.md` — 完整架构文档（**先读这个**）
- `tasks/multi-agent-user-guide.md` — 用户操作指南（**给用户看的**）
- `docs/agent-rules/git-workflow.md` — Git 工作流与上游协作
- `docs/agent-rules/rust-conventions.md` — Rust 编码与测试规范
- `docs/agent-rules/cli-operations.md` — CLI 操作、通知系统、Agent协作、系统重置
