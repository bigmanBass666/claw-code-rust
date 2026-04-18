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

### [TASK-010] 修复 CJK 文本 panic（Issue #36）
- **优先级**: P0
- **描述**: 修复 `crates/provider/src/text_normalization.rs` 中 `earliest_partial_start()` 函数的 byte indexing 问题。将 `for start in 0..text.len()` 改为 `for (start, _ch) in text.char_indices()`。这是对应 Issue #36 的修复。此 bug 导致所有 CJK 用户在使用流式响应时 panic。
- **期望结果**: CJK 文本不再 panic，所有测试通过，`cargo fmt` + `cargo clippy` 通过
- **值得提 PR**: 是 — 有对应 Issue #36，一行改动，高价值
- **截止时间**: 2026-04-19
- **依赖**: TASK-009 ✅
- **状态**: assigned
- **分配给**: Worker-001
- **创建时间**: 2026-04-19
- **更新时间**: 2026-04-19
- **分支策略**: 从 upstream/main 创建 `agent/worker-001/fix-cjk-panic`
- **预估文件数**: 1

### [TASK-011] 重新提取 Windows UNC path 修复为干净分支
- **优先级**: P1
- **描述**: 当前 PR #42 的分支 `feat/fix-windows-unc-path` 包含大量无关改动（64文件，5597行删除）。需要从 upstream/main 创建新分支，只 cherry-pick 核心修复 commit（`35dab7b fix: strip Windows UNC prefix from canonicalized CLAWCR_HOME path`），生成干净的 PR 替代 #42。注意：cherry-pick 后需检查 `strip_unc_prefix` 函数是否完整保留。
- **期望结果**: 新分支只含 1 个 commit，只改 `crates/utils/src/home_dir.rs`，通过所有质量检查
- **值得提 PR**: 是 — 替代 PR #42
- **截止时间**: 2026-04-19
- **依赖**: TASK-009 ✅
- **状态**: assigned
- **分配给**: Worker-002
- **创建时间**: 2026-04-19
- **更新时间**: 2026-04-19
- **分支策略**: 从 upstream/main 创建 `feat/fix-windows-unc-path-v2`
- **预估文件数**: 1

### [TASK-012] 清理远程分支
- **优先级**: P2
- **描述**: 删除以下无用远程分支：`dev/tools0412`（0 commit ahead）、`dev/wang`（0 commit ahead）、`test-mcp-branch`（0 commit ahead）、`feat/clippy-fixes`（不值得提 PR）。评估 `feat/fix-log-level-prompt-mode` 是否需要保留（含功能代码但分支不干净，3 commit ahead）。
- **期望结果**: 远程分支列表干净，只保留有价值的分支
- **值得提 PR**: 否
- **截止时间**: 2026-04-19
- **依赖**: TASK-009 ✅
- **状态**: assigned
- **分配给**: Worker-003
- **创建时间**: 2026-04-19
- **更新时间**: 2026-04-19

---

## 进行中任务

<!-- 正在执行的任务 -->

---

## 已完成任务

### [TASK-009] 配置 upstream 远程仓库
- **优先级**: P0
- **描述**: 添加 `upstream` 远程仓库指向 `https://github.com/7df-lab/claw-code-rust.git`，执行 `git fetch upstream` 确认可用。
- **完成时间**: 2026-04-19
- **执行者**: Coordinator
