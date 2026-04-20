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

### TASK-016: 同步 local main 到 upstream/main
- **优先级**: P0
- **描述**: 将 upstream/main 的 8 个新提交合并到 local main。local main 有大量 ValveOS 专属提交，需谨慎 merge。在 worktree 中操作，确认无冲突后再合并到主仓库。上游新提交包括：PR #37/#40 合并、PR #42 修复采纳、log-level 修复等。
- **期望结果**: `git merge-base --is-ancestor upstream/main main` 返回 0；编译通过
- **值得提 PR**: 否（内部操作）
- **截止时间**: 2026-04-21
- **依赖**: 无

### TASK-017: 关闭 PR #42、Issue #36、Issue #35
- **优先级**: P1
- **描述**:
  1. 在 PR #42 评论说明核心修复已被上游采纳（commit 82e2d40 `fix: take crates/utils/src/home_dir.rs from PR #42`），请求关闭
  2. 在 Issue #36 评论确认修复已合并到 main（`char_indices()` 在 `text_normalization.rs`），请求关闭
  3. 在 Issue #35 评论确认功能已通过 PR #37 合并到上游，请求关闭
- **期望结果**: PR #42、Issue #36、Issue #35 关闭
- **值得提 PR**: 否（评论操作，需用户审批后执行）
- **截止时间**: 2026-04-22
- **依赖**: TASK-016

### TASK-018: 清理 origin/feat/fix-windows-unc-path 旧分支
- **优先级**: P2
- **描述**: 删除 origin 上的旧分支 `feat/fix-windows-unc-path`（包含不相关提交），避免混淆
- **期望结果**: 旧分支已删除
- **值得提 PR**: 否
- **截止时间**: 2026-04-22
- **依赖**: TASK-017

### TASK-019: 评估上游 Issue 贡献可行性
- **优先级**: P2
- **描述**: 分析上游 open issues 的贡献可行性：
  - #10 (tool calling refactor) — 评估改动范围、是否需要先讨论
  - #9 (context management) — 评估改动范围
  - #16 (Ollama provider) — 评估改动范围
  输出评估报告到 backlog.md
- **期望结果**: 每个候选 issue 有可行性评估（改动范围、PR大小预估、是否需先开issue讨论）
- **值得提 PR**: 否（评估阶段）
- **截止时间**: 2026-04-22
- **依赖**: 无

---

## 进行中任务

<!-- 正在执行的任务 -->

---

## 已完成任务

<!-- 已完成的任务 -->
