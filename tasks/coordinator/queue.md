# 任务队列

此文件由 **Planner** 下发任务，由 **Coordinator** 消费。

> **双通道说明**：Coordinator 主要通过 **inbox** (`tasks/shared/inbox/coordinator.md`) 接收 Planner 的任务分配和指令。本 queue.md 作为**结构化任务看板**，提供更详细的任务元数据（优先级、依赖、截止时间）。两者互补，不冲突。

## 任务状态

- `pending` — 待处理
- `in_progress` — 进行中
- `completed` — 已完成
- `blocked` — 被阻塞

## 优先级

- `P0` — 紧急，影响核心功能
- `P1` — 重要，待审核的 PR、关键改进
- `P2` — 一般，代码优化、文档
- `P3` — 低，长期改进、探索

---

## 待处理任务

### TASK-013: 同步本地 main 到 upstream/main
- **优先级**: P0
- **描述**: 将本地 main 分支 rebase 到 `upstream/main`，同步上游最新代码（包括 UNC path 修复、null array fix、log-level fix 等），同时保留 ValveOS 文件
- **期望结果**: 本地 main 包含上游最新代码 + ValveOS 文件，编译通过
- **值得提 PR**: 否
- **依赖**: 无
- **注意**: rebase 可能产生冲突（ValveOS 文件与上游不冲突，但 Cargo.toml 等可能冲突）。Worker 必须使用 worktree 操作

---

## 进行中任务

<!-- 正在执行的任务 -->

---

## 已完成任务

<!-- 已完成的任务 -->
