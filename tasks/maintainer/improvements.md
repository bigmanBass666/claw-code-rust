# 改进队列

此文件记录 Maintainer 提出的改进建议及其状态。

## 状态说明
- `proposed` — 已提出，待审批
- `approved` — 用户批准，待实施
- `implementing` — 正在实施
- `done` — 已完成
- `rejected` — 用户拒绝

#### 改进列表

### [IMP-2026-0421-001] 建立日志基础设施
- **优先级**: P1
- **类型**: tool
- **状态**: done
- **提出时间**: 2026-04-21 04:59
- **目标**: 确保所有 Agent 都有日志记录
- **问题**: 5个Agent日志文件缺失(system.log, pr-manager.log, housekeeper.log, coo.log, maintainer.log)，无法完整追踪Agent行为
- **方案**: 1) 为每个Agent创建日志文件模板 2) 在Agent指令中明确日志记录规范 3) 将日志记录作为Agent唤醒协议的必要步骤
- **涉及文件**: 各Agent的instructions.md, AGENTS.md
- **预期效果**: 所有Agent行为可追踪
- **实施结果**: 5个日志文件已创建（含标准格式头），5个Agent的instructions.md唤醒协议已添加WAKEUP日志步骤

### [IMP-2026-0421-002] Worker心跳机制强化
- **优先级**: P1
- **类型**: flow
- **状态**: done
- **提出时间**: 2026-04-21 04:59
- **目标**: 防止任务卡住无法察觉
- **问题**: TASK-ITER11-003分配给Worker-002但无任何日志，状态卡在in_progress
- **方案**: 1) Worker每5分钟更新tasks/workers/status.md心跳时间戳 2) Coordinator定期检查心跳，超时则重分配 3) 任务超过30分钟无进度自动告警
- **涉及文件**: tasks/workers/status.md, tasks/coordinator/instructions.md, tasks/maintainer/instructions.md
- **预期效果**: 任务阻塞可及时发现
- **实施结果**: Worker心跳规范已定义（唤醒/认领/里程碑/完成时更新），Coordinator 30min心跳检查+60min升级Planner，Maintainer 24h任务超时告警

## 格式模板

```markdown
### [IMP-XXX] [标题]
- **优先级**: P0/P1/P2/P3
- **类型**: flow(流程)/tool(工具)/doc(文档)/arch(架构)
- **状态**: proposed/approved/implementing/done/rejected
- **提出时间**: YYYY-MM-DD HH:MM
- **目标**: 改进什么
- **问题**: 解决什么问题（基于日志分析）
- **方案**: 具体怎么做
- **涉及文件**: 需要修改哪些文件
- **预期效果**: 预期改善什么指标
- **实施结果**: （done 后填写）
```
