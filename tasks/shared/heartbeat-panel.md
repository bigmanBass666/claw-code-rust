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
| Planner | 🌙 Dormant | 0 | — | — | 系统已就绪，等待唤醒 |
| Coordinator | 🌙 Dormant | 0 | — | — | 系统已就绪，等待唤醒 |
| Worker | 🌙 Dormant | 0 | — | — | 系统已就绪，等待任务 |
| PR Manager | 🌙 Dormant | 0 | — | — | 系统已就绪，等待新任务 |
| Maintainer | 🌙 Dormant | 0 | — | — | 系统已就绪，定期分析待触发 |
| Housekeeper | 🌙 Dormant | 0 | — | — | ⚠️ 待处理分支清理任务(MRG-HK-001) |
| COO | 🌙 Dormant | 0 | — | — | 系统已就绪，等待用户唤醒 |

> 💡 工作区列：标识每个 Agent 当前所在的 git 工作位置（main 分支或 worktree 目录）
> 📋 ITER12 已归档至 `tasks/logs/iteration-12-archive-2026-04-23.md`
> 🚀 系统已重置，所有 Agent 待机中
