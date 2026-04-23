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

| Agent | 状态 | 心跳计数 | 最后活跃时间 | 备注 |
|-------|------|---------|-------------|------|
| Coordinator | 💓 Heartbeat | 1 | 2026-04-23 14:11:50 | 处理 MSG-004 + MSG-005；已分配 ITER12 任务给 Worker |
| Worker | 💤 Standby | 2 | 2026-04-23 15:06:57 | ITER12-001~004 ✅ 全部完成；ITER12 第一批基础层完成 |
| Planner | 🌙 Dormant | 0 | — | Test #10已完成，ITER12已下发 |
| PR Manager | 💓 Heartbeat | 1 | 2026-04-23 15:06:57 | Test #12：处理 ITER12 PR#31~33 创建请求 |
| Maintainer | 🌙 Dormant | 0 | — | — |
| Housekeeper | 🌙 Dormant | 0 | — | — |
| COO | 🌙 Dormant | 0 | — | — |
