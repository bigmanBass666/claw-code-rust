# ValveOS v0.2.0 首次演练计划

> 目标：验证整个系统从命令输入到 Agent 协作的完整链路是否通畅。
> 你的角色：阀门（用户）。只负责发信号，不替 Agent 做决策。

---

## 演练场景：上游同步 + 小改进任务

> 选择理由：简单、真实、能走通完整流水线（Planner → Coordinator → Worker）

### 第一阶段：系统自检（~3 条命令）

按顺序发送以下命令，观察输出格式：

1. **`/help`** — 验证斜杠命令协议工作，确认 5 个命令都列出
2. **`/status`** — 验证 agent-status.md 被正确读取，看到 v0.2.0 状态
3. **`/log`** — 验证日志文件可读（可能为空或只有少量记录）

### 第二阶段：启动真实任务流（~4 步）

4. **唤醒 Planner**：发送「唤醒 Planner，我想让系统执行一次 upstream 同步检查，看看我们落后上游多少」

   预期行为：
   - AI 读 AGENTS.md → 识别到"唤醒 Planner"
   - 读 planner/instructions.md → 执行醒来协议（读 inbox + agent-status + iteration-log）
   - 输出标准开场白 + 当前状态摘要
   - 分析任务可行性，输出决策
   - 写入 coordinator inbox
   - 输出：「请唤醒 Coordinator」+ 一句话原因

5. **唤醒 Coordinator**：发送「唤醒 Coordinator」

   预期行为：
   - 读 coordinator inbox → 发现 Planner 的任务
   - 拆分任务为子任务
   - 写入 worker inbox 或 assignments.md
   - 输出：「请唤醒 Worker」+ 分配详情

6. **唤醒 Worker**：发送「唤醒 Worker」

   预期行为：
   - 读 worker inbox/assignments → 认领任务
   - 执行 `git fetch upstream`、`git log` 等操作
   - 完成后写入 coordinator inbox 或 pr-manager inbox
   - 输出完成摘要 + 下一步建议

7. **可选：如果 Worker 发现了可改进点**
   
   发送「唤醒 PR Manager」或继续在当前会话讨论发现

---

## 观察清单（每步都要注意）

| 检查项 | 通过标志 |
|--------|---------|
| AI 是否读了 AGENTS.md？ | 开头应该有上下文 |
| 是否用了标准开场白？ | 应该说"我是 XXX（角色）" |
| 是否写了 inbox？ | 不应只输出不写入 |
| 是否以"请唤醒 XXX"结尾？ | 铁门协议 |
| 日志是否被记录？ | 检查 system-commands.log 有无新条目 |
| 安全铁律是否被遵守？ | 没有 push 到 upstream 等 |

## 如果某一步卡住了

- Agent 不知道该做什么 → 检查 instructions.md 是否有足够细节
- Agent 直接执行了没问用户 → 检查社交边界是否清晰
- Agent 说"我无法完成" → 记录下来，这是 instructions 需要改进的地方
- 流程完全跑通了 🎉 → 记录到 session-reports，这就是第一次成功！

---

## 演练后的收尾

无论成功与否，最后发送：

**`/audit`** 或直接「帮我审计一下系统一致性」

→ 让 COO 检查本次演练有没有留下不一致的文件状态

---

## 备选场景（如果主场景太复杂）

### 简化版：纯命令测试
- 只做第一阶段（/help → /status → /log）
- 目标：验证斜杠协议本身工作正常

### 中等版：单 Agent 往返
- 唤醒 Housekeeper 执行一次分支清理检查
- 只涉及一个 Agent，不触发完整流水线
