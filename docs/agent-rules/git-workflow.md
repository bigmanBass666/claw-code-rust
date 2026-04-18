# Git 工作流与上游协作

## 远程仓库

```
origin (bigmanBass666/claw-code-rust)  ← 你的 fork
upstream (claw-cli/claw-code-rust)     ← 上游（只读）
```

**首次设置**：
```bash
git remote add upstream https://github.com/claw-cli/claw-code-rust.git
git fetch upstream
```

---

## 分支策略

### 分支类型

| 分支名 | 用途 | 基于 | 推送到 |
|--------|------|------|--------|
| `main` | 开发分支，包含所有 AI 文件 | - | origin |
| `agent/planner/<task>` | Planner 工作分支 | `main` | origin |
| `agent/coordinator/<task>` | Coordinator 工作分支 | `main` | origin |
| `agent/worker-<id>/<task>` | Worker 工作分支 | **`upstream/main`** | origin |
| `feat/<description>` | 准备提 PR 的干净分支 | **`upstream/main`** | origin |

### 五层 Agent 流程

```
Planner → Coordinator → Worker → PR Manager → Maintainer
   ↓         ↓           ↓          ↓           ↓
 决策    分配任务     执行代码    提取干净PR   分析改进
```

**Maintainer** 是第五层，负责日志分析 → 发现问题 → 提出改进 → 持续优化系统。

### 关键规则

1. **Worker 和 feat/ 分支必须基于 `upstream/main`**
   - 这样它们的 diff 天然就是干净的
   - 不包含 main 上积累的 AI 协调文件（tasks/、notifications/ 等）

2. **Planner 和 Coordinator 的分支基于 `main`**
   - 它们需要访问 tasks/ 协调文件
   - 它们的工作不需要提 PR

3. **不要在 main 上直接做要给上游的改动**
   - main 是开发分支，可以乱
   - 提 PR 必须用 feat/ 分支

### 分支命名规范

- **Worker**: `agent/worker-001/fix-windows-unc`
- **PR**: `feat/42-fix-windows-unc-path`
- **功能**: `feat/improve-error-messages`

---

## 多 Agent 协作的完整流程

### 开发阶段

```
1. Planner 决策
   └→ "我们需要修复 Windows UNC 路径问题"

2. Coordinator 分配
   └→ 将任务分配给 Worker-001

3. Worker 开发
   a. git fetch upstream
   b. git checkout -b agent/worker-001/fix-windows-unc upstream/main
   c. 编写代码、测试、提交
   d. push 到 origin
   e. 通知完成
```

### PR 准备阶段

```
4. PR Manager 准备 PR
   a. 检查 Worker 的分支
   b. 创建 feat/fix-windows-unc (基于 upstream/main)
   c. cherry-pick Worker 的相关 commit
   d. 运行质量检查：
      - cargo fmt --check
      - cargo clippy
      - cargo test
      - 检查 diff 是否干净
   e. 如果通过 → 生成 PR 描述
   f. 如果失败 → 返回给 Worker 修复

5. 用户审批
   └→ 查看 PR 描述和质量报告

6. 提交 PR
   └→ 从 feat/xxx 向 upstream/main 提 PR
```

---

## 提交信息

- 格式：`type: 简短描述`
- 类型：`feat:` `fix:` `refactor:` `test:` `docs:` `chore:`
- 在提交正文中引用相关 issue

### 好的 commit 信息示例
```
fix: strip Windows UNC prefix from canonicalized path  ✅
fix: handle null arrays in OpenAI responses            ✅
```

### 不好的 commit 信息示例
```
chore: apply clippy fixes across workspace             ❌ 太泛
chore: run cargo clippy --fix                         ❌ 太懒
```

---

## 提交频率

- **`main`（个人维护分支）**：可以频繁提交，每次改动随时 commit
- **`agent/xxx`（Agent 工作分支）**：可以频繁提交
- **`feat/xxx`（给上游提 PR 的分支）**：只放干净的、相关的 commit

---

## 提交 PR 前（必须全部通过才能 push）

1. `cargo fmt --all -- --check` 无差异
2. `cargo clippy --workspace --all-targets` 无错误
3. `cargo test --workspace` 全部通过
4. 验证上游兼容性
5. 写清晰的 PR 描述：做什么/为什么/怎么做
6. **绝对不能跳过以上任何步骤**

### Diff 清洁度检查

```bash
# 检查 PR 会包含哪些文件
git diff upstream/main --name-only

# 确保不包含以下内容：
# - tasks/
# - notifications/
# - .trae/
# - AGENTS.md
# - progress.txt
```

---

## 上游协作

- 开始重要工作前务必检查上游，避免重复劳动
- 上游合并相关变更时，rebase 或 merge 保持本地同步
- 及时响应维护者对 PR 的反馈

---

## 开始重要工作前检查

1. 上游是否已实现了这个功能？
2. 是否已有相关的 open issue 或 PR？
3. 是否会与某个 open PR 冲突？
4. 是否有需要更新或添加的测试？
5. 是否需要更新文档？

---

## PR Manager 角色

PR Manager 是专门负责将 AI 工作转化为干净 PR 的 Agent：

- 从 agent/worker-xxx 分支提取干净的功能改动
- 创建基于 upstream/main 的 feat/xxx 分支
- 自动化执行所有质量检查
- 生成 PR 描述
- 向用户汇报，等待审批

详见 `tasks/pr-manager/instructions.md`
