# 心跳控制面板

> 本面板追踪所有 Agent 的心跳状态。由 Agent 自行更新。
> 详见 `docs/agent-rules/heartbeat-protocol.md`

## 状态说明

| Emoji | 状态 | 含义 |
|-------|------|------|
| 🌙 | Dormant | 未启动心跳 |
| 💓 | Heartbeat | 轮询中，等待任务 |
| ⚡ | Working | 正在处理任务 |
| 💤 | Standby | 空闲轮询 |

## Agent 状态

| Agent | 状态 | 心跳计数 | 最后活跃时间 | 工作区 | 备注 |
|-------|------|---------|-------------|--------|------|
| Coordinator | 🌙 Dormant | 0 | — | main | Test #12 已暂停，等待重新运行 |
| Worker | 🌙 Dormant | 0 | — | main | ITER12-001~004 ✅ 全部完成 |
| Planner | 🌙 Dormant | 0 | — | main | Test #10已完成，ITER12已下发 |
| PR Manager | 💓 Heartbeat | 2 | 2026-04-23 21:52:35 | worktree/D:/Test/claw-code-rust-pr | ✅ ITER12 PR#31,32,33 已验证合并 |
| Maintainer | 🌙 Dormant | 0 | — | main | — |
| Housekeeper | 🌙 Dormant | 0 | — | main | — |
| COO | 🌙 Dormant | 0 | — | main | — |

> 💡 工作区列：标识每个 Agent 当前所在的 git 工作位置（main 分支或 worktree 目录）
