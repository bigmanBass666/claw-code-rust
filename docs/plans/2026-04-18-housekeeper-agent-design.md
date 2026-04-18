# Housekeeper Agent 设计文档

## 概述

Housekeeper Agent 是六层 Agent 架构中的**执行层**，专门负责 origin 仓库的分支清理工作，保持仓库整洁。

## 角色定位

```
Planner → Coordinator → Worker → PR Manager → Maintainer → Housekeeper
                                                              ↑
                                                    （第六层 — 仓库守护者）
```

### 与其他 Agent 的关系

| Agent | 职责 | 与 Housekeeper 的关系 |
|-------|------|----------------------|
| PR Manager | 提 PR、追踪状态 | PR 合并后通知 Housekeeper 清理分支 |
| Maintainer | 分析日志、提出改进 | Housekeeper 向其汇报清理结果 |
| Planner | 决策 | 可下发周期性检查任务 |

## 职责范围

### 核心职责
- **清理已合并的 feat/ 分支**
- **清理已过期的 dev/ 分支**
- **识别并报告孤立分支**

### 永不删除的分支
- `main` — 主开发分支
- `upstream/*` — 上游仓库分支
- `origin/main` — 远程主分支

## 触发机制

### 主触发：PR 合并通知
```
PR Manager 完成 PR 合并后
    ↓
写入 tasks/housekeeper/cleanup-queue.md
    ↓
Housekeeper 读取队列，执行清理
    ↓
记录到 tasks/logs/housekeeper.log
    ↓
向 Maintainer 汇报结果
```

### 安全网：周期性检查
- **频率**：每 24 小时
- **条件**：超过 7 天未更新的 dev/ 分支标记为待清理

## 判断规则

| 分支模式 | 条件 | 动作 |
|----------|------|------|
| `feat/<name>` | 对应 PR 已合并到 upstream | ✅ 自动删除 |
| `feat/<name>` | PR 已关闭（非合并） | ⚠️ 报告待确认 |
| `dev/<name>` | 超过 7 天无更新 | ⚠️ 报告给 Maintainer |
| `agent/<name>` | 超过 14 天无更新 | ⚠️ 报告待确认 |
| `test-<name>` | 任何时候 | ⚠️ 报告待确认 |
| `main` / `upstream/*` | 任意 | ❌ 永不删除 |

## 输出产物

### 日志文件
位置：`tasks/logs/housekeeper.log`
```
[2026-04-18 22:00:00] [Housekeeper] [INFO] 启动分支清理检查
[2026-04-18 22:00:01] [Housekeeper] [INFO] 发现 3 个待清理分支
  - feat/null-array-fix-v2 (PR #40 已合并)
  - feat/clippy-fixes (PR 已关闭)
  - dev/tools0412 (14天无更新)
[2026-04-18 22:00:05] [Housekeeper] [INFO] 已删除 feat/null-array-fix-v2
[2026-04-18 22:00:06] [Housekeeper] [WARN] 需要确认: feat/clippy-fixes, dev/tools0412
```

### 清理队列
位置：`tasks/housekeeper/cleanup-queue.md`
```markdown
## 待清理队列

### 自动清理（已确认安全）
| 分支名 | 原因 | 创建时间 |
|--------|------|----------|
| feat/null-array-fix-v2 | PR #40 已合并 | 2026-04-17 |

### 需要确认
| 分支名 | 原因 | 建议动作 |
|--------|------|----------|
| feat/clippy-fixes | PR 已关闭 | 删除 |
| dev/tools0412 | 14天无更新 | 删除 |
```

## 实现位置

```
tasks/
├── housekeeper/              # Housekeeper 专用
│   ├── instructions.md      # Agent 指令
│   └── cleanup-queue.md      # 清理队列
├── logs/
│   └── housekeeper.log      # 运行日志
└── ARCHITECTURE.md          # 更新架构图
```

## 实施步骤

1. 创建 `tasks/housekeeper/instructions.md`
2. 创建 `tasks/housekeeper/cleanup-queue.md`
3. 更新 `tasks/ARCHITECTURE.md` 加入第六层
4. 更新 `AGENTS.md` 加入 Housekeeper 说明
5. 更新 PR Manager 指令，在 PR 合并后通知 Housekeeper

## 注意事项

- **不删除本地分支** — 只清理 origin 的远程分支
- **记录所有操作** — 便于追踪和回溯
- **保留确认环节** — 模糊情况报告给 Maintainer
