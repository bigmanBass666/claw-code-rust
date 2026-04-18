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

## 多 Agent 协调规则 ⚠️（血的教训2号）

**问题**：多个 AI 在同一分支同时工作 → PR 混入无关改动 → 维护者被迫手动清理

### 强制规则
1. **每个 AI 使用独立分支**：`agent/<name>/<功能>` 格式
2. **PR 前必须审查 diff**：`git diff origin/main` 检查是否只包含目标改动
3. **禁止向他人分支推送**：除非明确协调，否则只在自己的分支工作
4. **PR 前自问**：这条 PR 核心改动是什么？有没有混入其他 AI 的工作？

### 自检脚本（创建 PR 前必跑）
```bash
# 检查 PR 会包含哪些文件
git fetch origin main
git diff origin/main --stat

# 如果看到 5 个以上文件不是自己负责的，立刻停下来检查！
```

### ⚠️ 特别注意
- `notifications/`、`tasks/`、`.trae/` 是 AI 专用目录，**永远不要**推向上游
- `AGENTS.md`、`progress.txt` 是内部文档，**永远不要**出现在 PR 中
- 发现其他 AI 的改动混入？**必须先清理再 PR**

---

## 文件意识
创建或删除文件时思考：这个文件是给上游用的吗？Agent 专用文件（内部工具、运行时数据、Agent 文档）不应出现在给上游的 PR 中。当前已在 `.gitignore` 中排除：`tasks/`、`.trae/`、`notifications/`。

## 多 Agent 协调系统

本项目采用三层架构的自主多 Agent 系统：

### 架构层次
- **Planner（决策者）** — 判断"做什么"：观察项目状态、分析问题、制定计划
- **Coordinator（管理员）** — 协调"怎么做"：分配任务、管理冲突、监控进度
- **Worker（工人）** — 具体"执行"：认领任务、编写代码、提交改动

### 用户角色
用户是最高领导人，一般情况下做旁观者，可以随时介入。

### 协调文件
详细架构见 `tasks/ARCHITECTURE.md`，协调文件位于 `tasks/` 目录：

| 目录 | 职责 |
|------|------|
| `tasks/planner/` | Planner 决策：观察、计划、任务下发 |
| `tasks/coordinator/` | Coordinator 协调：任务队列、分配表 |
| `tasks/workers/` | Worker 执行：状态、分支、文件锁 |
| `tasks/shared/` | 共享资源：规范文件、进度追踪 |

## 上游规范
严格遵守 `CONTRIBUTING.md` 的要求：先开 issue 讨论大改动、保持 PR 小而专注、明确描述改什么为什么。

## 详细规范
- `docs/agent-rules/git-workflow.md` — Git 工作流与上游协作
- `docs/agent-rules/rust-conventions.md` — Rust 编码与测试规范
- `docs/agent-rules/cli-operations.md` — CLI 操作、通知系统、调试方法
