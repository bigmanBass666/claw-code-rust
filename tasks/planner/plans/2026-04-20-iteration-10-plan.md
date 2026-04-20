# Iteration 10 计划

> 制定时间: 2026-04-20
> 制定者: Planner

## 背景

Iteration 9 已废弃（系统重置）。Iteration 10 从空白开始。

观察循环结果:
- 编译: ✅ cargo check 通过
- 测试: ✅ cargo test 全部通过
- Git: local main = origin/main = cbae86b，无未提交改动
- Upstream: git fetch upstream 成功，但 sandbox 中 tracking ref 不持久化（需要进一步验证）
- TODO: 10 个散落在 query.rs 等文件

## 目标

P0 验证系统健康 + 评估 TODO 改进机会

## 任务

### TASK-ITER10-001: 验证 upstream/main 同步状态
- **描述**: 验证 local main 是否与 upstream/main 同步。git fetch upstream 成功但 tracking ref 在 sandbox 中不持久化，需通过其他方式确认。
- **优先级**: P0
- **负责人**: Planner（直接执行，不需要 Worker）
- **验证方法**: 对比 cbae86b 与 git fetch 后可见的上游 commit

### TASK-ITER10-002: 清理未追踪的 test/ 目录
- **描述**: 仓库根目录存在 `test/` 未追踪目录，内容仅一个"重置状态.md"文件，疑似调试遗留，应清理。
- **优先级**: P1
- **负责人**: Worker
- **操作**: 删除 test/ 目录，确认为无害文件后 commit

### TASK-ITER10-003: 评估 query.rs 中 TODO 并形成改进建议
- **描述**: 10 个 TODO 主要集中在 query.rs（8个），评估其可行性和优先级，形成改进建议。
- **优先级**: P2
- **负责人**: Worker

## 执行顺序

1. TASK-ITER10-001 (Planner 直接验证)
2. TASK-ITER10-002 (Coordinator → Worker)
3. TASK-ITER10-003 (Coordinator → Worker)

## 备注

- 上游新分支: upstream/dev/tools0412, upstream/dev/wang, upstream/revert-37-feat/prompt-cli-only — 这些分支不影响当前计划
- 如 TASK-ITER10-001 发现 local main 落后，执行 sync 后先验证 cargo test 通过
