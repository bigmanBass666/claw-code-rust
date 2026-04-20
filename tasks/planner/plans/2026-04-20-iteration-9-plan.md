# Iteration 9 计划

> 创建时间: 2026-04-20
> 创建者: Planner

## 项目状态总结

### 健康
- ✅ 编译通过 (`cargo check --workspace --all-targets`)
- ✅ upstream 远程仓库已配置，`upstream/main` 已 fetch
- ✅ PR #37 (prompt-cli-only) 和 PR #40 (null-array-fix-v2) 已被上游合并
- ✅ PR #42 的核心修复已被上游采纳（commit 82e2d40）

### 问题
- ⚠️ **P0**: local main 落后 upstream/main 8 个提交，需同步
- ⚠️ PR #42 仍 OPEN + mergeable=False，核心修复已被上游采纳，可关闭
- ⚠️ Issue #36 仍 OPEN，CJK panic 修复已在上游 main
- ⚠️ Issue #35 仍 OPEN，prompt mode 已通过 PR #37 合并到上游
- ⚠️ `origin/feat/fix-windows-unc-path` 旧分支包含不相关提交，需清理
- ⚠️ test.md 为未追踪文件

### 机会
- 🟢 上游 open issues 可贡献：#10 (tool calling), #9 (context management), #16 (Ollama provider)
- 🟢 PR #30 (emit plan items) 仍在审核中

## 识别的问题/机会

1. **local main 落后 upstream/main** — 8 个提交未同步，含重要修复
2. **PR #42 可关闭** — 核心修复已被上游采纳
3. **Issue #36 可关闭** — 修复已在上游 main
4. **Issue #35 可关闭** — 功能已通过 PR #37 合并
5. **旧分支需清理** — origin/feat/fix-windows-unc-path
6. **上游开放 Issue 可贡献** — #10, #9, #16 等

## 任务列表

### TASK-016: 同步 local main 到 upstream/main
- **优先级**: P0
- **描述**: 将 upstream/main 的 8 个新提交合并到 local main。local main 有大量 ValveOS 专属提交，需谨慎 merge。在 worktree 中操作，确认无冲突后再合并到主仓库。
- **期望结果**: `git merge-base --is-ancestor upstream/main main` 返回 0；编译通过
- **值得提 PR**: 否（内部操作）
- **依赖**: 无

### TASK-017: 关闭 PR #42、Issue #36、Issue #35
- **优先级**: P1
- **描述**:
  1. 在 PR #42 评论说明核心修复已被上游采纳（commit 82e2d40），请求关闭
  2. 在 Issue #36 评论确认修复已合并到 main（`char_indices()` 在 `text_normalization.rs`），请求关闭
  3. 在 Issue #35 评论确认功能已通过 PR #37 合并，请求关闭
- **期望结果**: PR #42、Issue #36、Issue #35 关闭
- **值得提 PR**: 否（评论操作，需用户审批后执行）
- **依赖**: TASK-016 完成后（确认同步状态再操作）

### TASK-018: 清理 origin/feat/fix-windows-unc-path 旧分支
- **优先级**: P2
- **描述**: 删除 origin 上的旧分支 `feat/fix-windows-unc-path`（包含不相关提交），避免混淆
- **期望结果**: 旧分支已删除
- **值得提 PR**: 否
- **依赖**: TASK-017 完成后（PR 关闭后再删分支）

### TASK-019: 评估上游 Issue 贡献可行性
- **优先级**: P2
- **描述**: 分析上游 open issues 的贡献可行性：
  - #10 (tool calling refactor) — 评估改动范围、是否需要先讨论
  - #9 (context management) — 评估改动范围
  - #16 (Ollama provider) — 评估改动范围
  输出评估报告到 backlog.md
- **期望结果**: 每个候选 issue 有可行性评估（改动范围、PR大小预估、是否需先开issue讨论）
- **值得提 PR**: 否（评估阶段）
- **依赖**: 无（可与 TASK-016 并行）

## 任务依赖关系

```
TASK-016 (P0 sync) ──→ TASK-017 (P1 close) ──→ TASK-018 (P2 cleanup)
TASK-019 (P2 evaluate) ──→ [独立，可与 TASK-016 并行]
```

## 执行策略

- TASK-016 和 TASK-019 可并行执行
- TASK-016 是关键路径，必须先完成 sync 才能执行 TASK-017
- TASK-017 的评论操作需要用户审批后才能执行（社交边界）
- TASK-018 等待 PR #42 关闭后再执行

## PR 可行性评估

| 任务 | 对上游价值 | 符合贡献规范 | PR大小 | 有Issue | 评估 |
|------|-----------|-------------|--------|---------|------|
| TASK-016 | 内部操作 | N/A | N/A | N/A | 内部操作 |
| TASK-017 | 中(社区沟通) | ✅ | N/A | #35+#36+#42 | 评论操作 |
| TASK-018 | 低(仓库整洁) | ✅ | N/A | N/A | 内部操作 |
| TASK-019 | 高(贡献规划) | ✅ | N/A | #10/#9/#16 | 评估阶段 |
