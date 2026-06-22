# AgentPaint 中文安装指南

AgentPaint 有两个需要安装的部分：

- `agentpaint` CLI：Rust 编译出来的命令行程序，需要能从 `PATH` 调用。
- `agent-paint-pixel-skills` Skill：给 Codex、Claude Code、Cursor、Gemini 等 CLI/IDE Agent 读取的技能目录或规则文件。

正常使用 Skill 时不需要 AgentPaint 源码仓库。Agent 只需要读取已安装的 Skill，并调用 `PATH` 里的 `agentpaint`。

## 前置条件

需要 Rust 和 Cargo：

```bash
cargo --version
rustc --version
```

如果没有 Rust，先安装：

```text
https://rustup.rs/
```

## 一键安装 CLI 和默认 Skill

在仓库根目录运行：

```bash
sh ./scripts/install.sh --update-path
```

Windows PowerShell：

```powershell
.\scripts\install.ps1 -UpdatePath
```

默认行为：

- 用 `cargo install --path . --locked --force` 安装 `agentpaint`。
- 把 Skill 安装到通用/Codex 路径：`$HOME/.agents/skills/agent-paint-pixel-skills`。
- `--update-path` 会尝试提示或写入 Cargo bin 路径，让 `agentpaint` 能从 PATH 调用。

验证：

```bash
agentpaint --help
```

## 只安装 Skill

如果 CLI 已经安装，只想给某个 Agent 安装 Skill：

```bash
sh ./scripts/install.sh --skip-cli --skill-target claude-code
sh ./scripts/install.sh --skip-cli --skill-target gemini
sh ./scripts/install.sh --skip-cli --skill-target cursor --project-skills
```

Windows PowerShell：

```powershell
.\scripts\install.ps1 -SkipCli -SkillTargets claude-code
.\scripts\install.ps1 -SkipCli -SkillTargets gemini
.\scripts\install.ps1 -SkipCli -ProjectSkills -SkillTargets cursor
```

安装到所有支持目标：

```bash
sh ./scripts/install.sh --skip-cli --all-skill-targets
```

Windows PowerShell：

```powershell
.\scripts\install.ps1 -SkipCli -AllSkillTargets
```

## 支持的安装目标

原生 `SKILL.md` 目录目标：

| Target | 安装路径 |
| --- | --- |
| `universal` / `codex` | `$HOME/.agents/skills/agent-paint-pixel-skills` |
| `claude-code` | `$HOME/.claude/skills/agent-paint-pixel-skills` |
| `copilot` | `$HOME/.copilot/skills/agent-paint-pixel-skills` |
| `gemini` | `$HOME/.gemini/skills/agent-paint-pixel-skills` |
| `kiro` | `$HOME/.kiro/skills/agent-paint-pixel-skills` |
| `cline` | `$HOME/.cline/skills/agent-paint-pixel-skills` |
| `roo-code` | `$HOME/.roo/skills/agent-paint-pixel-skills` |
| `kilo-code` | `$HOME/.kilocode/skills/agent-paint-pixel-skills` |
| `factory` | `$HOME/.factory/skills/agent-paint-pixel-skills` |
| `goose` | `$HOME/.config/goose/skills/agent-paint-pixel-skills` |
| `opencode` | `$HOME/.config/opencode/skills/agent-paint-pixel-skills` |
| `antigravity` | `$HOME/.gemini/antigravity/skills/agent-paint-pixel-skills` |

规则适配目标：

| Target | 输出 |
| --- | --- |
| `cursor` | `.cursor/rules/agent-paint-pixel-skills.mdc` 或 `$HOME/.cursor/rules/agent-paint-pixel-skills.mdc` |
| `windsurf` | `.windsurf/rules/agent-paint-pixel-skills.md` 或 `$HOME/.codeium/windsurf/skills/agent-paint-pixel-skills.md` |
| `trae` | `.trae/rules/agent-paint-pixel-skills.md` 或 `$HOME/.trae/rules/agent-paint-pixel-skills.md` |
| `junie` | `.junie/skills/agent-paint-pixel-skills/guidelines.md` 或 `$HOME/.junie/skills/agent-paint-pixel-skills/guidelines.md` |

规则适配目标通常只写一个短规则文件，同时保留一个通用 Skill 目录作为完整参考来源。

## 项目级安装

当某个工具只读取当前仓库规则，或者你希望 Skill 跟随某个项目时，用项目级安装：

```bash
sh ./scripts/install.sh --skip-cli --project-skills --skill-target cursor
sh ./scripts/install.sh --skip-cli --project-skills --skill-target windsurf
sh ./scripts/install.sh --skip-cli --project-skills --skill-target universal
```

Windows PowerShell：

```powershell
.\scripts\install.ps1 -SkipCli -ProjectSkills -SkillTargets cursor
.\scripts\install.ps1 -SkipCli -ProjectSkills -SkillTargets windsurf
.\scripts\install.ps1 -SkipCli -ProjectSkills -SkillTargets universal
```

安装到另一个仓库：

```bash
sh ./scripts/install.sh --skip-cli --project-skills --project-path /path/to/repo --skill-target universal
```

Windows PowerShell：

```powershell
.\scripts\install.ps1 -SkipCli -ProjectSkills -ProjectPath C:\path\to\repo -SkillTargets universal
```

## 给 CLI AI 的安装提示词

可以把下面的话交给 CLI AI 执行：

```text
在当前 AgentPaint 仓库里安装本项目：
1. 运行当前平台的安装脚本。
2. 把 agentpaint CLI 安装到 PATH 可访问的位置。
3. 给我的 CLI/IDE Agent 安装 agent-paint-pixel-skills。
4. 运行 `agentpaint --help` 验证 CLI。
5. 验证对应 Skill 目录或规则文件存在。
6. 不要删除无关文件。
```

如果只安装 Skill：

```text
只把 AgentPaint 的 agent-paint-pixel-skills 安装到所有支持的 CLI/IDE Agent 目标，不重新安装 Rust CLI。
```

## 更新

拉取新代码后：

```bash
sh ./scripts/install.sh --update-path --all-skill-targets
```

Windows PowerShell：

```powershell
.\scripts\install.ps1 -UpdatePath -AllSkillTargets
```

## 卸载

卸载 CLI：

```bash
cargo uninstall agentpaint
```

然后删除你安装过的 Skill 目录或规则文件。

## 常见问题

`agentpaint --help` 找不到命令：

- 确认 `cargo install` 成功。
- 确认 Cargo bin 目录在 `PATH` 中。常见路径是 `$HOME/.cargo/bin` 或 Windows 的 `%USERPROFILE%\.cargo\bin`。

Agent 找不到 Skill：

- 确认安装目标是否对应你的工具。
- 对于 Cursor、Windsurf、Trae、Junie 这类规则适配目标，确认是装到用户级还是项目级路径。
- 重启对应 CLI/IDE，让它重新读取 Skill 或规则文件。

Agent 运行时去找 AgentPaint 源码仓库：

- 这是不应该发生的。正常生成像素画只需要安装好的 Skill 和 `PATH` 中的 `agentpaint`。
- 如果规则适配目标只装了短规则文件，请确认通用 Skill 目录也存在，或者重新运行安装脚本且不要禁用 companion Skill。
