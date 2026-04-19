# Iteration 7 计划

> 创建时间: 2026-04-20
> 创建者: Planner

## 项目状态总结

### 健康
- ✅ 编译通过 (`cargo check --workspace --all-targets`)
- ✅ upstream 远程仓库已配置，`upstream/main` 已 fetch
- ✅ CJK panic (Issue #36) 已在上游修复（`char_indices()` 已在代码中）
- ✅ Windows UNC path 修复已在上游采纳（`82e2d40` 从 PR #42 提取了 home_dir.rs）

### 问题
- ⚠️ PR #42 仍 OPEN + CONFLICTING，但核心修复已被上游采纳（cherry-pick），PR 可关闭
- ⚠️ `origin/feat/fix-windows-unc-path` 旧分支包含不相关提交，需清理
- ⚠️ Issue #36 仍 OPEN，但修复已在上游 main

### 机会
- 🟢 Issue #35 (prompt mode + --model) 维护者已批准，等待 PR
- 🟢 上游有多个开放 Issue 可贡献（#10 tool calling, #9 context management 等）
- 🟢 本地 main 分支与 upstream/main 有大量差异（ValveOS 文件），需要 rebase 同步

## 识别的问题/机会

1. **PR #42 可关闭** — 核心修复已被上游采纳，PR 因不相关提交而 CONFLICTING
2. **Issue #36 可关闭** — 修复已在上游 main
3. **origin/feat/fix-windows-unc-path 需要清理** — 旧分支包含不相关提交
4. **本地 main 需要同步上游** — upstream/main 有新提交，本地 main 落后
5. **Issue #35 可以提 PR** — prompt mode + --model 已获维护者批准

## 任务列表

### TASK-013: 同步本地 main 到 upstream/main
- **优先级**: P0
- **描述**: 将本地 main 分支 rebase 到 `upstream/main`，同步上游最新代码（包括 UNC path 修复、null array fix、log-level fix 等），同时保留 ValveOS 文件
- **期望结果**: 本地 main 包含上游最新代码 + ValveOS 文件，编译通过
- **值得提 PR**: 否
- **依赖**: 无
- **注意**: rebase 可能产生冲突（ValveOS 文件与上游不冲突，但 Cargo.toml 等可能冲突）

### TASK-014: 关闭 PR #42 和 Issue #36
- **优先级**: P1
- **描述**: 
  1. 在 PR #42 评论说明核心修复已被上游采纳（`82e2d40`），请求关闭
  2. 在 Issue #36 评论确认修复已合并到 main，请求关闭
- **期望结果**: PR #42 和 Issue #36 关闭
- **值得提 PR**: 否（评论操作，需用户审批后执行）
- **依赖**: TASK-013（确认本地代码与上游一致后再评论）

### TASK-015: 清理 origin/feat/fix-windows-unc-path 旧分支
- **优先级**: P2
- **描述**: 删除 origin 上的旧分支 `feat/fix-windows-unc-path`（包含不相关提交），避免混淆
- **期望结果**: 旧分支已删除
- **值得提 PR**: 否
- **依赖**: TASK-014 完成后（PR 关闭后再删分支）

## 任务依赖关系

```
TASK-013 ──→ TASK-014 ──→ TASK-015
```

## 执行策略

- 串行执行：先同步代码(TASK-013)，再关闭 PR/Issue(TASK-014)，最后清理分支(TASK-015)
- TASK-013 是基础，确保本地代码与上游一致
- TASK-014 的评论操作需要用户审批后才能执行（社交边界）

## PR 可行性评估

| 任务 | 对上游价值 | 符合贡献规范 | PR大小 | 有Issue | 评估 |
|------|-----------|-------------|--------|---------|------|
| TASK-013 | 内部操作 | N/A | N/A | N/A | 不提PR |
| TASK-014 | 中(社区沟通) | ✅ | N/A | #36+#42 | 评论操作 |
| TASK-015 | 低(仓库整洁) | ✅ | N/A | N/A | 内部操作 |
