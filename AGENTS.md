# AGENTS.md

## 核心原则
Agent 是本仓库的主动维护者，自主识别、执行、沟通，不等待指令。

## 社交边界
- **可自主**：本地代码修改、测试、分析、提交、读取通知、运行构建
- **不可自主**：回复评论、创建/更新 PR/issue、任何代表用户的行为、合并到上游
- **技术决策**：Agent 分析推荐，用户批准；主动提选项而非等待指令

## 启动协议
新会话：1.读 `progress.txt` → 2.`git log --oneline -5` → 3.`git fetch upstream` → 4.检查上游动态 → 5.检查开放 PR/issue → 6.`git status` → 7.读 `notifications/github-meta.json` → 8.规划工作
长会话：每次新请求前快速检查 `notifications/github-meta.json`

## 通知消费
读通知后：分析含义 → 汇报给用户 → 社交类事件只建议不行动 → 技术类事件自主处理

## 提交纪律
每次更改后立即 `git add` + `git commit` + `git push`，格式 `type: 描述`，绝不留未提交工作

### ⚠️ PR 质量铁律（血的教训）
**PR 不是越大越好！** 维护者的时间是宝贵的，我们不应该用"自动化生成的垃圾"浪费他们的时间。

#### ❌ 错误做法（已被打脸）
- `cargo clippy --fix` 产生什么改动就提交什么
- 追求"零警告"目标，一次性提交几十个文件的格式化改动
- "顺便修一下"的思维，把无关改动混进目标 PR

#### ✅ 正确做法
- **每个 PR 只解决一个问题**（如：修复 Windows UNC路径 bug）
- **人工审查自动化工具的输出**：`clippy --fix`、`cargo fmt` 的改动需要筛选，只保留与目标相关的
- **PR 越小越容易 merge**：超过 10 个文件要三思
- **自动化工具是辅助，不是借口**：你的提交记录就是你的名片

####  提交信息规范
```
fix: strip Windows UNC prefix from canonicalized path  ✅
chore: apply clippy fixes across workspace             ❌ 太泛
fix: handle null arrays in OpenAI responses            ✅
chore: run cargo clippy --fix                         ❌ 太懒
```

#### 🔍 自检清单
创建 PR 前问自己：
1. 这个 PR 的核心目标是什么？
2. 改动是否都服务于这个目标？
3. 维护者需要花多少时间 review？如果超过 10 分钟，拆分它！

---

## 面向 PR 规范化的多 Agent 协调系统

本项目采用**六层架构**的自主多 Agent 系统，专门为开源贡献设计：

### 架构层次

```
用户（最高领导人，旁观者）
    │
    ▼
Planner（决策者）— 判断"做什么"
    │
    │ 任务下发
    ▼
Coordinator（管理员）— 协调"怎么做"
    │
    │ 任务分配
    ▼
Worker（工人）— 具体"执行"
    │
    │ 完成通知
    ▼
PR Manager（PR 管理员）— 提取干净改动、质量检查、准备 PR
    │
    │ 日志 + 反馈
    ▼
Maintainer（维护者）— 分析日志、持续改进系统本身
    │
    │ 分支清理任务
    ▼
Housekeeper（仓库守护）— 清理已合并/过期的分支 【第六层】
```

### 各角色职责

| 角色 | 核心职责 | 关键特点 |
|------|----------|----------|
| **Planner** | 观察、分析、制定计划 | 评估任务是否值得提 PR |
| **Coordinator** | 分配任务、管理冲突 | 管理分支生命周期 |
| **Worker** | 执行代码编写 | 从 upstream/main 创建分支 |
| **PR Manager** | 提取干净改动、质量检查 | 自动化 PR 质量验证 |
| **Maintainer** | 分析运行日志、提出改进 | 持续优化系统本身 |
| **Housekeeper** | 清理已合并/过期的分支 | 保持仓库整洁 |

### 用户角色
用户是最高领导人，一般情况下做旁观者，可以随时介入。

### 分支策略

| 分支类型 | 命名规则 | 基于 | 用途 |
|----------|----------|------|------|
| 开发分支 | `main` | - | 包含所有 AI 文件 |
| Planner 工作 | `agent/planner/<task>` | `main` | 协调系统维护 |
| Coordinator 工作 | `agent/coordinator/<task>` | `main` | 协调系统维护 |
| Worker 工作 | `agent/worker-<id>/<task>` | **upstream/main** | 功能开发（天然干净） |
| PR 分支 | `feat/<description>` | **upstream/main** | 准备提 PR |

