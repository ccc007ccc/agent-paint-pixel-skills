# AgentPaint 中文指南

AgentPaint 是一个面向 CLI AI / 编程 Agent 的像素画项目。它包含三部分：

- `agentpaint`：Rust 写的命令行工具，负责校验、渲染、预览、打补丁、导出 PSD/RGBA、渲染 GIF。
- `APX/APXA`：适合 LLM 手写的像素画源格式。APX 是静态图，APXA 是动画。
- `agent-paint-pixel-skills`：可安装到多个 CLI/IDE Agent 的 `SKILL.md` 技能，让 AI 按固定流程生成、检查和导出像素画。

项目目标不是让 AI 输出原始 RGBA 数组，而是让 AI 写“调色板 + 单字符像素网格 + 图层/局部块”。这样更容易检查、修改和复用。

## 为什么需要 APX

直接让 LLM 输出 RGBA 数组会有几个问题：

- 数据量太大，容易漏像素、错行、错列。
- 人类不容易审查结果。
- 大分辨率、动画、多图层时很难局部修改。
- LLM 容易用大片同色块糊完画面。

APX 把问题拆成更适合 LLM 的形式：

```txt
palette:
. = transparent
K = #171717
R = #d93636

rows:
.......
..RRR..
.RKKR..
.......
```

一个字符就是一个像素。程序负责校验每行宽度、颜色引用、图层顺序和导出。

## 核心能力

- 精确尺寸：用户要 `32x32`，APX 画布就是 `32x32`，渲染图也是 `32x32`。
- 分层绘制：`layers[0]` 是视觉最上层，最后一个图层是最底层，和 Photoshop 的图层列表逻辑一致。
- 局部块：大图可以用 `chunks` 分区绘制，避免每次重写整张画。
- 局部修改：用 patch 文件执行 `add_chunk`、`set_rows`、`move_layer`、`set_layer_opacity` 等操作。
- 动画：APXA 用基础图层加逐帧 patch 操作，适合跳跃、眨眼、闪烁、飘动等循环动画。
- 导出：支持 PNG、GIF、PSD、RGBA JSON。
- 预览：低分辨率像素画应先点对点放大再让 AI 视觉检查，避免视觉模型把像素图模糊处理。

## 展示图和真实尺寸

README 顶部的史莱姆动图来自：

```text
examples/red-slime-jump-16.apxa
```

真实动画尺寸是 `16x16`。README 使用的文件是：

```text
examples/red-slime-jump-16-highres.gif
```

它是从透明背景的 `16x16` 原始 GIF 最近邻点对点放大到 `512x512` 的展示图，只用于网页展示，不代表源作品被放大或改尺寸。

## 快速安装

Windows PowerShell：

```powershell
.\scripts\install.ps1 -UpdatePath
```

macOS/Linux：

```bash
sh ./scripts/install.sh --update-path
```

安装后验证：

```bash
agentpaint --help
```

详细安装说明见 [中文安装指南](install.zh-CN.md)。

## 给 CLI AI 安装 Skill

默认安装会把 Skill 放到通用路径：

```text
$HOME/.agents/skills/agent-paint-pixel-skills
```

给某个 CLI/IDE Agent 单独安装：

```bash
sh ./scripts/install.sh --skip-cli --skill-target claude-code
sh ./scripts/install.sh --skip-cli --skill-target cursor --project-skills
```

Windows PowerShell：

```powershell
.\scripts\install.ps1 -SkipCli -SkillTargets claude-code
.\scripts\install.ps1 -SkipCli -ProjectSkills -SkillTargets cursor
```

支持的目标包括：

```text
universal/codex, claude-code, copilot, gemini, kiro, cline,
roo-code, kilo-code, factory, goose, opencode, antigravity,
cursor, windsurf, trae, junie
```

注意：Skill 只提供工作流和参考文件，不包含 Rust 二进制。运行时仍需要 `agentpaint` 在 `PATH` 里。

## APX 最小示例

```json
{
  "canvas": { "width": 4, "height": 4 },
  "background": "#dfe8c8",
  "palette": {
    ".": "transparent",
    "K": "#171717",
    "R": "#d93636"
  },
  "layers": [
    {
      "name": "paint",
      "rows": [
        "_",
        "_RR_",
        "_KK_",
        "_"
      ]
    }
  ]
}
```

规则：

- `.` 固定代表透明。
- `_` 是保留字符，不要写进 `palette`。
- 没有顶层 `background` 时，`_` 渲染为透明。
- 有顶层 `background` 时，`_` 渲染为背景色。
- 完整画布 `rows` 中一行只有 `"_"` 时，表示这一整行都是背景/透明。

## 常用命令

静态图：

```bash
agentpaint validate art.apx
agentpaint inspect art.apx
agentpaint import-image art.png --out art.apx
agentpaint render art.apx --out art.png
agentpaint supersample art.apx --out art-preview.png
agentpaint export-psd art.apx --out art.psd
```

动画：

```bash
agentpaint validate-animation jump.apxa
agentpaint import-gif jump.gif --out jump.apxa
agentpaint inspect-animation jump.apxa
agentpaint render-frame jump.apxa --frame 0 --out frame0.png
agentpaint supersample-frame jump.apxa --frame 0 --out frame0-preview.png
agentpaint render-gif jump.apxa --out jump.gif
```

`import-image` 和 `import-gif` 用于把已有位图素材转换成 APX/APXA。转换会保留源尺寸，不做缩放；GIF 导入会保留帧时长。

注意：GIF 导出无法保留柔和半透明。完全透明像素会保持透明，但半透明阴影等 alpha 会在 GIF 编码时被量化。

局部修改：

```bash
agentpaint patch art.apx --patch patch.json --out art-patched.apx
```

## 文档索引

- [安装指南](install.zh-CN.md)
- [LLM 像素画源格式思路](llm-pixel-art-format.md)
- [CLI + Skill 规格](agentpaint-cli-skill-spec.md)
- [示例说明](examples.md)
- [风格研究](pixel-art-style-research.md)
- [Skill 兼容性说明](skill-compatibility.md)
