# ValveOS 用户指南

> 让多个 AI Agent 自动协作完成项目任务

> **本文档是写给人类（你）看的。** Agent 的行为规范在其他文件中。

---

## 一句话原理

**你是阀门，Agent 是水流。**

你只需要决定"唤醒谁"，Agent 自己搞定剩下的一切。

Agent 之间通过共享文件（inbox）通信，**不经过你**。你只负责打开正确的门。

---

## 架构总览

```
核心流水线（线性，多会话模式）：
  Planner → Coordinator → Worker → PR Manager

横切服务（独立触发，覆盖所有层）：
  Maintainer — 数据分析后台
  Housekeeper — 仓库清理后台
  COO — 系统维护后台（支持单会话模式）
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

---

## 核心概念

| 概念 | 含义 | 类比 |
|------|------|------|
| 沉睡 | Agent 收到消息但未被人唤醒，无法执行 | 信箱里有信，但人还没起床 |
| 唤醒 | 用户打开特定 Agent 的会话 | 你推门把人叫醒 |
| 睁眼 | 被唤醒的 Agent 主动读取自己的 inbox 消息 | 醒来后第一件事看信箱 |
| 声音 | Agent 写入共享文件的消息 | 写好信放进别人的信箱 |
| 待机 | Agent 被唤醒但未收到消息，轮询等待中 | 醒了但没信，每隔5分钟看一眼信箱 |

---

## 典型工作流程：从想法到 PR

### 第1步：唤醒 Planner

打开一个新会话，告诉 Planner 你想做什么：

```
"我想给项目添加 XX 功能"
```

Planner 会分析项目现状，制定任务计划，然后告诉你：

```
请唤醒 Coordinator — 任务计划已写入其 inbox
```

### 第2步：唤醒 Coordinator

按 Planner 的提示，打开新会话唤醒 Coordinator。

Coordinator 读取 Planner 的计划，拆分任务后告诉你：

```
请唤醒 Worker-001 和 Worker-002 — 任务已分配
```

### 第3步：唤醒 Worker(s)

Coordinator 会告诉你需要几个 Worker。每个 Worker 独立工作：

- 用 `git worktree add` 创建独立工作目录（不影响主仓库）
- 编写代码、运行测试、提交并 push
- 完成后通知你：**"请唤醒 PR Manager"**

### 第4步：唤醒 PR Manager

PR Manager 检查 Worker 的改动质量：
- 提取干净的功能 commit 到 `feat/` 分支
- 运行格式化、lint、测试检查
- 准备好 PR 描述，等你审批

### 第5步：你审批 PR

查看 PR 内容，确认无误后提交到上游。

> **注意**：Maintainer / Housekeeper / COO 是横切服务，独立运行，你不需要手动管理它们。

---

## 单会话模式（系统维护专用）

### 什么时候用

- 修改系统文档（如本文件）
- 执行审计修复
- 改进 skill 触发规则

### 怎么用

只需唤醒 COO，告诉他要做什么，COO 会在一个会话内用子代理并行完成所有工作。

### 什么时候不用

**需要向上游提 PR 的代码修改**——必须走核心流水线（Planner → ... → PR Manager）。

### 自动闭环

每次系统修改后，COO 自动执行：

```
审计 → 修复 → 评估 skill → 改进 skill
```

---

## 待机模式（节省你的时间）

待机模式让 Agent 在被唤醒后自动轮询 inbox，而不是干等着你再次手动唤醒。

### Inbox 待机（等特定 Agent 的消息）

适用于 Coordinator 等 Planner 下发任务的场景：

1. 唤醒 Coordinator，说：**"待机模式，等 Planner 消息"**
2. Coordinator 每 5 分钟检查一次自己的 inbox
3. 收到消息后自动开始工作

### 分配表待机（批量 Worker 待机）

适用于你要去休息、让多个 Worker 同时待命的场景：

1. 唤醒多个 Worker，说：**"待机模式，等分配表"**
2. 每个 Worker 轮询 `tasks/coordinator/assignments.md` 中的就绪标记
3. 有任务就认领执行

### 级联技巧

不需要一口气开所有终端。每步只开紧邻下游：

```
Planner(工作) + Coordinator(待机)     ← 2个终端
→ Planner 完成，Coordinator 自动开始
→ 再开 1 个 Worker(待机)             ← 仍占2-3个终端
→ Coordinator 完成，Worker 自动开始
```

Agent 完成后终端自动空出，同时最多占用 2-3 个终端。

### 安全规则

- 终端数量取决于 Trae IDE 能同时开多少个聊天窗口
- **不要用 while 循环**——用单次 `Start-Sleep -Seconds 300`，让 Agent 自主决定是否重新调用
- 如果 sleep 被 Trae 超时杀掉，重新唤醒 Agent 即可恢复（天然幂等）

---

## Git 安全规则（用户需知）

这些规则保护你的仓库不被多 Agent 并发操作搞坏：

| 规则 | 说明 |
|------|------|
| Worker 必须用 worktree | `git worktree add ../claw-code-rust-w001 -b agent/worker-001/<task> upstream/main`，绝不在主仓库 checkout -b |
| 主仓库永远在 main | 所有 Agent 不在主仓库切换分支 |
| 每次改完立即 push | `git add` + `git commit` + `git push`，不留未提交工作 |
| push 前先 pull | `git pull --rebase origin main` 再 push |
| PR 越小越好 | 一个 PR 只解决一个问题，超过10个文件要三思 |
| commit 信息要具体 | `fix: strip Windows UNC prefix` ✅ / `chore: apply clippy fixes` ❌ |

遇到 git 报错？参见 [cli-operations.md](docs/agent-rules/cli-operations.md) 的 `.git 损坏应急协议`。

---

## FAQ

**Q: 为什么 Agent 不能直接互相说话？**
A: 它们可以往对方的 inbox 文件写消息，但只有你能"唤醒" Agent（打开会话）。这是 Trae 平台的限制——Agent 无法主动启动新会话。

**Q: 我需要做决策怎么办？**
A: Agent 会列出选项让你选。比如："方案A更快但风险稍高，方案B更稳但耗时较长，你选哪个？"

**Q: 可以跳过某些 Agent 吗？**
A: 可以！系统维护类的任务用**单会话模式**，直接唤醒 COO 就行，跳过整个流水线。但需要提 PR 的代码改动建议走完整流水线。

**Q: Git 坏了怎么办？**
A: 参见 `docs/agent-rules/cli-operations.md` 中的 `.git 损坏应急协议`。简单来说：停止写操作 → 诊断损坏程度 → 尝试修复 → 严重时告知用户手动处理。

**Q: 怎么知道下一步该唤醒谁？**
A: Agent 完成工作时**一定会**以这句话结尾：**"请唤醒 [Agent名] + 一句话原因"**。你照做就行。

**Q: 多个 Worker 能同时跑吗？**
A: 可以！每个 Worker 用独立的 git worktree 工作目录，互不干扰。主仓库永远保持在 main 分支不动。

**Q: 核心流水线和横切服务有什么区别？**
A: 核心流水线是**线性执行的**（Planner → Coordinator → Worker → PR Manager），一步步推进任务。横切服务是**独立后台运行的**（Maintainer / Housekeeper / COO），自己触发自己运行，覆盖所有层。

---

## 文件速查表

| 文件 | 谁看它 | 一句话说明 |
|------|--------|-----------|
| `AGENTS.md` | 所有 Agent + 你 | ValveOS 宪法，最高规范 |
| `tasks/ARCHITECTURE.md` | 所有 Agent + 你 | 完整架构文档，Agent 先读这个 |
| `tasks/multi-agent-user-guide.md` | **你** | 本文件，唯一面向人类的指南 |
| `tasks/planner/instructions.md` | Planner | Planner 行为规范 |
| `tasks/coordinator/instructions.md` | Coordinator | Coordinator 行为规范 |
| `tasks/workers/instructions.md` | Worker | Worker 行为规范 |
| `tasks/pr-manager/instructions.md` | PR Manager | PR Manager 行为规范 |
| `tasks/maintainer/instructions.md` | Maintainer | Maintainer 行为规范 |
| `tasks/housekeeper/instructions.md` | Housekeeper | Housekeeper 行为规范 |
| `tasks/coo/instructions.md` | COO | COO 行为规范 |
| `docs/agent-rules/git-workflow.md` | Worker + PR Manager | Git 工作流与上游协作规则 |
| `docs/agent-rules/rust-conventions.md` | Worker | Rust 编码与测试规范 |
| `docs/agent-rules/cli-operations.md` | 所有 Agent | CLI 操作参考（通知、调试、重置、待机） |
| `tasks/shared/inbox/*.md` | 各 Agent 对应 | 消息收件箱，Agent 间通信总线 |
| `tasks/shared/agent-status.md` | 所有 Agent | 全局状态与任务追踪面板 |