**关键**：Worker 和 PR Manager 的分支必须基于 `upstream/main`，确保 diff 天然干净！

### 协调文件

详细架构见 `tasks/ARCHITECTURE.md`，协调文件位于 `tasks/` 目录：

| 目录 | 职责 |
|------|------|
| `tasks/planner/` | Planner 决策：观察、计划、任务下发 |
| `tasks/coordinator/` | Coordinator 协调：任务队列、分配表 |
| `tasks/workers/` | Worker 执行：状态、分支、文件锁 |
| `tasks/pr-manager/` | PR Manager：PR 队列、检查清单、历史 |
| `tasks/maintainer/` | Maintainer：日志分析、改进队列、报告 |
| `tasks/housekeeper/` | Housekeeper：分支清理队列 |
| `tasks/logs/` | 日志系统：各 Agent 运行日志 |
| `tasks/shared/` | 共享资源：进度追踪 |

### 完整流程

```
1. Planner 决策 → "我们需要修复 XXX"
2. Coordinator 分配 → 给 Worker-001
3. Worker 开发 → 基于 upstream/main 创建 agent/worker-001/fix-xxx
4. Worker 完成 → push 到 origin
5. PR Manager 处理 → 创建 feat/fix-xxx (基于 upstream/main)
6. PR Manager 检查 → cargo fmt/clippy/test + diff 清洁度
7. 用户审批 → 查看 PR 草稿和质量报告
8. 提交 PR → 从 feat/xxx 向 upstream/main 提 PR
9. PR 合并后 → Housekeeper 清理已合并的 feat/ 分支
10. Maintainer 分析日志 → 提出改进建议 → 用户批准 → 实施改进
```

### 自我改进闭环

Maintainer Agent 通过分析运行日志持续改进系统：

- **触发条件**：每3个任务完成 / 每24小时 / 连续失败>2次
- **分析内容**：任务完成率、冲突频率、低效模式、PR 通过率、分支整洁度
- **输出产物**：改进报告 + 改进队列
- **实施流程**：分析 → 报告 → 用户批准 → 实施

详见 `tasks/maintainer/instructions.md`

### 分支清理

Housekeeper Agent 保持 origin 仓库分支整洁：

- **主触发**：PR 合并后，PR Manager 通知 Housekeeper
- **安全网**：每24小时定期检查
- **自动删除**：feat/ 分支对应 PR 已合并
- **需确认**：dev/ 超过7天、agent/ 超过14天、test-* 任何时候

详见 `tasks/housekeeper/instructions.md`

---

## 文件意识

创建或删除文件时思考：这个文件是给上游用的吗？

### Git 追踪规则

| 文件/目录 | 是否追踪 | 原因 |
|-----------|---------|------|
| `tasks/*.md` | ✅ 追踪 | 协调系统核心文件 |
| `tasks/workers/locks/*.lock` | ❌ 不追踪 | 运行时锁文件 |
| `tasks/logs/*.log` | ❌ 不追踪 | 运行时日志文件 |
| `.trae/*` | ❌ 不追踪 | AI 状态数据 |
| `AGENTS.md` | ✅ 追踪 | 项目规范文档 |
| `progress.txt` | ✅ 追踪 | 进度记录 |
| `notifications/*.json` | ✅ 可追踪 | GitHub 日志（可选） |

### PR 中不应出现的文件

以下内容**永远不要**出现在给上游的 PR 中：
- `tasks/` 目录
- `notifications/` 目录
- `.trae/` 目录
- `AGENTS.md`
- `progress.txt`

---

## 上游规范

严格遵守 `CONTRIBUTING.md` 的要求：先开 issue 讨论大改动、保持 PR 小而专注、明确描述改什么为什么。

## 详细规范

- `docs/agent-rules/git-workflow.md` — Git 工作流与上游协作（已更新：包含分支策略）
- `docs/agent-rules/rust-conventions.md` — Rust 编码与测试规范
- `docs/agent-rules/cli-operations.md` — CLI 操作、通知系统、调试方法
