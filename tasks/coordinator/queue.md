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

### TASK-014: 关闭 PR #42 和 Issue #36
- **优先级**: P1
- **描述**: 
  1. 在 PR #42 评论说明核心修复已被上游采纳（`strip_unc_prefix` 已在 `home_dir.rs`），请求关闭
  2. 在 Issue #36 评论确认修复已合并到 main（`char_indices()` 已在 `text_normalization.rs`），请求关闭
- **期望结果**: PR #42 和 Issue #36 关闭
- **值得提 PR**: 否（评论操作，需用户审批后执行）
- **依赖**: 无
- **注意**: 评论操作属于社交边界，需用户审批后才能执行。不可自主回复评论或关闭 PR/Issue

### TASK-015: 清理 origin/feat/fix-windows-unc-path 旧分支
- **优先级**: P2
- **描述**: 删除 origin 上的旧分支 `feat/fix-windows-unc-path`（包含不相关提交），避免混淆
- **期望结果**: 旧分支已删除
- **值得提 PR**: 否
- **依赖**: TASK-014（PR #42 关闭后再删分支）

---

## 进行中任务

<!-- 正在执行的任务 -->

---

## 已完成任务

### TASK-013: 同步本地 main 到 upstream/main
- **完成时间**: 2026-04-20 13:00
- **结果**: local main 已包含 upstream/main 全部提交，无需 rebase
- **验证**: `git merge-base --is-ancestor FETCH_HEAD main` 返回 0
