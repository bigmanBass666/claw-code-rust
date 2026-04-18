# Worker 消息收件箱

## 待处理消息

| 时间 | 来源 | 内容摘要 | 状态 |
|------|------|----------|------|
| 2026-04-19T17:00:00Z | Coordinator | TASK-010 分配给 Worker-001：修复 CJK 文本 panic（Issue #36）。详见下方完整指令。 | 已读(2026-04-19) |
| 2026-04-19T17:00:00Z | Coordinator | TASK-011 分配给 Worker-002：重新提取 Windows UNC path 修复为干净分支。详见下方完整指令。 | 已读(Worker-002) |
| 2026-04-19T17:00:00Z | Coordinator | TASK-012 分配给 Worker-003：清理远程分支。详见下方完整指令。 | 未读 |

## 完整任务指令

### Worker-001 / TASK-010: 修复 CJK 文本 panic（Issue #36）

- **任务ID**: TASK-010
- **优先级**: P0
- **值得提 PR**: 是
- **分支策略**: 从 `upstream/main` 创建分支 `agent/worker-001/fix-cjk-panic`
- **涉及文件**: `crates/provider/src/text_normalization.rs`
- **描述**: 修复 `earliest_partial_start()` 函数的 byte indexing 问题。将 `for start in 0..text.len()` 改为 `for (start, _ch) in text.char_indices()`。此 bug 导致所有 CJK 用户在使用流式响应时 panic。
- **期望结果**: CJK 文本不再 panic，所有测试通过，`cargo fmt` + `cargo clippy` 通过
- **前置条件**: upstream 已配置（TASK-009 已完成 ✅）
- **完成后**: commit + push 到 origin，然后写入 PR Manager inbox 通知完成

### Worker-002 / TASK-011: 重新提取 Windows UNC path 修复为干净分支

- **任务ID**: TASK-011
- **优先级**: P1
- **值得提 PR**: 是（替代 PR #42）
- **分支策略**: 从 `upstream/main` 创建分支 `feat/fix-windows-unc-path-v2`
- **涉及文件**: `crates/utils/src/home_dir.rs`
- **描述**: 当前 PR #42 的分支包含大量无关改动（64文件，5597行删除）。需要从 upstream/main 创建新分支，只 cherry-pick 核心修复 commit（`35dab7b fix: strip Windows UNC prefix from canonicalized CLAWCR_HOME path`），生成干净的 PR 替代 #42。
- **关键注意**: cherry-pick 后需检查 `strip_unc_prefix` 函数是否完整保留
- **期望结果**: 新分支只含 1 个 commit，只改 `crates/utils/src/home_dir.rs`，通过所有质量检查
- **前置条件**: upstream 已配置（TASK-009 已完成 ✅）
- **完成后**: commit + push 到 origin，然后写入 PR Manager inbox 通知完成

### Worker-003 / TASK-012: 清理远程分支

- **任务ID**: TASK-012
- **优先级**: P2
- **值得提 PR**: 否
- **分支策略**: 无需创建分支，直接在 main 上操作
- **描述**: 删除以下无用远程分支：
  - `dev/tools0412`（0 commit ahead）
  - `dev/wang`（0 commit ahead）
  - `test-mcp-branch`（0 commit ahead）
  - `feat/clippy-fixes`（不值得提 PR）
  - 评估 `feat/fix-log-level-prompt-mode` 是否需要保留（含功能代码但分支不干净，3 commit ahead）
- **期望结果**: 远程分支列表干净，只保留有价值的分支
- **前置条件**: upstream 已配置（TASK-009 已完成 ✅）
- **完成后**: 更新 assignments.md 状态为 completed，写入 Coordinator inbox 通知完成

## 已处理消息

| 时间 | 来源 | 内容摘要 | 处理时间 |
|------|------|----------|----------|
