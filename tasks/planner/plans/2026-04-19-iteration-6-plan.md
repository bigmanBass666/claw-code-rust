# Iteration 6 计划

> 日期: 2026-04-19
> 触发: 用户唤醒 Planner
> 状态: 执行中

## 当前项目状态

- **编译**: ✅ 通过
- **Git**: 工作树干净，main 分支
- **upstream**: ❌ 未配置（只有 origin）
- **上游组织**: `7df-lab/claw-code-rust`

## 识别的问题/机会

### 紧急问题
1. **upstream 远程仓库未配置** — 无法 fetch 上游最新代码，Worker 无法从 upstream/main 创建分支
2. **PR #42 分支含垃圾 commit** — `feat/fix-windows-unc-path` 包含 64 个文件、5597 行删除，远超 PR 质量标准
3. **Issue #36: CJK panic** — 高严重性 bug，`text_normalization.rs` 中 byte indexing 导致 CJK 文本 panic

### 重要机会
4. **Issue #36 修复** — 一行改动（`0..text.len()` → `text.char_indices()`），高价值、小改动、有 issue 对应
5. **feat/fix-log-level-prompt-mode 重新提取** — 有实际功能代码但分支不干净

### 清理需求
6. **3个空远程分支** — `dev/tools0412`、`dev/wang`、`test-mcp-branch` 可删除
7. **feat/clippy-fixes 分支** — 不值得提 PR，可删除

## 任务列表

### TASK-009: 配置 upstream 远程仓库
- **优先级**: P0
- **描述**: 添加 `upstream` 远程仓库指向 `https://github.com/7df-lab/claw-code-rust.git`，执行 `git fetch upstream` 确认可用
- **期望结果**: `git remote -v` 显示 upstream，`git fetch upstream` 成功
- **值得提 PR**: 否（本地配置）
- **依赖**: 无

### TASK-010: 修复 CJK 文本 panic（Issue #36）
- **优先级**: P0
- **描述**: 修复 `crates/provider/src/text_normalization.rs` 中 `earliest_partial_start()` 函数的 byte indexing 问题。将 `for start in 0..text.len()` 改为 `for (start, _ch) in text.char_indices()`。这是对应 Issue #36 的修复。
- **期望结果**: CJK 文本不再 panic，所有测试通过，`cargo fmt` + `cargo clippy` 通过
- **值得提 PR**: 是 — 有对应 Issue #36，一行改动，高价值
- **依赖**: TASK-009（需要 upstream/main 作为基准）
- **预估文件数**: 1 个文件
- **分支策略**: 从 upstream/main 创建 `agent/worker-001/fix-cjk-panic`

### TASK-011: 重新提取 Windows UNC path 修复为干净分支
- **优先级**: P1
- **描述**: 当前 PR #42 的分支 `feat/fix-windows-unc-path` 包含大量无关改动（64文件）。需要从 upstream/main 创建新分支，只 cherry-pick 核心修复 commit（`35dab7b fix: strip Windows UNC prefix from canonicalized CLAWCR_HOME path`），生成干净的 PR 替代 #42。
- **期望结果**: 新分支只含 1 个 commit，只改 `crates/utils/src/home_dir.rs`，通过所有质量检查
- **值得提 PR**: 是 — 替代 PR #42
- **依赖**: TASK-009
- **预估文件数**: 1 个文件
- **分支策略**: 从 upstream/main 创建 `feat/fix-windows-unc-path-v2`

### TASK-012: 清理远程分支
- **优先级**: P2
- **描述**: 删除以下无用远程分支：`dev/tools0412`、`dev/wang`、`test-mcp-branch`、`feat/clippy-fixes`。评估 `feat/fix-log-level-prompt-mode` 是否需要保留。
- **期望结果**: 远程分支列表干净，只保留有价值的分支
- **值得提 PR**: 否
- **依赖**: TASK-009（需要确认 upstream 状态后再清理）

## 任务依赖关系

```
TASK-009 (upstream配置)
    ├── TASK-010 (CJK修复) — 依赖009
    ├── TASK-011 (UNC重新提取) — 依赖009
    └── TASK-012 (分支清理) — 依赖009
```

## 执行策略

- **TASK-009 先行**：所有后续任务依赖 upstream 配置
- **TASK-010 和 TASK-011 可并行**：无文件冲突，可分配给不同 Worker
- **TASK-012 最后**：等其他分支使用完毕后再清理

## PR 可行性评估

| 任务 | 有 Issue | 改动小 | 上游会接受 | 评估 |
|------|----------|--------|-----------|------|
| TASK-010 | ✅ #36 | ✅ 1文件1行 | ✅ 高严重性 bug | **强烈推荐** |
| TASK-011 | ✅ PR #42 | ✅ 1文件 | ✅ 已有 PR 在审 | **推荐** |
