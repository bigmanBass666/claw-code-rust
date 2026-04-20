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

### TASK-ITER10-002: 同步 upstream/main → origin/main
- **描述**: upstream/main (82e2d40) 领先 origin/main (cbae86b) 8个提交，包括 PR #40 null数组修复、PR #42 home_dir.rs 修复等。需要同步。
- **状态**: pending
- **优先级**: P0
- **负责人**: Coordinator → Worker
- **操作步骤**:
  1. Coordinator 分配任务给 Worker
  2. Worker 执行 `git pull --rebase upstream main` 到 local main
  3. 验证 cargo test 通过
  4. push 到 origin/main
  5. 验证 origin/main = upstream/main

### TASK-ITER10-003: 清理未追踪的 test/ 目录
- **描述**: 仓库根目录存在 `test/` 未追踪目录，内容仅一个"重置状态.md"文件，疑似调试遗留，应删除。
- **状态**: pending
- **优先级**: P1
- **负责人**: Worker

### TASK-ITER10-004: 评估 query.rs TODO 并形成改进建议
- **描述**: query.rs 中有 8 个 TODO，分布在上下文压缩、shell 问题、memory_content 等处。评估可行性和优先级，形成改进建议。
- **状态**: pending
- **优先级**: P2
- **负责人**: Worker
- **产出**: 在 tasks/planner/backlog.md 中添加改进条目

---

## 进行中任务

<!-- 正在执行的任务 -->

---

## 已完成任务

<!-- 已完成的任务 -->
